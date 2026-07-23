use std::io;

pub struct ImageMeta {
    pub width: usize,
    pub height: usize,
    pub depth: zune_core::bit_depth::BitDepth,
}

pub struct GifFrame {
    pub image: egui::ColorImage,
    pub delay_ms: u32,
}

pub enum DecodedImage {
    Static(egui::ColorImage),
    Animated(Vec<GifFrame>),
}

pub fn is_gif(bytes: &[u8]) -> bool {
    bytes.len() >= 6 && (bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a"))
}

pub fn is_jpeg(bytes: &[u8]) -> bool {
    bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xD8
}

/// Extract the EXIF thumbnail — near-instant, ~160×120.
pub fn decode_jpeg_exif_thumbnail(
    bytes: &[u8],
    max_side: u32,
) -> Option<(DecodedImage, ImageMeta)> {
    extract_exif_thumbnail(bytes, max_side)
}

/// DC-only 1/8-scale decode — fast but must parse the entropy stream.
pub fn decode_jpeg_dc_preview(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    decode_jpeg_dc_only(bytes, max_side)
}

pub fn decode_gif_first_frame(
    bytes: &[u8],
    max_side: u32,
) -> Option<(egui::ColorImage, ImageMeta)> {
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let cursor = io::Cursor::new(bytes);
    let mut decoder = options.read_info(cursor).ok()?;
    let screen_w = decoder.width() as usize;
    let screen_h = decoder.height() as usize;
    if screen_w == 0 || screen_h == 0 {
        return None;
    }
    let frame = decoder.read_next_frame().ok()??;
    let mut canvas = vec![0u8; screen_w * screen_h * 4];
    let fw = frame.width as usize;
    let fh = frame.height as usize;
    let fl = frame.left as usize;
    let ft = frame.top as usize;
    for y in 0..fh {
        for x in 0..fw {
            let cx = fl + x;
            let cy = ft + y;
            if cx >= screen_w || cy >= screen_h {
                continue;
            }
            let src = (y * fw + x) * 4;
            if src + 3 >= frame.buffer.len() {
                continue;
            }
            if frame.buffer[src + 3] > 0 {
                let dst = (cy * screen_w + cx) * 4;
                canvas[dst..dst + 4].copy_from_slice(&frame.buffer[src..src + 4]);
            }
        }
    }
    let (out_w, out_h, out_rgba) = downscale_rgba(&canvas, screen_w, screen_h, max_side);
    let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
    Some((
        color,
        ImageMeta {
            width: screen_w,
            height: screen_h,
            depth: zune_core::bit_depth::BitDepth::Eight,
        },
    ))
}

pub fn decode_image_bytes(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    // Try GIF first (before zune, which only decodes the first frame)
    if is_gif(bytes) {
        return decode_gif_bytes(bytes, max_side);
    }

    let options = zune_core::options::DecoderOptions::new_fast();
    if let Ok(image) = zune_image::image::Image::read(std::io::Cursor::new(bytes), options) {
        let orientation = exif_orientation(&image).unwrap_or(1);
        let (width, height) = image.dimensions();
        let depth = image.depth();
        let colorspace = image.colorspace();
        let mut frames = image.flatten_to_u8();
        let data = frames.pop()?;
        let rgba = convert_to_rgba(&data, width, height, colorspace)?;
        let (rgba, width, height) = apply_orientation_rgba(rgba, width, height, orientation);
        let (out_w, out_h, out_rgba) = downscale_rgba(&rgba, width, height, max_side);
        let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
        return Some((
            DecodedImage::Static(color),
            ImageMeta {
                width,
                height,
                depth,
            },
        ));
    }

    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return decode_webp_bytes(bytes, max_side);
    }

    if bytes.len() >= 2 && bytes[0] == b'B' && bytes[1] == b'M' {
        return decode_bmp_bytes(bytes, max_side);
    }

    if bytes.starts_with(b"#?RADIANCE") || bytes.starts_with(b"#?RGBE") {
        return decode_hdr_bytes(bytes, max_side);
    }

    if bytes.len() >= 4 && &bytes[..4] == b"DDS " {
        return decode_dds_bytes(bytes, max_side);
    }

    // TGA has no magic bytes; try it as a last resort
    if bytes.len() >= 18
        && let Some(result) = decode_tga_bytes(bytes, max_side)
    {
        return Some(result);
    }

    None
}

fn decode_gif_bytes(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let cursor = io::Cursor::new(bytes);
    let mut decoder = options.read_info(cursor).ok()?;
    let screen_w = decoder.width() as usize;
    let screen_h = decoder.height() as usize;
    if screen_w == 0 || screen_h == 0 {
        return None;
    }

    let mut canvas = vec![0u8; screen_w * screen_h * 4];
    let mut frames = Vec::new();

    while let Ok(Some(frame)) = decoder.read_next_frame() {
        let saved = canvas.clone();

        // Composite frame onto canvas
        let fw = frame.width as usize;
        let fh = frame.height as usize;
        let fl = frame.left as usize;
        let ft = frame.top as usize;
        for y in 0..fh {
            for x in 0..fw {
                let cx = fl + x;
                let cy = ft + y;
                if cx >= screen_w || cy >= screen_h {
                    continue;
                }
                let src = (y * fw + x) * 4;
                if src + 3 >= frame.buffer.len() {
                    continue;
                }
                let alpha = frame.buffer[src + 3];
                if alpha > 0 {
                    let dst = (cy * screen_w + cx) * 4;
                    canvas[dst..dst + 4].copy_from_slice(&frame.buffer[src..src + 4]);
                }
            }
        }

        let (out_w, out_h, out_rgba) = downscale_rgba(&canvas, screen_w, screen_h, max_side);
        let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
        // GIF delays are in centiseconds; 0 means use default (~100ms)
        let delay_cs = u32::from(frame.delay);
        let delay_ms = if delay_cs == 0 { 100 } else { delay_cs * 10 };
        frames.push(GifFrame {
            image: color,
            delay_ms,
        });

        // Apply disposal method
        match frame.dispose {
            gif::DisposalMethod::Keep | gif::DisposalMethod::Any => {}
            gif::DisposalMethod::Background => {
                for y in 0..fh {
                    for x in 0..fw {
                        let cx = fl + x;
                        let cy = ft + y;
                        if cx < screen_w && cy < screen_h {
                            let dst = (cy * screen_w + cx) * 4;
                            canvas[dst..dst + 4].copy_from_slice(&[0, 0, 0, 0]);
                        }
                    }
                }
            }
            gif::DisposalMethod::Previous => {
                canvas = saved;
            }
        }

        if frames.len() >= 200 {
            break;
        }
    }

    if frames.is_empty() {
        return None;
    }

    let meta = ImageMeta {
        width: screen_w,
        height: screen_h,
        depth: zune_core::bit_depth::BitDepth::Eight,
    };

    if frames.len() == 1 {
        let frame = frames.into_iter().next().unwrap();
        Some((DecodedImage::Static(frame.image), meta))
    } else {
        Some((DecodedImage::Animated(frames), meta))
    }
}

fn decode_webp_bytes(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    let cursor = io::Cursor::new(bytes);
    let mut decoder = image_webp::WebPDecoder::new(cursor).ok()?;
    let size = decoder.output_buffer_size()?;
    let mut data = vec![0u8; size];
    decoder.read_image(&mut data).ok()?;
    let (width, height) = decoder.dimensions();
    let width = width as usize;
    let height = height as usize;
    let has_alpha = decoder.has_alpha();
    let rgba = if has_alpha {
        data
    } else {
        let mut out = Vec::with_capacity(width * height * 4);
        for rgb in data.chunks_exact(3) {
            out.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
        }
        out
    };
    let (out_w, out_h, out_rgba) = downscale_rgba(&rgba, width, height, max_side);
    let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
    Some((
        DecodedImage::Static(color),
        ImageMeta {
            width,
            height,
            depth: zune_core::bit_depth::BitDepth::Eight,
        },
    ))
}

fn decode_dds_bytes(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    let cursor = io::Cursor::new(bytes);
    let mut decoder = dds::Decoder::new(cursor).ok()?;
    let size = decoder.main_size();
    let width = size.width as usize;
    let height = size.height as usize;
    if width == 0 || height == 0 {
        return None;
    }
    let pixel_count = width.checked_mul(height)?;
    let mut rgba = vec![0u8; pixel_count * 4];
    let view = dds::ImageViewMut::new(&mut rgba, size, dds::ColorFormat::RGBA_U8)?;
    decoder.read_surface(view).ok()?;
    let (out_w, out_h, out_rgba) = downscale_rgba(&rgba, width, height, max_side);
    let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
    Some((
        DecodedImage::Static(color),
        ImageMeta {
            width,
            height,
            depth: zune_core::bit_depth::BitDepth::Eight,
        },
    ))
}

fn decode_hdr_bytes(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    // Skip header lines until empty line
    let mut pos = 0;
    loop {
        let line_end = memchr::memchr(b'\n', &bytes[pos..])? + pos;
        let line = &bytes[pos..line_end];
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        pos = line_end + 1;
        if line.is_empty() {
            break;
        }
    }

    // Parse resolution line: "-Y height +X width"
    let res_end = memchr::memchr(b'\n', &bytes[pos..])? + pos;
    let res_line = std::str::from_utf8(&bytes[pos..res_end]).ok()?;
    let res_line = res_line.trim();
    pos = res_end + 1;

    let mut parts = res_line.split_whitespace();
    let (width, height) = match (parts.next()?, parts.next()?, parts.next()?, parts.next()?) {
        ("-Y", h, "+X", w) => (w.parse::<usize>().ok()?, h.parse::<usize>().ok()?),
        ("+X", w, "-Y", h) => (w.parse::<usize>().ok()?, h.parse::<usize>().ok()?),
        _ => return None,
    };

    if width == 0 || height == 0 {
        return None;
    }

    let pixels = width.checked_mul(height)?;
    let mut rgbe_data = vec![[0u8; 4]; pixels];

    // Decode scanlines
    for y in 0..height {
        let scanline = &mut rgbe_data[y * width..(y + 1) * width];
        pos = hdr_read_scanline(bytes, pos, scanline, width)?;
    }

    // Convert RGBE to RGBA (tone-mapped to LDR)
    let mut rgba = vec![0u8; pixels * 4];
    for (i, rgbe) in rgbe_data.iter().enumerate() {
        let (r, g, b) = rgbe_to_rgb(rgbe);
        // Simple Reinhard tone mapping + gamma
        let tone_map = |v: f32| -> u8 {
            let mapped = v / (1.0 + v);
            let gamma = mapped.powf(1.0 / 2.2);
            (gamma * 255.0).clamp(0.0, 255.0) as u8
        };
        rgba[i * 4] = tone_map(r);
        rgba[i * 4 + 1] = tone_map(g);
        rgba[i * 4 + 2] = tone_map(b);
        rgba[i * 4 + 3] = 255;
    }

    let (out_w, out_h, out_rgba) = downscale_rgba(&rgba, width, height, max_side);
    let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
    Some((
        DecodedImage::Static(color),
        ImageMeta {
            width,
            height,
            depth: zune_core::bit_depth::BitDepth::Eight,
        },
    ))
}

fn rgbe_to_rgb(rgbe: &[u8; 4]) -> (f32, f32, f32) {
    if rgbe[3] == 0 {
        return (0.0, 0.0, 0.0);
    }
    let exp = 2.0_f32.powi(rgbe[3] as i32 - 128 - 8);
    (
        rgbe[0] as f32 * exp,
        rgbe[1] as f32 * exp,
        rgbe[2] as f32 * exp,
    )
}

fn hdr_read_scanline(
    bytes: &[u8],
    mut pos: usize,
    scanline: &mut [[u8; 4]],
    width: usize,
) -> Option<usize> {
    if pos + 4 > bytes.len() {
        return None;
    }

    // Check for new-style RLE: starts with 2, 2, width_hi, width_lo
    if bytes[pos] == 2 && bytes[pos + 1] == 2 && (8..=0x7FFF).contains(&width) {
        let line_w = ((bytes[pos + 2] as usize) << 8) | bytes[pos + 3] as usize;
        if line_w != width {
            return None;
        }
        pos += 4;

        // Each channel is RLE-encoded separately
        #[allow(clippy::needless_range_loop)]
        for ch in 0..4 {
            let mut x = 0;
            while x < width {
                if pos >= bytes.len() {
                    return None;
                }
                let code = bytes[pos];
                pos += 1;
                if code > 128 {
                    // Run
                    let count = (code - 128) as usize;
                    if pos >= bytes.len() || x + count > width {
                        return None;
                    }
                    let val = bytes[pos];
                    pos += 1;
                    for i in 0..count {
                        scanline[x + i][ch] = val;
                    }
                    x += count;
                } else {
                    // Literal
                    let count = code as usize;
                    if count == 0 || pos + count > bytes.len() || x + count > width {
                        return None;
                    }
                    for i in 0..count {
                        scanline[x + i][ch] = bytes[pos + i];
                    }
                    pos += count;
                    x += count;
                }
            }
        }
    } else {
        // Old-style: raw RGBE pixels (possibly with old RLE)
        for px in scanline.iter_mut() {
            if pos + 4 > bytes.len() {
                return None;
            }
            px.copy_from_slice(&bytes[pos..pos + 4]);
            pos += 4;
        }
    }

    Some(pos)
}

fn decode_tga_bytes(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    if bytes.len() < 18 {
        return None;
    }
    let id_length = bytes[0] as usize;
    let image_type = bytes[2];
    let width = u16::from_le_bytes([bytes[12], bytes[13]]) as usize;
    let height = u16::from_le_bytes([bytes[14], bytes[15]]) as usize;
    let pixel_depth = bytes[16];
    let descriptor = bytes[17];
    let top_origin = descriptor & 0x20 != 0;

    if width == 0 || height == 0 {
        return None;
    }

    let colormap_length = u16::from_le_bytes([bytes[5], bytes[6]]) as usize;
    let colormap_entry_size = bytes[7] as usize;
    let colormap_bytes = colormap_length * colormap_entry_size.div_ceil(8);
    let pixel_data_offset = 18 + id_length + colormap_bytes;
    if pixel_data_offset > bytes.len() {
        return None;
    }
    let pixel_data = &bytes[pixel_data_offset..];

    let pixels = width.checked_mul(height)?;
    let mut rgba = vec![0u8; pixels * 4];

    match image_type {
        // Uncompressed true-color
        2 => {
            tga_decode_uncompressed(pixel_data, &mut rgba, width, height, pixel_depth)?;
        }
        // RLE true-color
        10 => {
            tga_decode_rle(pixel_data, &mut rgba, width, height, pixel_depth)?;
        }
        // Uncompressed grayscale
        3 => {
            tga_decode_uncompressed_gray(pixel_data, &mut rgba, width, height, pixel_depth)?;
        }
        // RLE grayscale
        11 => {
            tga_decode_rle_gray(pixel_data, &mut rgba, width, height, pixel_depth)?;
        }
        _ => return None,
    }

    // Flip vertically if origin is bottom-left (default for TGA)
    if !top_origin {
        let row_bytes = width * 4;
        let mut tmp = vec![0u8; row_bytes];
        for y in 0..height / 2 {
            let top = y * row_bytes;
            let bot = (height - 1 - y) * row_bytes;
            tmp.copy_from_slice(&rgba[top..top + row_bytes]);
            rgba.copy_within(bot..bot + row_bytes, top);
            rgba[bot..bot + row_bytes].copy_from_slice(&tmp);
        }
    }

    let (out_w, out_h, out_rgba) = downscale_rgba(&rgba, width, height, max_side);
    let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
    Some((
        DecodedImage::Static(color),
        ImageMeta {
            width,
            height,
            depth: zune_core::bit_depth::BitDepth::Eight,
        },
    ))
}

fn tga_read_pixel(src: &[u8], pixel_depth: u8) -> Option<[u8; 4]> {
    match pixel_depth {
        32 => {
            if src.len() < 4 {
                return None;
            }
            Some([src[2], src[1], src[0], src[3]]) // BGRA -> RGBA
        }
        24 => {
            if src.len() < 3 {
                return None;
            }
            Some([src[2], src[1], src[0], 255]) // BGR -> RGBA
        }
        16 => {
            if src.len() < 2 {
                return None;
            }
            let v = u16::from_le_bytes([src[0], src[1]]);
            let r = ((v & 0x7C00) >> 7) as u8 | ((v & 0x7C00) >> 12) as u8;
            let g = ((v & 0x03E0) >> 2) as u8 | ((v & 0x03E0) >> 7) as u8;
            let b = ((v & 0x001F) << 3) as u8 | ((v & 0x001F) >> 2) as u8;
            Some([r, g, b, 255])
        }
        _ => None,
    }
}

fn tga_decode_uncompressed(
    data: &[u8],
    rgba: &mut [u8],
    width: usize,
    height: usize,
    depth: u8,
) -> Option<()> {
    let bpp = (depth as usize).div_ceil(8);
    let needed = width * height * bpp;
    if data.len() < needed {
        return None;
    }
    for i in 0..width * height {
        let px = tga_read_pixel(&data[i * bpp..], depth)?;
        rgba[i * 4..i * 4 + 4].copy_from_slice(&px);
    }
    Some(())
}

fn tga_decode_rle(
    data: &[u8],
    rgba: &mut [u8],
    width: usize,
    height: usize,
    depth: u8,
) -> Option<()> {
    let bpp = (depth as usize).div_ceil(8);
    let total = width * height;
    let mut src = 0;
    let mut dst = 0;
    while dst < total {
        if src >= data.len() {
            return None;
        }
        let header = data[src];
        src += 1;
        let count = (header & 0x7F) as usize + 1;
        if header & 0x80 != 0 {
            // RLE packet
            let px = tga_read_pixel(data.get(src..)?, depth)?;
            src += bpp;
            for _ in 0..count.min(total - dst) {
                rgba[dst * 4..dst * 4 + 4].copy_from_slice(&px);
                dst += 1;
            }
        } else {
            // Raw packet
            for _ in 0..count.min(total - dst) {
                let px = tga_read_pixel(data.get(src..)?, depth)?;
                src += bpp;
                rgba[dst * 4..dst * 4 + 4].copy_from_slice(&px);
                dst += 1;
            }
        }
    }
    Some(())
}

fn tga_decode_uncompressed_gray(
    data: &[u8],
    rgba: &mut [u8],
    width: usize,
    height: usize,
    depth: u8,
) -> Option<()> {
    let total = width * height;
    match depth {
        8 => {
            if data.len() < total {
                return None;
            }
            for i in 0..total {
                let v = data[i];
                rgba[i * 4..i * 4 + 4].copy_from_slice(&[v, v, v, 255]);
            }
        }
        16 => {
            if data.len() < total * 2 {
                return None;
            }
            for i in 0..total {
                let v = data[i * 2];
                let a = data[i * 2 + 1];
                rgba[i * 4..i * 4 + 4].copy_from_slice(&[v, v, v, a]);
            }
        }
        _ => return None,
    }
    Some(())
}

fn tga_decode_rle_gray(
    data: &[u8],
    rgba: &mut [u8],
    width: usize,
    height: usize,
    depth: u8,
) -> Option<()> {
    let bpp = (depth as usize).div_ceil(8);
    let total = width * height;
    let mut src = 0;
    let mut dst = 0;
    while dst < total {
        if src >= data.len() {
            return None;
        }
        let header = data[src];
        src += 1;
        let count = (header & 0x7F) as usize + 1;
        if header & 0x80 != 0 {
            // RLE packet
            if src + bpp > data.len() {
                return None;
            }
            let (v, a) = if bpp == 2 {
                (data[src], data[src + 1])
            } else {
                (data[src], 255)
            };
            src += bpp;
            for _ in 0..count.min(total - dst) {
                rgba[dst * 4..dst * 4 + 4].copy_from_slice(&[v, v, v, a]);
                dst += 1;
            }
        } else {
            // Raw packet
            for _ in 0..count.min(total - dst) {
                if src + bpp > data.len() {
                    return None;
                }
                let (v, a) = if bpp == 2 {
                    (data[src], data[src + 1])
                } else {
                    (data[src], 255)
                };
                src += bpp;
                rgba[dst * 4..dst * 4 + 4].copy_from_slice(&[v, v, v, a]);
                dst += 1;
            }
        }
    }
    Some(())
}

fn decode_bmp_bytes(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    let mut decoder = zune_bmp::BmpDecoder::new(std::io::Cursor::new(bytes));
    decoder.decode_headers().ok()?;
    let (width, height) = decoder.dimensions()?;
    let depth = decoder.depth();
    let colorspace = decoder.colorspace()?;
    let data = decoder.decode().ok()?;
    let rgba = convert_to_rgba(&data, width, height, colorspace)?;
    let (out_w, out_h, out_rgba) = downscale_rgba(&rgba, width, height, max_side);
    let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
    Some((
        DecodedImage::Static(color),
        ImageMeta {
            width,
            height,
            depth,
        },
    ))
}

// ---------------------------------------------------------------------------
// JPEG preview fast-path: EXIF thumbnail extraction + DC-only 1/8 decode
// ---------------------------------------------------------------------------

/// Extract and decode the EXIF thumbnail JPEG if present and large enough.
fn extract_exif_thumbnail(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    let cursor = io::Cursor::new(bytes);
    let mut buf_reader = io::BufReader::new(cursor);
    let exif = exif::Reader::new()
        .read_from_container(&mut buf_reader)
        .ok()?;
    let orientation = exif
        .get_field(exif::Tag::Orientation, exif::In::PRIMARY)
        .and_then(|f| match f.value {
            exif::Value::Short(ref v) => v.first().copied(),
            _ => None,
        })
        .unwrap_or(1);
    // Read original image dimensions from EXIF so the preview meta is accurate
    let orig_width = exif
        .get_field(exif::Tag::PixelXDimension, exif::In::PRIMARY)
        .or_else(|| exif.get_field(exif::Tag::ImageWidth, exif::In::PRIMARY))
        .and_then(|f| f.value.get_uint(0))
        .map(|v| v as usize);
    let orig_height = exif
        .get_field(exif::Tag::PixelYDimension, exif::In::PRIMARY)
        .or_else(|| exif.get_field(exif::Tag::ImageLength, exif::In::PRIMARY))
        .and_then(|f| f.value.get_uint(0))
        .map(|v| v as usize);

    let offset = exif
        .get_field(exif::Tag::JPEGInterchangeFormat, exif::In::THUMBNAIL)?
        .value
        .get_uint(0)? as usize;
    let length = exif
        .get_field(exif::Tag::JPEGInterchangeFormatLength, exif::In::THUMBNAIL)?
        .value
        .get_uint(0)? as usize;
    // The offset is relative to the TIFF header inside the EXIF APP1 segment.
    // exif.buf() gives us the raw EXIF data starting from the TIFF header.
    let exif_buf = exif.buf();
    if offset + length > exif_buf.len() {
        return None;
    }
    let thumb_bytes = &exif_buf[offset..offset + length];
    // Sanity: must be a JPEG
    if thumb_bytes.len() < 3 || thumb_bytes[0] != 0xFF || thumb_bytes[1] != 0xD8 {
        return None;
    }
    // Decode the small thumbnail with zune
    let options = zune_core::options::DecoderOptions::new_fast();
    let image = zune_image::image::Image::read(std::io::Cursor::new(thumb_bytes), options).ok()?;
    let (width, height) = image.dimensions();
    // Skip if thumbnail is too small to be useful
    if width.max(height) < 160 {
        return None;
    }
    let depth = image.depth();
    let colorspace = image.colorspace();
    let mut frames = image.flatten_to_u8();
    let data = frames.pop()?;
    let rgba = convert_to_rgba(&data, width, height, colorspace)?;
    let (rgba, width, height) = apply_orientation_rgba(rgba, width, height, orientation);
    let (out_w, out_h, out_rgba) = downscale_rgba(&rgba, width, height, max_side);
    let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
    // Use original image dimensions if available, accounting for orientation swap
    let (meta_w, meta_h) = match (orig_width, orig_height) {
        (Some(w), Some(h)) => {
            if (5..=8).contains(&orientation) {
                (h, w)
            } else {
                (w, h)
            }
        }
        _ => (width, height),
    };
    Some((
        DecodedImage::Static(color),
        ImageMeta {
            width: meta_w,
            height: meta_h,
            depth,
        },
    ))
}

// -- DC-only baseline JPEG decoder ------------------------------------------

/// Huffman lookup table for fast decoding (up to 8-bit codes).
struct HuffLut {
    /// For codes up to 8 bits: (symbol, code_length). 0 length = not valid.
    fast: [u16; 256], // packed: high 8 = symbol, low 8 = length
    /// Slower path for codes > 8 bits.
    symbols: Vec<u8>,
    min_code: [i32; 17],
    max_code: [i32; 17],
    val_offset: [i32; 17],
}

impl HuffLut {
    fn build(counts: &[u8; 16], symbols: &[u8]) -> Self {
        let mut lut = HuffLut {
            fast: [0; 256],
            symbols: symbols.to_vec(),
            min_code: [0; 17],
            max_code: [-1; 17],
            val_offset: [0; 17],
        };
        let mut code: u32 = 0;
        let mut si = 0usize;
        for bits in 1..=16usize {
            lut.min_code[bits] = code as i32;
            let count = counts[bits - 1] as usize;
            for _ in 0..count {
                if bits <= 8 && si < symbols.len() {
                    let sym = symbols[si];
                    let pad = 8 - bits;
                    for padding in 0..(1u32 << pad) {
                        let idx = ((code << pad) | padding) as usize;
                        if idx < 256 {
                            lut.fast[idx] = ((sym as u16) << 8) | bits as u16;
                        }
                    }
                }
                code += 1;
                si += 1;
            }
            lut.max_code[bits] = code as i32 - 1;
            lut.val_offset[bits] = (si as i32) - (code as i32);
            code <<= 1;
        }
        lut
    }
}

struct JpegComponent {
    h_samp: usize,
    v_samp: usize,
    qt_id: usize,
    dc_table: usize,
    ac_table: usize,
}

struct JpegHeader {
    qt_tables: [[i32; 64]; 4],
    dc_huff: [Option<HuffLut>; 4],
    ac_huff: [Option<HuffLut>; 4],
    components: Vec<JpegComponent>,
    width: u16,
    height: u16,
    restart_interval: u16,
    orientation: u16,
    entropy_start: usize,
}

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bits: u32,
    nbits: u8,
    /// Set when `read_byte` encounters a marker (0xFF + non-zero).
    /// Prevents subsequent `fill` calls from reading past the marker.
    hit_marker: bool,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8], start: usize) -> Self {
        BitReader {
            data,
            pos: start,
            bits: 0,
            nbits: 0,
            hit_marker: false,
        }
    }

    /// Read a single byte from the JPEG entropy stream, handling FF 00 stuffing.
    fn read_byte(&mut self) -> Option<u8> {
        if self.hit_marker || self.pos >= self.data.len() {
            return None;
        }
        let b = self.data[self.pos];
        self.pos += 1;
        if b == 0xFF {
            // Must be followed by 0x00 (stuffed byte)
            if self.pos < self.data.len() && self.data[self.pos] == 0x00 {
                self.pos += 1;
            } else {
                // Marker encountered — stop reading; pos is at the marker type byte
                self.hit_marker = true;
                return None;
            }
        }
        Some(b)
    }

    /// Skip past a restart marker (FF Dn) and reset the bit buffer.
    fn skip_restart_marker(&mut self) {
        self.nbits = 0;
        self.bits = 0;
        if self.hit_marker {
            // pos is at the marker type byte; skip it
            if self.pos < self.data.len() && (self.data[self.pos] & 0xF8) == 0xD0 {
                self.pos += 1;
            }
            self.hit_marker = false;
        } else {
            // Scan forward for the restart marker
            while self.pos + 1 < self.data.len() {
                if self.data[self.pos] == 0xFF && (self.data[self.pos + 1] & 0xF8) == 0xD0 {
                    self.pos += 2;
                    break;
                }
                self.pos += 1;
            }
        }
    }

    fn fill(&mut self) {
        while self.nbits <= 24 {
            if let Some(b) = self.read_byte() {
                self.bits = (self.bits << 8) | b as u32;
                self.nbits += 8;
            } else {
                break;
            }
        }
    }

    fn peek(&mut self, n: u8) -> u32 {
        if self.nbits < n {
            self.fill();
        }
        if self.nbits < n {
            return 0;
        }
        (self.bits >> (self.nbits - n)) & ((1u32 << n) - 1)
    }

    fn consume(&mut self, n: u8) {
        self.nbits = self.nbits.saturating_sub(n);
    }

    fn get_bits(&mut self, n: u8) -> i32 {
        let v = self.peek(n);
        self.consume(n);
        v as i32
    }

    fn decode_huff(&mut self, lut: &HuffLut) -> Option<u8> {
        // Try fast 8-bit lookup
        let peek8 = self.peek(8) as usize;
        let entry = lut.fast[peek8];
        if entry != 0 {
            let len = (entry & 0xFF) as u8;
            let sym = (entry >> 8) as u8;
            self.consume(len);
            return Some(sym);
        }
        // Slow path for codes > 8 bits
        let mut code = self.peek(8) as i32;
        self.consume(8);
        for bits in 9..=16u8 {
            code = (code << 1) | self.get_bits(1);
            if code <= lut.max_code[bits as usize] {
                let idx = (code + lut.val_offset[bits as usize]) as usize;
                return lut.symbols.get(idx).copied();
            }
        }
        None
    }
}

fn jpeg_segment(bytes: &[u8], pos: usize) -> Option<(&[u8], usize)> {
    let length = usize::from(u16::from_be_bytes([
        *bytes.get(pos)?,
        *bytes.get(pos.checked_add(1)?)?,
    ]));
    if length < 2 {
        return None;
    }
    let payload_start = pos.checked_add(2)?;
    let end = pos.checked_add(length)?;
    Some((bytes.get(payload_start..end)?, end))
}

fn parse_jpeg_orientation(payload: &[u8], current: u16) -> u16 {
    let Some(tiff_data) = payload.strip_prefix(b"Exif\0\0") else {
        return current;
    };
    let Ok(exif_fields) = exif::parse_exif(tiff_data) else {
        return current;
    };
    exif_fields
        .0
        .iter()
        .find_map(|field| {
            if field.tag != exif::Tag::Orientation {
                return None;
            }
            match field.value {
                exif::Value::Short(ref values) => values.first().copied(),
                _ => None,
            }
        })
        .unwrap_or(current)
}

fn parse_jpeg_quantization_tables(payload: &[u8], tables: &mut [[i32; 64]; 4]) -> Option<()> {
    let mut pos = 0usize;
    while pos < payload.len() {
        let descriptor = *payload.get(pos)?;
        pos += 1;
        let precision = descriptor >> 4;
        let table_id = usize::from(descriptor & 0x0F);
        let table = tables.get_mut(table_id)?;
        match precision {
            0 => {
                let values = payload.get(pos..pos.checked_add(64)?)?;
                for (target, value) in table.iter_mut().zip(values) {
                    *target = i32::from(*value);
                }
                pos += 64;
            }
            1 => {
                let values = payload.get(pos..pos.checked_add(128)?)?;
                for (target, value) in table.iter_mut().zip(values.chunks_exact(2)) {
                    *target = i32::from(u16::from_be_bytes([value[0], value[1]]));
                }
                pos += 128;
            }
            _ => return None,
        }
    }
    Some(())
}

fn parse_jpeg_huffman_tables(
    payload: &[u8],
    dc_tables: &mut [Option<HuffLut>; 4],
    ac_tables: &mut [Option<HuffLut>; 4],
) -> Option<()> {
    let mut pos = 0usize;
    while pos < payload.len() {
        let descriptor = *payload.get(pos)?;
        pos += 1;
        let class = descriptor >> 4;
        let table_id = usize::from(descriptor & 0x0F);
        let counts_slice = payload.get(pos..pos.checked_add(16)?)?;
        let mut counts = [0u8; 16];
        counts.copy_from_slice(counts_slice);
        pos += 16;
        let symbol_count: usize = counts.iter().map(|&count| usize::from(count)).sum();
        let symbols = payload.get(pos..pos.checked_add(symbol_count)?)?;
        pos += symbol_count;
        let table = HuffLut::build(&counts, symbols);
        match class {
            0 => dc_tables.get_mut(table_id)?.replace(table),
            1 => ac_tables.get_mut(table_id)?.replace(table),
            _ => return None,
        };
    }
    Some(())
}

fn parse_jpeg_components(payload: &[u8]) -> Option<(u16, u16, Vec<JpegComponent>)> {
    if *payload.first()? != 8 {
        return None;
    }
    let height = u16::from_be_bytes([*payload.get(1)?, *payload.get(2)?]);
    let width = u16::from_be_bytes([*payload.get(3)?, *payload.get(4)?]);
    let count = usize::from(*payload.get(5)?);
    if width == 0 || height == 0 || !(1..=4).contains(&count) {
        return None;
    }
    let component_bytes = payload.get(6..6usize.checked_add(count.checked_mul(3)?)?)?;
    let mut components = Vec::with_capacity(count);
    for component in component_bytes.chunks_exact(3) {
        let sampling = component[1];
        let h_samp = usize::from(sampling >> 4);
        let v_samp = usize::from(sampling & 0x0F);
        let qt_id = usize::from(component[2]);
        if h_samp == 0 || v_samp == 0 || qt_id >= 4 {
            return None;
        }
        components.push(JpegComponent {
            h_samp,
            v_samp,
            qt_id,
            dc_table: 0,
            ac_table: 0,
        });
    }
    Some((width, height, components))
}

fn parse_jpeg_scan(payload: &[u8], components: &mut [JpegComponent]) -> Option<()> {
    let count = usize::from(*payload.first()?);
    if count != components.len() {
        return None;
    }
    let selectors_end = 1usize.checked_add(count.checked_mul(2)?)?;
    let selectors = payload.get(1..selectors_end)?;
    payload.get(selectors_end..selectors_end.checked_add(3)?)?;
    for (component, selector) in components.iter_mut().zip(selectors.chunks_exact(2)) {
        let tables = selector[1];
        component.dc_table = usize::from(tables >> 4);
        component.ac_table = usize::from(tables & 0x0F);
        if component.dc_table >= 4 || component.ac_table >= 4 {
            return None;
        }
    }
    Some(())
}

fn parse_jpeg_header(bytes: &[u8]) -> Option<JpegHeader> {
    if !bytes.starts_with(&[0xFF, 0xD8]) {
        return None;
    }
    let mut header = JpegHeader {
        qt_tables: [[0; 64]; 4],
        dc_huff: [const { None }; 4],
        ac_huff: [const { None }; 4],
        components: Vec::new(),
        width: 0,
        height: 0,
        restart_interval: 0,
        orientation: 1,
        entropy_start: 0,
    };
    let mut pos = 2usize;
    loop {
        let marker_start = pos.checked_add(1)?;
        if *bytes.get(pos)? != 0xFF {
            return None;
        }
        let marker = *bytes.get(marker_start)?;
        pos = pos.checked_add(2)?;
        if marker == 0xD8 {
            continue;
        }
        if marker == 0xD9 || marker == 0xC2 {
            return None;
        }
        let (payload, end) = jpeg_segment(bytes, pos)?;
        match marker {
            0xE1 => {
                header.orientation = parse_jpeg_orientation(payload, header.orientation);
            }
            0xDB => parse_jpeg_quantization_tables(payload, &mut header.qt_tables)?,
            0xC4 => {
                parse_jpeg_huffman_tables(payload, &mut header.dc_huff, &mut header.ac_huff)?;
            }
            0xC0 => {
                (header.width, header.height, header.components) = parse_jpeg_components(payload)?;
            }
            0xDD => {
                header.restart_interval = u16::from_be_bytes([*payload.first()?, *payload.get(1)?]);
            }
            0xDA => {
                parse_jpeg_scan(payload, &mut header.components)?;
                header.entropy_start = end;
                return Some(header);
            }
            _ => {}
        }
        pos = end;
    }
}

/// Decode a baseline JPEG at 1/8 scale using only DC coefficients.
fn decode_jpeg_dc_only(bytes: &[u8], max_side: u32) -> Option<(DecodedImage, ImageMeta)> {
    let JpegHeader {
        qt_tables,
        dc_huff,
        ac_huff,
        components,
        width,
        height,
        restart_interval,
        orientation,
        entropy_start,
    } = parse_jpeg_header(bytes)?;

    let w = width as usize;
    let h = height as usize;
    let h_max = components.iter().map(|c| c.h_samp).max().unwrap_or(1);
    let v_max = components.iter().map(|c| c.v_samp).max().unwrap_or(1);
    let mcu_w = w.div_ceil(h_max * 8);
    let mcu_h = h.div_ceil(v_max * 8);
    // Allocate DC coefficient planes for each component
    let mut dc_planes: Vec<Vec<f32>> = components
        .iter()
        .map(|c| vec![0.0f32; mcu_w * c.h_samp * mcu_h * c.v_samp])
        .collect();

    let mut reader = BitReader::new(bytes, entropy_start);
    let mut dc_pred = [0i32; 4];
    let mut mcu_count = 0u32;

    for mcu_y in 0..mcu_h {
        for mcu_x in 0..mcu_w {
            // Handle restart markers
            if restart_interval > 0
                && mcu_count > 0
                && mcu_count.is_multiple_of(restart_interval as u32)
            {
                reader.skip_restart_marker();
                dc_pred = [0i32; 4];
            }

            for (ci, comp) in components.iter().enumerate() {
                let dc_lut = dc_huff[comp.dc_table].as_ref()?;
                let ac_lut = ac_huff[comp.ac_table].as_ref()?;
                let qt_dc = qt_tables.get(comp.qt_id).map(|t| t[0]).unwrap_or(1);

                for vy in 0..comp.v_samp {
                    for hx in 0..comp.h_samp {
                        // Decode DC coefficient
                        let cat = reader.decode_huff(dc_lut)?;
                        let dc_diff = if cat == 0 {
                            0
                        } else {
                            let raw = reader.get_bits(cat);
                            // Extend sign
                            if raw < (1 << (cat - 1)) {
                                raw - (1 << cat) + 1
                            } else {
                                raw
                            }
                        };
                        dc_pred[ci] += dc_diff;

                        // Skip AC coefficients (must parse to maintain stream position)
                        let mut ac_count = 0;
                        while ac_count < 63 {
                            let sym = reader.decode_huff(ac_lut)?;
                            if sym == 0x00 {
                                break; // EOB
                            }
                            let run = (sym >> 4) as usize;
                            let size = sym & 0x0F;
                            ac_count += run + 1;
                            if size > 0 {
                                reader.get_bits(size);
                            }
                            // ZRL (0xF0): run=15, size=0 → skip 16 zeros
                        }

                        // Store dequantized DC value
                        let dc_val = (dc_pred[ci] * qt_dc) as f32;
                        let plane_w = mcu_w * comp.h_samp;
                        let px = mcu_x * comp.h_samp + hx;
                        let py = mcu_y * comp.v_samp + vy;
                        let idx = py * plane_w + px;
                        if idx < dc_planes[ci].len() {
                            dc_planes[ci][idx] = dc_val;
                        }
                    }
                }
            }
            mcu_count += 1;
        }
    }

    // Assemble output image from DC planes
    // Output dimensions: 1 pixel per 8×8 block (accounting for max sampling)
    let out_w = mcu_w * h_max;
    let out_h = mcu_h * v_max;
    let nc = components.len();

    let mut rgba = vec![0u8; out_w * out_h * 4];

    for y in 0..out_h {
        for x in 0..out_w {
            let idx = (y * out_w + x) * 4;
            if nc == 1 {
                // Grayscale
                let luma_w = mcu_w * components[0].h_samp;
                let lx = x * components[0].h_samp / h_max;
                let ly = y * components[0].v_samp / v_max;
                let li = ly * luma_w + lx;
                let v = (dc_planes[0].get(li).copied().unwrap_or(0.0) / 8.0 + 128.0)
                    .clamp(0.0, 255.0) as u8;
                rgba[idx] = v;
                rgba[idx + 1] = v;
                rgba[idx + 2] = v;
                rgba[idx + 3] = 255;
            } else {
                // YCbCr → RGB
                let y_w = mcu_w * components[0].h_samp;
                let cb_w = mcu_w * components[1].h_samp;
                let cr_w = mcu_w * components[2].h_samp;

                let yx = x * components[0].h_samp / h_max;
                let yy_coord = y * components[0].v_samp / v_max;
                let yi = yy_coord * y_w + yx;
                let yv = dc_planes[0].get(yi).copied().unwrap_or(0.0) / 8.0 + 128.0;

                let cbx = x * components[1].h_samp / h_max;
                let cby = y * components[1].v_samp / v_max;
                let cbi = cby * cb_w + cbx;
                let cb = dc_planes[1].get(cbi).copied().unwrap_or(0.0) / 8.0 + 128.0;

                let crx = x * components[2].h_samp / h_max;
                let cry = y * components[2].v_samp / v_max;
                let cri = cry * cr_w + crx;
                let cr = dc_planes[2].get(cri).copied().unwrap_or(0.0) / 8.0 + 128.0;

                let r = (yv + 1.402 * (cr - 128.0)).clamp(0.0, 255.0) as u8;
                let g = (yv - 0.344136 * (cb - 128.0) - 0.714136 * (cr - 128.0)).clamp(0.0, 255.0)
                    as u8;
                let b = (yv + 1.772 * (cb - 128.0)).clamp(0.0, 255.0) as u8;
                rgba[idx] = r;
                rgba[idx + 1] = g;
                rgba[idx + 2] = b;
                rgba[idx + 3] = 255;
            }
        }
    }

    // Trim to actual image dimensions at 1/8 scale
    let real_w = w.div_ceil(8).min(out_w);
    let real_h = h.div_ceil(8).min(out_h);
    if real_w < out_w || real_h < out_h {
        let mut trimmed = vec![0u8; real_w * real_h * 4];
        for y in 0..real_h {
            let src_off = y * out_w * 4;
            let dst_off = y * real_w * 4;
            trimmed[dst_off..dst_off + real_w * 4]
                .copy_from_slice(&rgba[src_off..src_off + real_w * 4]);
        }
        rgba = trimmed;
    }
    let (final_w, final_h) = (real_w, real_h);

    let (rgba, final_w, final_h) = apply_orientation_rgba(rgba, final_w, final_h, orientation);
    let (out_w, out_h, out_rgba) = downscale_rgba(&rgba, final_w, final_h, max_side);
    let color = egui::ColorImage::from_rgba_unmultiplied([out_w, out_h], &out_rgba);
    Some((
        DecodedImage::Static(color),
        ImageMeta {
            width: w,
            height: h,
            depth: zune_core::bit_depth::BitDepth::Eight,
        },
    ))
}

fn exif_orientation(image: &zune_image::image::Image) -> Option<u16> {
    let exif = image.metadata().exif()?;
    for field in exif {
        if field.tag == exif::Tag::Orientation
            && let exif::Value::Short(ref values) = field.value
        {
            return values.first().copied();
        }
    }
    None
}

fn apply_orientation_rgba(
    rgba: Vec<u8>,
    width: usize,
    height: usize,
    orientation: u16,
) -> (Vec<u8>, usize, usize) {
    match orientation {
        2 => (flip_horizontal(&rgba, width, height), width, height),
        3 => (rotate_180(&rgba, width, height), width, height),
        4 => (flip_vertical(&rgba, width, height), width, height),
        5 => (
            transpose_flip_horizontal(&rgba, width, height),
            height,
            width,
        ),
        6 => (rotate_90_cw(&rgba, width, height), height, width),
        7 => (transpose_flip_vertical(&rgba, width, height), height, width),
        8 => (rotate_90_ccw(&rgba, width, height), height, width),
        _ => (rgba, width, height),
    }
}

fn flip_horizontal(rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; rgba.len()];
    for y in 0..height {
        for x in 0..width {
            let src = (y * width + x) * 4;
            let dst = (y * width + (width - 1 - x)) * 4;
            out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
        }
    }
    out
}

fn flip_vertical(rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; rgba.len()];
    for y in 0..height {
        for x in 0..width {
            let src = (y * width + x) * 4;
            let dst = ((height - 1 - y) * width + x) * 4;
            out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
        }
    }
    out
}

fn rotate_180(rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; rgba.len()];
    for y in 0..height {
        for x in 0..width {
            let src = (y * width + x) * 4;
            let dst = ((height - 1 - y) * width + (width - 1 - x)) * 4;
            out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
        }
    }
    out
}

fn rotate_90_cw(rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; rgba.len()];
    for y in 0..height {
        for x in 0..width {
            let src = (y * width + x) * 4;
            let dst_x = height - 1 - y;
            let dst_y = x;
            let dst = (dst_y * height + dst_x) * 4;
            out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
        }
    }
    out
}

fn rotate_90_ccw(rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; rgba.len()];
    for y in 0..height {
        for x in 0..width {
            let src = (y * width + x) * 4;
            let dst_x = y;
            let dst_y = width - 1 - x;
            let dst = (dst_y * height + dst_x) * 4;
            out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
        }
    }
    out
}

fn transpose_flip_horizontal(rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; rgba.len()];
    for y in 0..height {
        for x in 0..width {
            let src = (y * width + x) * 4;
            let dst_x = height - 1 - y;
            let dst_y = width - 1 - x;
            let dst = (dst_y * height + dst_x) * 4;
            out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
        }
    }
    out
}

fn transpose_flip_vertical(rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; rgba.len()];
    for y in 0..height {
        for x in 0..width {
            let src = (y * width + x) * 4;
            let dst_x = y;
            let dst_y = x;
            let dst = (dst_y * height + dst_x) * 4;
            out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
        }
    }
    out
}

fn convert_to_rgba(
    data: &[u8],
    width: usize,
    height: usize,
    colorspace: zune_core::colorspace::ColorSpace,
) -> Option<Vec<u8>> {
    let pixels = width.checked_mul(height)?;
    match colorspace {
        zune_core::colorspace::ColorSpace::RGBA => {
            if data.len() == pixels * 4 {
                Some(data.to_vec())
            } else {
                None
            }
        }
        zune_core::colorspace::ColorSpace::RGB => {
            if data.len() != pixels * 3 {
                return None;
            }
            let mut out = Vec::with_capacity(pixels * 4);
            for chunk in data.chunks_exact(3) {
                out.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
            }
            Some(out)
        }
        zune_core::colorspace::ColorSpace::BGR => {
            if data.len() != pixels * 3 {
                return None;
            }
            let mut out = Vec::with_capacity(pixels * 4);
            for chunk in data.chunks_exact(3) {
                out.extend_from_slice(&[chunk[2], chunk[1], chunk[0], 255]);
            }
            Some(out)
        }
        zune_core::colorspace::ColorSpace::BGRA => {
            if data.len() != pixels * 4 {
                return None;
            }
            let mut out = Vec::with_capacity(pixels * 4);
            for chunk in data.chunks_exact(4) {
                out.extend_from_slice(&[chunk[2], chunk[1], chunk[0], chunk[3]]);
            }
            Some(out)
        }
        zune_core::colorspace::ColorSpace::ARGB => {
            if data.len() != pixels * 4 {
                return None;
            }
            let mut out = Vec::with_capacity(pixels * 4);
            for chunk in data.chunks_exact(4) {
                out.extend_from_slice(&[chunk[1], chunk[2], chunk[3], chunk[0]]);
            }
            Some(out)
        }
        zune_core::colorspace::ColorSpace::Luma => {
            if data.len() != pixels {
                return None;
            }
            let mut out = Vec::with_capacity(pixels * 4);
            for &v in data {
                out.extend_from_slice(&[v, v, v, 255]);
            }
            Some(out)
        }
        zune_core::colorspace::ColorSpace::LumaA => {
            if data.len() != pixels * 2 {
                return None;
            }
            let mut out = Vec::with_capacity(pixels * 4);
            for chunk in data.chunks_exact(2) {
                out.extend_from_slice(&[chunk[0], chunk[0], chunk[0], chunk[1]]);
            }
            Some(out)
        }
        _ => None,
    }
}

fn downscale_rgba(
    rgba: &[u8],
    width: usize,
    height: usize,
    max_side: u32,
) -> (usize, usize, Vec<u8>) {
    let max_dim = width.max(height);
    if max_dim <= max_side as usize {
        return (width, height, rgba.to_vec());
    }
    let scale = max_side as f32 / max_dim as f32;
    let out_w = (width as f32 * scale).round().max(1.0) as usize;
    let out_h = (height as f32 * scale).round().max(1.0) as usize;
    let mut out = vec![0u8; out_w * out_h * 4];
    for y in 0..out_h {
        let src_y = y * height / out_h;
        for x in 0..out_w {
            let src_x = x * width / out_w;
            let src_idx = (src_y * width + src_x) * 4;
            let dst_idx = (y * out_w + x) * 4;
            out[dst_idx..dst_idx + 4].copy_from_slice(&rgba[src_idx..src_idx + 4]);
        }
    }
    (out_w, out_h, out)
}

#[cfg(test)]
mod tests {
    use super::decode_jpeg_dc_preview;

    #[test]
    fn malformed_jpeg_segment_lengths_are_rejected() {
        let too_short_app1 = [0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x01];
        let truncated_sof = [0xFF, 0xD8, 0xFF, 0xC0, 0x00, 0x02];

        assert!(decode_jpeg_dc_preview(&too_short_app1, 64).is_none());
        assert!(decode_jpeg_dc_preview(&truncated_sof, 64).is_none());
    }

    #[test]
    fn zero_jpeg_sampling_factor_is_rejected() {
        let bytes = [
            0xFF, 0xD8, // SOI
            0xFF, 0xC0, 0x00, 0x0B, // SOF0, one component
            0x08, 0x00, 0x01, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, // zero sampling
            0xFF, 0xDA, 0x00, 0x08, // SOS
            0x01, 0x01, 0x00, 0x00, 0x3F, 0x00,
        ];

        assert!(decode_jpeg_dc_preview(&bytes, 64).is_none());
    }
}
