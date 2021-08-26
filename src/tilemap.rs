use crate::component::{Tilemap, Viewport};
use crate::image::{blit, load_image, Image, ImageViewMut};
use shipyard::World;
use std::collections::HashMap;
use std::io::Cursor;
use tiled::{LayerData, PropertyValue};
use ultraviolet::Vec2;

pub(crate) fn add_tilemap(world: &mut World, tmx: &str) {
    let (viewport, layers) = load_tilemap(tmx);

    world.add_unique(viewport).expect("Add viewport to world");

    for layer in layers {
        world.add_entity((layer,));
    }
}

// TODO: This should load object layers, too!
fn load_tilemap(tmx: &str) -> (Viewport, Vec<Tilemap>) {
    let tmx = tiled::parse(Cursor::new(tmx)).unwrap();
    let mut maps = Vec::new();

    // Don't want to implement features that I don't use
    let tw = tmx.tile_width;
    let th = tmx.tile_height;
    let tileset = &tmx.tilesets[0];
    assert_eq!(tileset.tile_width, tw);
    assert_eq!(tileset.tile_height, th);
    assert_eq!(tileset.spacing, 0);
    assert_eq!(tileset.margin, 0);

    // TODO: hardcoding the tileset image for now
    // TODO: Move the image blitting into a build script
    let (tileset_width, tileset_height, tileset_image) =
        load_image(include_bytes!("../assets/tileset.png"));
    assert_eq!(tileset_width % tw as isize, 0);
    assert_eq!(tileset_height % th as isize, 0);

    let tile_size = Vec2::new(tw as f32, th as f32);
    let tileset_width_in_tiles = tileset_width / tw as isize;
    let layer_width = tw * tmx.width;
    let layer_height = th * tmx.height;
    let dst_size = Vec2::new(layer_width as f32, layer_height as f32);

    let src_size = Vec2::new(tileset_width as f32, tileset_height as f32);
    let src = Image::new(tileset_image, src_size);

    for layer in &tmx.layers {
        if let LayerData::Finite(rows) = &layer.tiles {
            let mut image = Vec::with_capacity((layer_width * layer_height * 4) as usize);
            image.resize(image.capacity(), 0);
            let mut dest = ImageViewMut::new(&mut image, dst_size);

            for (dst_y, cols) in rows.iter().enumerate() {
                for (dst_x, tile) in cols.iter().enumerate() {
                    if tile.gid == 0 {
                        continue;
                    }

                    // I don't want to implement tile flipping (if I don't have to)
                    assert!(!tile.flip_h);
                    assert!(!tile.flip_v);
                    assert!(!tile.flip_d);

                    let tile_id = (tile.gid - tileset.first_gid) as isize;
                    let x = tile_id % tileset_width_in_tiles;
                    let y = tile_id / tileset_width_in_tiles;
                    let dest_pos = Vec2::new(dst_x as f32, dst_y as f32) * tile_size;
                    let src_pos = Vec2::new(x as f32, y as f32) * tile_size;

                    blit(&mut dest, dest_pos, &src, src_pos, tile_size);
                }
            }

            // DEBUG
            #[cfg(feature = "debug-mode")]
            {
                let file_name = format!("{}.tiff", layer.name);
                let mut file = std::fs::File::create(&file_name).unwrap();
                let mut tiff = tiff::encoder::TiffEncoder::new(&mut file).unwrap();
                let tiff = tiff
                    .new_image::<tiff::encoder::colortype::RGBA8>(
                        layer_width as u32,
                        layer_height as u32,
                    )
                    .unwrap();
                tiff.write_data(&image).unwrap();
            }

            let image = Image::new(image, dst_size);
            let parallax = get_parallax(&layer.properties);
            let tilemap = Tilemap { image, parallax };

            maps.push(tilemap);
        }
    }

    let viewport = Viewport {
        pos: Vec2::default(),
        world_height: layer_height as f32,
    };

    (viewport, maps)
}

fn get_parallax(properties: &HashMap<String, PropertyValue>) -> Vec2 {
    let parallax_x = properties.get("parallax_x").map_or(1.0, |value| {
        if let PropertyValue::FloatValue(value) = value {
            *value
        } else {
            1.0
        }
    });
    let parallax_y = properties.get("parallax_y").map_or(1.0, |value| {
        if let PropertyValue::FloatValue(value) = value {
            *value
        } else {
            1.0
        }
    });

    Vec2::new(parallax_x, parallax_y)
}
