use ultraviolet::Vec2;

pub(crate) fn load_image(png: &[u8]) -> (isize, isize, Vec<u8>) {
    let (header, image) = png_decoder::decode(png).unwrap();

    let width = header.width as isize;
    let height = header.height as isize;

    (width, height, image)
}

pub(crate) fn blit(
    frame: &mut [u8],
    image: &[u8],
    stride: isize,
    dst_pos: Vec2,
    dst_size: Vec2,
    src_width: isize,
) {
    assert!(stride >= src_width * 4);
    assert!(stride % 4 == 0);

    let width = dst_size.x as isize;
    let height = dst_size.y as isize;

    for (ri, row) in image.chunks(stride as usize).enumerate() {
        for (i, color) in row.chunks(4).take(src_width as usize).enumerate() {
            let i = i as isize;
            let x = i % src_width;
            if x >= width {
                break;
            }

            if color[3] == 0xff {
                let x = x + dst_pos.x as isize;
                let y = i / src_width + dst_pos.y as isize + ri as isize;

                if y >= height {
                    return;
                }

                if x >= 0 && x < width && y >= 0 {
                    let index = ((y * width + x) * 4) as usize;
                    frame[index..index + 4].copy_from_slice(color);
                }
            }
        }
    }
}
