use anyhow::Result;
use kira::instance::InstanceSettings;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::{self, handle::SoundHandle, SoundSettings};
use std::io::Cursor;

pub(crate) struct Player {
    manager: AudioManager,
    sounds: Sounds,
}

struct Sounds {
    music: SoundHandle,
}

impl Player {
    pub(crate) fn new() -> Result<Self> {
        let mut manager = AudioManager::new(AudioManagerSettings::default())?;

        let music = Cursor::new(include_bytes!(
            "../cc0/01_-_A.T.M.O.M._-_Nochnoe_Dykhanie_Taigi.ogg"
        ));
        let sound = sound::Sound::from_ogg_reader(music, SoundSettings::default())?;
        let music = manager.add_sound(sound)?;

        let sounds = Sounds { music };
        let player = Self { manager, sounds };

        Ok(player)
    }

    pub(crate) fn play_music(&mut self) -> Result<()> {
        self.sounds.music.play(InstanceSettings::default())?;

        Ok(())
    }
}
