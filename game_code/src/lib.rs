#[macro_use]
extern crate engine;

use std::collections::HashMap;

use engine::{Behavior, BehaviorTraits};
use serde::{Deserialize, Serialize};
// derive_alias! {
//     #[derive(BehaviorTraits!)] = #[derive(Default + Serialize + Deserialize<'static> + Copy + Clone)];
// }
#[derive(Default, Serialize, Deserialize, Copy, Clone)]
pub struct Camera {
    pub yaw: f32,
    pub roll: f32,
    pub pitch: f32,
}

impl Behavior for Camera {
    fn on_tick(&mut self, delta_time: f32) {
        println!("Camera::on_tick novo, delta: {}", delta_time);
    }
    fn on_create(&mut self) {
        println!("Camera::on_create");
    }
    fn on_destroy(&mut self) {
        println!("Camera::on_destroy");
    }
}
#[derive(Default, Serialize, Deserialize, Copy, Clone)]
pub struct Coisa {
    pub health: u32,
    pub mana: u32,
}

impl Behavior for Coisa {
    fn on_tick(&mut self, delta_time: f32) {
        println!("Coisa::on_tick, delta: {}", delta_time);
    }

    fn on_create(&mut self) {
        println!("Coisa::on_create");
    }

    fn on_destroy(&mut self) {
        println!("Coisa::on_destroy");
    }
}

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
pub struct Player {
    pub health: u32,
    pub mana: u32,
}

impl Behavior for Player {
    fn on_tick(&mut self, delta_time: f32) {
        println!("Player::on_tick, delta: {}", delta_time);
    }

    fn on_create(&mut self) {
        println!("Player::on_create");
    }

    fn on_destroy(&mut self) {
        println!("Player::on_destroy");
    }
}

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
pub struct OutraCoisa {
    pub health: u32,
    pub mana: u32,
}

impl Behavior for OutraCoisa {
    fn on_tick(&mut self, delta_time: f32) {
        println!("OutraCoisa::on_tick, delta: {}", delta_time);
    }

    fn on_create(&mut self) {
        println!("OutraCoisa::on_create");
    }

    fn on_destroy(&mut self) {
        println!("OutraCoisa::on_destroy");
    }
}

#[no_mangle]
pub extern "C" fn game_main(map: &mut Box<HashMap<String, engine::NativeScriptComponent>>) {
    engine::register_class::<Coisa>(String::from("Coisa"), map);
    engine::register_class::<Camera>(String::from("Camera"), map);
    engine::register_class::<Player>(String::from("Player"), map);
    engine::register_class::<Player>(String::from("OutraCoisa"), map);
}

#[no_mangle]
extern "C" fn double(a: i32) -> i32 {
    return a * 2;
}
