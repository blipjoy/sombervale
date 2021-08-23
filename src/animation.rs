use std::time::{Duration, Instant};

pub(crate) struct Frame {
    pub(crate) index: usize,
    pub(crate) duration: Duration,
}

pub(crate) struct Animation {
    pub(crate) frames: Vec<Frame>,
    pub(crate) current_index: usize,
    pub(crate) start_time: Instant,
}

impl Animation {
    pub(crate) fn new(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            current_index: 0,
            start_time: Instant::now(),
        }
    }

    pub(crate) fn get_frame(&self) -> &Frame {
        &self.frames[self.current_index % self.frames.len()]
    }

    pub(crate) fn update(&mut self, dur: Duration) -> Option<usize> {
        if self.start_time.elapsed() > dur {
            self.current_index += 1;
            self.current_index %= self.frames.len();
            self.start_time = Instant::now();

            let index = self.frames[self.current_index].index;

            Some(index)
        } else {
            None
        }
    }
}

impl Frame {
    pub(crate) fn new(index: usize, duration: Duration) -> Self {
        Self { index, duration }
    }
}

pub(crate) struct FrogAnims {
    pub(crate) idle_right: Animation,
    pub(crate) idle_left: Animation,
    pub(crate) hop_right: Animation,
    pub(crate) hop_left: Animation,
}

pub(crate) enum FrogCurrentAnim {
    IdleRight,
    IdleLeft,
    HopRight,
    HopLeft,
}

pub(crate) struct JeanAnims {
    pub(crate) idle_right: Animation,
    pub(crate) idle_left: Animation,
    pub(crate) walk_right: Animation,
    pub(crate) walk_left: Animation,
}

pub(crate) enum JeanCurrentAnim {
    IdleRight,
    IdleLeft,
    WalkRight,
    WalkLeft,
}

impl FrogAnims {
    pub(crate) fn new() -> Self {
        Self {
            idle_right: Animation::new(vec![Frame::new(0, Duration::from_secs(1))]),
            idle_left: Animation::new(vec![Frame::new(5, Duration::from_secs(1))]),
            hop_right: Animation::new(vec![
                Frame::new(0, Duration::from_millis(100)),
                Frame::new(1, Duration::from_millis(100)),
                Frame::new(2, Duration::from_millis(100)),
                Frame::new(3, Duration::from_millis(100)),
                Frame::new(4, Duration::from_millis(140)),
            ]),
            hop_left: Animation::new(vec![
                Frame::new(5, Duration::from_millis(100)),
                Frame::new(6, Duration::from_millis(100)),
                Frame::new(7, Duration::from_millis(100)),
                Frame::new(8, Duration::from_millis(100)),
                Frame::new(9, Duration::from_millis(140)),
            ]),
        }
    }
}

impl JeanAnims {
    pub(crate) fn new() -> Self {
        Self {
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
}
