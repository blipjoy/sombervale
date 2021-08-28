use crate::component::{Annihilate, Controls, Hud, Intro, Random, UpdateTime};
use crate::map::add_tilemap;
use crate::power::FrogPower;
use shipyard::AllStoragesViewMut;

pub(crate) fn load_world(storages: AllStoragesViewMut) {
    storages.add_unique(Intro());
    storages.add_unique(Random::default());
    storages.add_unique(UpdateTime::default());
    storages.add_unique(Controls::default());
    storages.add_unique(Annihilate(Vec::new()));

    let hud = Hud {
        frog_power: Some(FrogPower::default()),
    };
    storages.add_unique(hud);

    add_tilemap(storages, include_str!("../assets/tilemap.tmx"));
}
