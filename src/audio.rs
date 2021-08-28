use anyhow::Result;
use kira::instance::InstanceSettings;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::{self, handle::SoundHandle, SoundSettings};
use std::io::Cursor;

pub(crate) struct Player {
    _manager: AudioManager,
    sounds: Sounds,
}

struct Sounds {
    music: SoundHandle,
    jump: SoundHandle,
    splat: SoundHandle,
}

impl Player {
    pub(crate) fn new() -> Result<Self> {
        let mut manager = AudioManager::new(AudioManagerSettings::default())?;

        let music = Cursor::new(include_bytes!(
            "../cc0/01_-_A.T.M.O.M._-_Nochnoe_Dykhanie_Taigi.ogg"
        ));
        let sound = sound::Sound::from_ogg_reader(music, SoundSettings::default())?;
        let music = manager.add_sound(sound)?;

        let jump = Cursor::new(include_bytes!("../assets/jump.ogg"));
        let sound = sound::Sound::from_ogg_reader(jump, SoundSettings::default())?;
        let jump = manager.add_sound(sound)?;

        let splat = Cursor::new(include_bytes!("../assets/splat.ogg"));
        let sound = sound::Sound::from_ogg_reader(splat, SoundSettings::default())?;
        let splat = manager.add_sound(sound)?;

        let sounds = Sounds { music, jump, splat };
        let player = Self {
            _manager: manager,
            sounds,
        };

        Ok(player)
    }

    pub(crate) fn music(&mut self) -> Result<()> {
        self.sounds.music.play(InstanceSettings::default())?;

        Ok(())
    }

    pub(crate) fn jump(&mut self) {
        self.sounds.jump.play(InstanceSettings::default()).ok();
    }

    pub(crate) fn splat(&mut self) {
        self.sounds.splat.play(InstanceSettings::default()).ok();
    }
}
