use crate::component::Tilemap;
use crate::image::{blit, load_image};
use shipyard::World;
use std::io::Cursor;
use tiled::{LayerData, PropertyValue};
use ultraviolet::Vec2;

// TODO: This should load object layers, too!
fn load_layers(tmx: &str) -> Vec<Tilemap> {
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
    let src_width = tw as isize;
    let stride = tileset_width * 4;
    let tileset_width_in_tiles = tileset_width / tw as isize;
    let layer_width = tw * tmx.width;
    let layer_height = th * tmx.height;
    let dst_size = Vec2::new(layer_width as f32, layer_height as f32);

    for layer in &tmx.layers {
        if let LayerData::Finite(rows) = &layer.tiles {
            let mut image = Vec::with_capacity((layer_width * layer_height * 4) as usize);
            image.resize(image.capacity(), 0);

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
                    let dst_pos = Vec2::new(dst_x as f32, dst_y as f32) * tile_size;
                    let src_pos = Vec2::new(x as f32, y as f32) * tile_size;

                    let start =
                        (src_pos.y as usize * tileset_width as usize + src_pos.x as usize) * 4;
                    let end = start + ((15 * tileset_width + 16) * 4) as usize;

                    blit(
                        &mut image,
                        &tileset_image[start..end],
                        stride,
                        dst_pos,
                        dst_size,
                        src_width,
                    )
                }
            }

            let tilemap = Tilemap {
                width: layer_width as isize,
                height: layer_height as isize,
                image,
                parallax: layer.properties.get("parallax").map_or(1.0, |value| {
                    if let PropertyValue::FloatValue(value) = value {
                        *value
                    } else {
                        1.0
                    }
                }),
            };

            maps.push(tilemap);
        }
    }

    maps
}

pub(crate) fn add_tilemap(world: &mut World, tmx: &str) {
    for layer in load_layers(tmx) {
        world.add_entity((layer,));
    }
}
