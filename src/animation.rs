use randomize::PCG32;
use std::time::{Duration, Instant};
use tiled::PropertyValue;

pub(crate) trait Animated {
    fn animate(&mut self) -> usize;
}

struct Frame {
    index: usize,
    duration: Duration,
}

struct Animation {
    frames: Vec<Frame>,
    current_index: usize,
    start_time: Instant,
}

impl Animation {
    fn new(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            current_index: 0,
            start_time: Instant::now(),
        }
    }

    fn get_frame(&self) -> &Frame {
        &self.frames[self.current_index % self.frames.len()]
    }

    fn update(&mut self) -> usize {
        let dur = self.get_frame().duration;

        if self.start_time.elapsed() > dur {
            self.current_index += 1;
            self.current_index %= self.frames.len();
            self.start_time = Instant::now();
        }

        self.frames[self.current_index].index
    }

    fn reset(&mut self) {
        self.current_index = 0;
        self.start_time = Instant::now();
    }
}

impl Frame {
    fn new(index: usize, duration: Duration) -> Self {
        Self { index, duration }
    }
}

pub(crate) struct FrogAnims {
    playing: FrogCurrentAnim,
    idle_right: Animation,
    idle_left: Animation,
    hop_right: Animation,
    hop_left: Animation,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum FrogCurrentAnim {
    IdleRight,
    IdleLeft,
    HopRight,
    HopLeft,
}

pub(crate) struct JeanAnims {
    playing: JeanCurrentAnim,
    idle_right: Animation,
    idle_left: Animation,
    walk_right: Animation,
    walk_left: Animation,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum JeanCurrentAnim {
    IdleRight,
    IdleLeft,
    WalkRight,
    WalkLeft,
}

pub(crate) struct BlobAnims {
    playing: BlobCurrentAnim,
    idle_right: Animation,
    idle_left: Animation,
    bounce_right: Animation,
    bounce_left: Animation,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum BlobCurrentAnim {
    IdleRight,
    IdleLeft,
    BounceRight,
    BounceLeft,
}

pub(crate) struct FireAnims {
    burn: Animation,
}

impl FrogAnims {
    pub(crate) fn new() -> Self {
        Self {
            playing: FrogCurrentAnim::IdleRight,
            idle_right: Animation::new(vec![Frame::new(0, Duration::from_secs(1))]),
            idle_left: Animation::new(vec![Frame::new(5, Duration::from_secs(1))]),
            hop_right: Animation::new(vec![
                Frame::new(0, Duration::from_millis(100)),
                Frame::new(1, Duration::from_millis(100)),
                Frame::new(2, Duration::from_millis(100)),
                Frame::new(3, Duration::from_millis(100)),
                Frame::new(4, Duration::from_millis(200)),
            ]),
            hop_left: Animation::new(vec![
                Frame::new(5, Duration::from_millis(100)),
                Frame::new(6, Duration::from_millis(100)),
                Frame::new(7, Duration::from_millis(100)),
                Frame::new(8, Duration::from_millis(100)),
                Frame::new(9, Duration::from_millis(200)),
            ]),
        }
    }

    pub(crate) fn set(&mut self, next: FrogCurrentAnim) {
        self.playing = next;

        // Reset the animation
        let animation = match self.playing {
            FrogCurrentAnim::IdleRight => &mut self.idle_right,
            FrogCurrentAnim::IdleLeft => &mut self.idle_left,
            FrogCurrentAnim::HopRight => &mut self.hop_right,
            FrogCurrentAnim::HopLeft => &mut self.hop_left,
        };

        animation.reset();
    }

    pub(crate) fn playing(&self) -> FrogCurrentAnim {
        self.playing
    }

    pub(crate) fn get_frame_index(&self) -> usize {
        match self.playing {
            FrogCurrentAnim::IdleRight => self.idle_right.get_frame().index,
            FrogCurrentAnim::IdleLeft => self.idle_left.get_frame().index,
            FrogCurrentAnim::HopRight => self.hop_right.get_frame().index,
            FrogCurrentAnim::HopLeft => self.hop_left.get_frame().index,
        }
    }
}

impl Animated for FrogAnims {
    fn animate(&mut self) -> usize {
        // Hopping animations will switch to idle after the animation cycle completes
        match self.playing {
            FrogCurrentAnim::IdleRight => self.idle_right.update(),
            FrogCurrentAnim::IdleLeft => self.idle_left.update(),
            FrogCurrentAnim::HopRight => {
                let last_frame_index = self.hop_right.get_frame().index;
                let frame_index = self.hop_right.update();

                if last_frame_index == 4 && frame_index == 0 {
                    self.set(FrogCurrentAnim::IdleRight);
                }

                frame_index
            }
            FrogCurrentAnim::HopLeft => {
                let last_frame_index = self.hop_left.get_frame().index;
                let frame_index = self.hop_left.update();

                if last_frame_index == 9 && frame_index == 5 {
                    self.set(FrogCurrentAnim::IdleLeft);
                }

                frame_index
            }
        }
    }
}

impl JeanAnims {
    pub(crate) fn new() -> Self {
        Self {
            playing: JeanCurrentAnim::IdleRight,
            idle_right: Animation::new(vec![Frame::new(0, Duration::from_secs(1))]),
            idle_left: Animation::new(vec![Frame::new(9, Duration::from_secs(1))]),
            walk_right: Animation::new(vec![
                Frame::new(1, Duration::from_millis(80)),
                Frame::new(2, Duration::from_millis(80)),
                Frame::new(3, Duration::from_millis(80)),
                Frame::new(4, Duration::from_millis(80)),
                Frame::new(5, Duration::from_millis(80)),
                Frame::new(6, Duration::from_millis(80)),
                Frame::new(7, Duration::from_millis(80)),
                Frame::new(8, Duration::from_millis(80)),
            ]),
            walk_left: Animation::new(vec![
                Frame::new(10, Duration::from_millis(80)),
                Frame::new(11, Duration::from_millis(80)),
                Frame::new(12, Duration::from_millis(80)),
                Frame::new(13, Duration::from_millis(80)),
                Frame::new(14, Duration::from_millis(80)),
                Frame::new(15, Duration::from_millis(80)),
                Frame::new(16, Duration::from_millis(80)),
                Frame::new(17, Duration::from_millis(80)),
            ]),
        }
    }

    pub(crate) fn set(&mut self, next: JeanCurrentAnim) {
        self.playing = next;

        // Reset the animation
        let animation = match self.playing {
            JeanCurrentAnim::IdleRight => &mut self.idle_right,
            JeanCurrentAnim::IdleLeft => &mut self.idle_left,
            JeanCurrentAnim::WalkRight => &mut self.walk_right,
            JeanCurrentAnim::WalkLeft => &mut self.walk_left,
        };

        animation.reset();
    }

    pub(crate) fn playing(&self) -> JeanCurrentAnim {
        self.playing
    }

    pub(crate) fn to_idle(&self) -> JeanCurrentAnim {
        match self.playing() {
            JeanCurrentAnim::IdleLeft => JeanCurrentAnim::IdleLeft,
            JeanCurrentAnim::IdleRight => JeanCurrentAnim::IdleRight,
            JeanCurrentAnim::WalkLeft => JeanCurrentAnim::IdleLeft,
            JeanCurrentAnim::WalkRight => JeanCurrentAnim::IdleRight,
        }
    }

    pub(crate) fn to_walking(&self) -> JeanCurrentAnim {
        match self.playing() {
            JeanCurrentAnim::IdleLeft => JeanCurrentAnim::WalkLeft,
            JeanCurrentAnim::IdleRight => JeanCurrentAnim::WalkRight,
            JeanCurrentAnim::WalkLeft => JeanCurrentAnim::WalkLeft,
            JeanCurrentAnim::WalkRight => JeanCurrentAnim::WalkRight,
        }
    }
}

impl Animated for JeanAnims {
    fn animate(&mut self) -> usize {
        match self.playing {
            JeanCurrentAnim::IdleRight => self.idle_right.update(),
            JeanCurrentAnim::IdleLeft => self.idle_left.update(),
            JeanCurrentAnim::WalkRight => self.walk_right.update(),
            JeanCurrentAnim::WalkLeft => self.walk_left.update(),
        }
    }
}

impl BlobAnims {
    pub(crate) fn new(playing: BlobCurrentAnim) -> Self {
        Self {
            playing,
            idle_right: Animation::new(vec![Frame::new(0, Duration::from_secs(1))]),
            idle_left: Animation::new(vec![Frame::new(8, Duration::from_secs(1))]),
            bounce_right: Animation::new(vec![
                Frame::new(1, Duration::from_millis(80)),
                Frame::new(2, Duration::from_millis(80)),
                Frame::new(3, Duration::from_millis(80)),
                Frame::new(4, Duration::from_millis(80)),
                Frame::new(5, Duration::from_millis(80)),
                Frame::new(6, Duration::from_millis(80)),
                Frame::new(7, Duration::from_millis(120)),
            ]),
            bounce_left: Animation::new(vec![
                Frame::new(9, Duration::from_millis(80)),
                Frame::new(10, Duration::from_millis(80)),
                Frame::new(11, Duration::from_millis(80)),
                Frame::new(12, Duration::from_millis(80)),
                Frame::new(13, Duration::from_millis(80)),
                Frame::new(14, Duration::from_millis(80)),
                Frame::new(15, Duration::from_millis(120)),
            ]),
        }
    }

    pub(crate) fn set(&mut self, next: BlobCurrentAnim) {
        self.playing = next;

        // Reset the animation
        let animation = match self.playing {
            BlobCurrentAnim::IdleRight => &mut self.idle_right,
            BlobCurrentAnim::IdleLeft => &mut self.idle_left,
            BlobCurrentAnim::BounceRight => &mut self.bounce_right,
            BlobCurrentAnim::BounceLeft => &mut self.bounce_left,
        };

        animation.reset();
    }

    pub(crate) fn playing(&self) -> BlobCurrentAnim {
        self.playing
    }

    pub(crate) fn get_frame_index(&self) -> usize {
        match self.playing {
            BlobCurrentAnim::IdleRight => self.idle_right.get_frame().index,
            BlobCurrentAnim::IdleLeft => self.idle_left.get_frame().index,
            BlobCurrentAnim::BounceRight => self.bounce_right.get_frame().index,
            BlobCurrentAnim::BounceLeft => self.bounce_left.get_frame().index,
        }
    }
}

impl Animated for BlobAnims {
    fn animate(&mut self) -> usize {
        match self.playing {
            BlobCurrentAnim::IdleRight => self.idle_right.update(),
            BlobCurrentAnim::IdleLeft => self.idle_left.update(),
            BlobCurrentAnim::BounceRight => {
                let last_frame_index = self.bounce_right.get_frame().index;
                let frame_index = self.bounce_right.update();

                if last_frame_index == 7 && frame_index == 1 {
                    self.set(BlobCurrentAnim::IdleRight);
                    self.get_frame_index()
                } else {
                    frame_index
                }
            }
            BlobCurrentAnim::BounceLeft => {
                let last_frame_index = self.bounce_left.get_frame().index;
                let frame_index = self.bounce_left.update();

                if last_frame_index == 15 && frame_index == 9 {
                    self.set(BlobCurrentAnim::IdleLeft);
                    self.get_frame_index()
                } else {
                    frame_index
                }
            }
        }
    }
}

impl BlobCurrentAnim {
    pub(crate) fn new(random: &mut PCG32, direction: Option<&PropertyValue>) -> Self {
        direction
            .map(|prop| match prop {
                PropertyValue::StringValue(direction) => match direction.as_str() {
                    "left" => Some(BlobCurrentAnim::IdleLeft),
                    "right" => Some(BlobCurrentAnim::IdleRight),
                    _ => None,
                },
                _ => None,
            })
            .flatten()
            .unwrap_or_else(|| {
                // TODO: Produce a random direction
                if random.next_u32() & 1 == 0 {
                    BlobCurrentAnim::IdleLeft
                } else {
                    BlobCurrentAnim::IdleRight
                }
            })
    }
}

impl FireAnims {
    pub(crate) fn new(random: &mut PCG32) -> Self {
        let mut burn = Animation::new(vec![
            Frame::new(0, Duration::from_millis(30)),
            Frame::new(1, Duration::from_millis(40)),
            Frame::new(2, Duration::from_millis(30)),
            Frame::new(3, Duration::from_millis(50)),
            Frame::new(4, Duration::from_millis(35)),
            Frame::new(5, Duration::from_millis(40)),
        ]);
        burn.current_index = random.next_u32() as usize % burn.frames.len();

        Self { burn }
    }
}

impl Animated for FireAnims {
    fn animate(&mut self) -> usize {
        self.burn.update()
    }
}
