use ultraviolet::Vec2;

pub(crate) struct Image {
    data: Vec<u8>,
    size: Vec2,
}

pub(crate) struct ImageViewMut<'data> {
    data: &'data mut [u8],
    size: Vec2,
}

impl Image {
    pub(crate) fn new(data: Vec<u8>, size: Vec2) -> Self {
        Self { data, size }
    }

    pub(crate) fn size(&self) -> Vec2 {
        self.size
    }
}

impl<'data> ImageViewMut<'data> {
    pub(crate) fn new(data: &'data mut [u8], size: Vec2) -> Self {
        Self { data, size }
    }
}

pub(crate) fn load_image(png: &[u8]) -> (isize, isize, Vec<u8>) {
    let (header, image) = png_decoder::decode(png).unwrap();

    let width = header.width as isize;
    let height = header.height as isize;

    (width, height, image)
}

pub(crate) fn bad_color_multiply(color: &mut [u8; 4], factor: f32) {
    fn mult_u8(value: &mut u8, factor: f32) {
        *value = (*value as f32 * factor) as u8;
    }

    mult_u8(&mut color[0], factor);
    mult_u8(&mut color[1], factor);
    mult_u8(&mut color[2], factor);
}

pub(crate) fn blit<'dest>(
    dest: &mut ImageViewMut<'dest>,
    mut dest_pos: Vec2,
    src: &Image,
    mut src_pos: Vec2,
    mut size: Vec2,
    factor: f32,
) {
    assert!(size.x <= src.size.x);
    assert!(size.y <= src.size.y);
    assert!(size.x <= dest.size.x);
    assert!(size.y <= dest.size.y);

    // Account for src_pos being negative
    if src_pos.x < 0.0 {
        let offset = src_pos.x.abs();
        dest_pos.x += offset;
        size.x -= offset;
        src_pos.x = 0.0;
    }
    if src_pos.y < 0.0 {
        let offset = src_pos.y.abs();
        dest_pos.y += offset;
        size.y -= offset;
        src_pos.y = 0.0;
    }

    // Account for src_pos being greater than src_size
    if src_pos.x >= src.size.x || src_pos.y >= src.size.y {
        return;
    }

    // Adjust the size to prevent wrapping
    if src_pos.x + size.x > src.size.x {
        size.x -= src_pos.x + size.x - src.size.x;
        if size.x <= 0.0 {
            return;
        }
    }

    // Bail early when the dest_pos is outside of the dest
    let lower_right = dest_pos + size;
    if dest_pos.x > dest.size.x
        || dest_pos.y > dest.size.y
        || lower_right.x < 0.0
        || lower_right.y < 0.0
    {
        return;
    }

    // TODO: Use f32::mul_add()
    let src_index = (src_pos.y * src.size.x + src_pos.x) as usize * 4;
    let slice = &src.data[src_index..];
    let rows = slice.chunks(src.size.x as usize * 4).take(size.y as usize);

    let dest_x = dest_pos.x as isize;
    let dest_y = dest_pos.y as isize;
    let dest_width = dest.size.x as isize;
    let dest_height = dest.size.y as isize;

    for (y, row) in rows.enumerate() {
        for (x, color) in row.chunks_exact(4).take(size.x as usize).enumerate() {
            if color[3] == 0xff {
                let x = x as isize + dest_x;
                let y = y as isize + dest_y;

                // Early bail when drawing below destination image
                if y >= dest_height as isize {
                    return;
                }

                if x >= 0 && x < dest_width && y >= 0 {
                    let mut factored_color = [0; 4];
                    factored_color.copy_from_slice(color);
                    bad_color_multiply(&mut factored_color, factor);

                    let index = ((y * dest_width + x) * 4) as usize;
                    dest.data[index..index + 4].copy_from_slice(&factored_color);
                }
            }
        }
    }
}
