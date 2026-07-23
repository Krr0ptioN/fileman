use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskKind {
    Copy,
    Move,
    Delete,
    Rename,
    CreateDirectory,
}

impl TaskKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Move => "move",
            Self::Delete => "delete",
            Self::Rename => "rename",
            Self::CreateDirectory => "mkdir",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

pub struct TaskRecord {
    pub id: TaskId,
    pub kind: TaskKind,
    pub state: TaskState,
    pub items_done: u64,
    pub items_total: u64,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub detail: Option<String>,
    runtime: Arc<TaskRuntime>,
}

pub struct TaskRuntime {
    cancelled: AtomicBool,
    items_done: AtomicU64,
    bytes_done: AtomicU64,
}

impl TaskRuntime {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub fn add_item(&self) {
        self.items_done.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_bytes(&self, bytes: u64) {
        self.bytes_done.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> (u64, u64) {
        (
            self.items_done.load(Ordering::Relaxed),
            self.bytes_done.load(Ordering::Relaxed),
        )
    }
}

pub struct TaskQueue {
    next_id: u64,
    history_limit: usize,
    tasks: VecDeque<TaskRecord>,
}

impl TaskQueue {
    pub fn new(history_limit: usize) -> Self {
        Self {
            next_id: 1,
            history_limit,
            tasks: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, kind: TaskKind, items_total: u64, bytes_total: u64) -> TaskId {
        let id = TaskId(self.next_id);
        self.next_id = self.next_id.wrapping_add(1).max(1);
        self.tasks.push_back(TaskRecord {
            id,
            kind,
            state: TaskState::Queued,
            items_done: 0,
            items_total,
            bytes_done: 0,
            bytes_total,
            detail: None,
            runtime: Arc::new(TaskRuntime {
                cancelled: AtomicBool::new(false),
                items_done: AtomicU64::new(0),
                bytes_done: AtomicU64::new(0),
            }),
        });
        self.trim_history();
        id
    }

    pub fn task(&self, id: TaskId) -> Option<&TaskRecord> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn runtime(&self, id: TaskId) -> Option<Arc<TaskRuntime>> {
        self.task(id).map(|task| Arc::clone(&task.runtime))
    }

    pub fn start(&mut self, id: TaskId) -> bool {
        self.transition(id, TaskState::Queued, TaskState::Running, None)
    }

    pub fn update(&mut self, id: TaskId, items_done: u64, bytes_done: u64) -> bool {
        let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) else {
            return false;
        };
        if task.state != TaskState::Running {
            return false;
        }
        task.items_done = items_done.min(task.items_total);
        task.bytes_done = match task.bytes_total {
            0 => bytes_done,
            total => bytes_done.min(total),
        };
        task.runtime.items_done.store(items_done, Ordering::Relaxed);
        task.runtime.bytes_done.store(bytes_done, Ordering::Relaxed);
        true
    }

    pub fn complete(&mut self, id: TaskId) -> bool {
        let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) else {
            return false;
        };
        if task.state != TaskState::Running {
            return false;
        }
        let (items_done, bytes_done) = task.runtime.snapshot();
        task.state = TaskState::Completed;
        task.items_done = items_done.max(task.items_done).min(task.items_total);
        task.bytes_done = bytes_done.max(task.bytes_done);
        true
    }

    pub fn fail(&mut self, id: TaskId, detail: String) -> bool {
        self.sync(id);
        self.transition(id, TaskState::Running, TaskState::Failed, Some(detail))
    }

    pub fn cancel(&mut self, id: TaskId) -> bool {
        let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) else {
            return false;
        };
        match task.state {
            TaskState::Queued | TaskState::Running => {
                task.runtime.cancelled.store(true, Ordering::Relaxed);
                task.state = TaskState::Cancelled;
            }
            TaskState::Cancelled => {}
            TaskState::Completed | TaskState::Failed => return false,
        }
        let (items_done, bytes_done) = task.runtime.snapshot();
        task.items_done = items_done.min(task.items_total);
        task.bytes_done = bytes_done;
        true
    }

    pub fn active_id(&self) -> Option<TaskId> {
        self.tasks
            .iter()
            .find(|task| task.state == TaskState::Running)
            .or_else(|| {
                self.tasks
                    .iter()
                    .find(|task| task.state == TaskState::Queued)
            })
            .map(|task| task.id)
    }

    pub fn status_line(&self) -> String {
        if let Some(task) = self
            .tasks
            .iter()
            .find(|task| task.state == TaskState::Running)
        {
            let (runtime_items, runtime_bytes) = task.runtime.snapshot();
            let items = format!(
                "{}/{} items",
                runtime_items.max(task.items_done).min(task.items_total),
                task.items_total
            );
            return match task.bytes_total {
                0 => format!("{} {items}", task.kind.label()),
                total => format!(
                    "{} {items} · {}/{} B",
                    task.kind.label(),
                    runtime_bytes.max(task.bytes_done).min(total),
                    total
                ),
            };
        }
        if let Some(task) = self
            .tasks
            .iter()
            .find(|task| task.state == TaskState::Queued)
        {
            return format!("{} queued", task.kind.label());
        }
        for (state, label) in [
            (TaskState::Failed, "failed"),
            (TaskState::Cancelled, "cancelled"),
            (TaskState::Completed, "completed"),
        ] {
            let count = self.tasks.iter().filter(|task| task.state == state).count();
            if count > 0 {
                return format!("{count} {label}");
            }
        }
        String::new()
    }

    fn transition(
        &mut self,
        id: TaskId,
        from: TaskState,
        to: TaskState,
        detail: Option<String>,
    ) -> bool {
        let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) else {
            return false;
        };
        if task.state != from {
            return false;
        }
        task.state = to;
        task.detail = detail;
        true
    }

    fn sync(&mut self, id: TaskId) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            let (items_done, bytes_done) = task.runtime.snapshot();
            task.items_done = items_done.min(task.items_total);
            task.bytes_done = bytes_done;
        }
    }

    fn trim_history(&mut self) {
        while self.tasks.len() > self.history_limit {
            let removable = self.tasks.front().is_some_and(|task| {
                matches!(
                    task.state,
                    TaskState::Completed | TaskState::Failed | TaskState::Cancelled
                )
            });
            if removable {
                self.tasks.pop_front();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TaskKind, TaskQueue, TaskState};

    #[test]
    fn queue_reports_running_progress_and_completed_summary() {
        let mut queue = TaskQueue::new(4);
        let task = queue.enqueue(TaskKind::Copy, 2, 100);

        assert!(queue.start(task));
        assert!(queue.update(task, 1, 40));
        assert_eq!(queue.status_line(), "copy 1/2 items · 40/100 B");

        assert!(queue.complete(task));
        assert_eq!(
            queue.task(task).map(|task| task.state),
            Some(TaskState::Completed)
        );
        assert_eq!(queue.status_line(), "1 completed");
    }

    #[test]
    fn cancellation_preserves_partial_progress() {
        let mut queue = TaskQueue::new(4);
        let task = queue.enqueue(TaskKind::Move, 3, 0);
        queue.start(task);
        queue.update(task, 1, 0);

        assert!(queue.cancel(task));
        assert_eq!(
            queue.task(task).map(|task| task.state),
            Some(TaskState::Cancelled)
        );
        assert_eq!(queue.task(task).map(|task| task.items_done), Some(1));
        assert_eq!(queue.status_line(), "1 cancelled");
    }
}
