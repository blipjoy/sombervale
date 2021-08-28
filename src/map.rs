use crate::component::{Collision, Random, Tilemap, Viewport};
use crate::entity;
use crate::image::{blit, load_image, Image, ImageViewMut};
use shipyard::{AllStoragesViewMut, UniqueViewMut};
use std::collections::HashMap;
use std::io::Cursor;
use tiled::{LayerData, Object, ObjectShape, PropertyValue};
use ultraviolet::{Vec2, Vec3};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Rect {
    pos: Vec2,
    size: Vec2,
}

impl Rect {
    fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }

    fn point_intersects(&self, point: Vec3) -> bool {
        let lower_right = self.pos + self.size;

        point.x > self.pos.x
            && point.x < lower_right.x
            && point.z > self.pos.y
            && point.z < lower_right.y
    }

    pub(crate) fn circle_intersects(&self, point: Vec3, radius: f32) -> bool {
        let mut extended = *self;
        extended.pos -= Vec2::broadcast(radius);
        extended.size += Vec2::broadcast(radius * 2.0);

        extended.point_intersects(point)
    }
}

pub(crate) fn add_tilemap(mut storages: AllStoragesViewMut, tmx: &str) {
    let tmx = tiled::parse(Cursor::new(tmx)).unwrap();
    let mut shapes = Vec::new();

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

    // Load all object groups
    for group in &tmx.object_groups {
        // TODO: hardcoding group names for now
        match group.name.as_str() {
            "Collision" => load_collision_shapes(&mut shapes, dst_size, &group.objects),
            "Entities" => load_entities(&mut storages, dst_size, &group.objects),
            _ => {
                panic!("Group name {} is not supported", group.name);
            }
        }
    }

    let mut layers = Vec::new();
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

                    blit(&mut dest, dest_pos, &src, src_pos, tile_size, 1.0);
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

            layers.push((tilemap,));
        }
    }
    storages.bulk_add_entity(layers.into_iter());

    let viewport = Viewport {
        pos: Vec2::default(),
        world_height: layer_height as f32,
    };
    storages.add_unique(viewport);

    let collision = Collision { shapes };
    storages.add_unique(collision);
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

fn load_collision_shapes(shapes: &mut Vec<Rect>, map_size: Vec2, objects: &[Object]) {
    for object in objects {
        match &object.shape {
            ObjectShape::Rect { width, height } => {
                assert!((object.width - width).abs() < f32::EPSILON);
                assert!((object.height - height).abs() < f32::EPSILON);

                let pos = Vec2::new(object.x, map_size.y - object.y - height);
                let size = Vec2::new(*width, *height);
                shapes.push(Rect::new(pos, size));
            }
            shape => {
                panic!("Collision shape not supported: {:?}", shape);
            }
        }
    }
}

fn load_entities(storages: &mut AllStoragesViewMut, map_size: Vec2, objects: &[Object]) {
    for object in objects {
        match (&object.shape, object.name.as_str()) {
            (ObjectShape::Rect { width, height }, "Jean") => {
                assert!((object.width - width).abs() < f32::EPSILON);
                assert!((object.height - height).abs() < f32::EPSILON);

                let pos = Vec3::new(object.x + width / 2.0, 0.0, map_size.y - object.y - height);
                storages.add_entity(entity::jean(pos));
            }
            (ObjectShape::Rect { width, height }, "Blob") => {
                let mut random = storages
                    .borrow::<UniqueViewMut<Random>>()
                    .expect("Need random");

                let pos = Vec3::new(object.x + width / 2.0, 0.0, map_size.y - object.y - height);
                let blob = entity::blob(pos, &object.properties, &mut random.0);
                drop(random);

                storages.add_entity(blob);
            }
            (ObjectShape::Rect { width, height }, "Fire") => {
                let mut random = storages
                    .borrow::<UniqueViewMut<Random>>()
                    .expect("Need random");

                let pos = Vec3::new(object.x + width / 2.0, 0.0, map_size.y - object.y - height);
                let fire = entity::fire(pos, &mut random.0);
                drop(random);

                storages.add_entity(fire);
            }
            (shape, name) => {
                panic!("Entity named {} not supported: {:?}", name, shape);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_circle_intersects() {
        let pos = Vec2::new(3.0, 4.0);
        let size = Vec2::new(3.0, 3.0);
        let rect = Rect::new(pos, size);

        let radius = 2.0;

        // Check intersections below the rect
        assert!(!rect.circle_intersects(Vec3::new(5.0, 0.0, 10.0), radius));
        assert!(rect.circle_intersects(Vec3::new(5.0, 0.0, 8.0), radius));

        // TODO: Ignore corner intersections for now
        // // Check intersections to the lower left of rect
        // let x = dbg!(3.0 - 2.0 * (TAU / 8.0).cos());
        // let y = dbg!(7.0 + 2.0 * (TAU / 8.0).sin());
        // assert!(!rect.circle_intersects(Vec3::new(x - 0.01, 0.0, y + 0.01), radius));
        // assert!(rect.circle_intersects(Vec3::new(x + 0.01, 0.0, y - 0.01), radius));

        // Check intersections to the left of rect
        assert!(!rect.circle_intersects(Vec3::new(0.0, 0.0, 5.0), radius));
        assert!(rect.circle_intersects(Vec3::new(2.0, 0.0, 5.0), radius));

        // Check intersections above the rect
        assert!(!rect.circle_intersects(Vec3::new(5.0, 0.0, 1.0), radius));
        assert!(rect.circle_intersects(Vec3::new(5.0, 0.0, 3.0), radius));

        // Check intersections to the right of rect
        assert!(!rect.circle_intersects(Vec3::new(9.0, 0.0, 5.0), radius));
        assert!(rect.circle_intersects(Vec3::new(7.0, 0.0, 5.0), radius));
    }
}
