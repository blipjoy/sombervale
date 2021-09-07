use crate::image::{self, bad_color_multiply, ImageViewMut};
use crate::power::FrogPower;
use ultraviolet::Vec2;

#[derive(Default)]
pub(crate) struct Hud {
    pub(crate) jean: JeanStats,
    pub(crate) frog_power: Option<FrogPower>,
}

pub(crate) struct JeanStats {
    // Experience points
    max_xp: usize,
    xp: usize,

    // Health points
    max_hp: usize,
    hp: usize,
}

impl Default for JeanStats {
    fn default() -> Self {
        Self {
            max_xp: 10,
            xp: 0,
            max_hp: 10,
            hp: 10,
        }
    }
}

impl Hud {
    pub(crate) fn draw(&self, dest: &mut ImageViewMut<'_>, factor: f32) {
        let mut green = [0x38, 0xb7, 0x64, 0xff];
        let mut purple = [0x5d, 0x27, 0x5d, 0xff];

        bad_color_multiply(&mut green, factor);
        bad_color_multiply(&mut purple, factor);

        // Draw HP meter
        let ratio = self.jean.hp as f32 / self.jean.max_hp as f32;
        draw_meter(dest, Vec2::new(14.0, 3.0), green, ratio, factor);

        // Draw XP meter
        let ratio = self.jean.xp as f32 / self.jean.max_xp as f32;
        draw_meter(dest, Vec2::new(40.0, 3.0), purple, ratio, factor);

        if let Some(frog_power) = &self.frog_power {
            // Draw PP meter
            let ratio = frog_power.pp() as f32 / frog_power.max_pp() as f32;
            draw_meter(dest, Vec2::new(14.0, 13.0), green, ratio, factor);

            // Draw XP meter
            let ratio = frog_power.xp() as f32 / frog_power.max_xp() as f32;
            draw_meter(dest, Vec2::new(40.0, 13.0), purple, ratio, factor);
        }
    }

    pub(crate) fn increase_xp(&mut self) {
        if self.jean.xp < self.jean.max_xp {
            self.jean.xp += 1;
        } else {
            self.jean.max_hp += 1;
            self.jean.max_xp *= 2;
            self.jean.xp = 0;
        }
    }
}

fn draw_meter(dest: &mut ImageViewMut<'_>, mut pos: Vec2, color: [u8; 4], ratio: f32, factor: f32) {
    let mut white = [0xf4, 0xf4, 0xf4, 0xff];
    let mut gray = [0x94, 0xb0, 0xc2, 0xff];

    bad_color_multiply(&mut white, factor);
    bad_color_multiply(&mut gray, factor);

    let size = Vec2::new(20.0, 2.0);

    // Draw border
    let lines = [
        (Vec2::new(1.0, 0.0), Vec2::new(22.0, 0.0)),
        (Vec2::new(23.0, 1.0), Vec2::new(23.0, 4.0)),
        (Vec2::new(1.0, 5.0), Vec2::new(22.0, 5.0)),
        (Vec2::new(0.0, 1.0), Vec2::new(0.0, 4.0)),
    ];
    image::lines(dest, pos + Vec2::unit_y(), white, &lines, factor);

    // Fill meter, active side
    let active_size = Vec2::new(size.x * ratio, size.y);
    pos += Vec2::new(2.0, 3.0);
    image::rect(dest, pos, color, active_size, factor);

    // Fill meter, inactive side
    let inactive_size = Vec2::new(size.x - active_size.x, size.y);
    pos += Vec2::new(active_size.x, 0.0);
    image::rect(dest, pos, gray, inactive_size, factor);
}
