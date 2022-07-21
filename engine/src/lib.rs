#[macro_use]
extern crate macro_rules_attribute;

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use scene::Scene;
use serde::{Deserialize, Serialize};
pub mod scene;

#[derive(Serialize, Deserialize, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Default, Component)]
pub struct Transform {
    pub position: Vec3f,
    pub rotation: Vec3f,
    pub scale: Vec3f,
}

#[derive(Serialize, Deserialize, Default, Component)]
pub struct Tag {
    pub value: String,
}

pub trait Behavior: 'static + Send + Sync {
    fn on_tick(&mut self, delta_time: f32);
    fn on_create(&mut self);
    fn on_destroy(&mut self);
}

pub trait BehaviorTraits: Default + Serialize + Deserialize<'static> + Copy + Clone {}
//impl<T> BehaviorTraits for T where T: Default + Serialize + Deserialize<'static> + Copy + Clone {}

#[derive(Component)]
pub struct NativeScriptComponent {
    pub instance: Option<Box<dyn Behavior>>,
    pub instantiate: fn() -> Box<dyn Behavior>,
    //pub to_yaml: fn(&NativeScriptComponent) -> serde_yaml::Value,
}

impl NativeScriptComponent {
    pub fn new<T: Behavior + Default + Serialize + Deserialize<'static> + Copy + Clone>() -> Self {
        let build = Self {
            instance: None,
            instantiate: || Box::new(T::default()),
            //to_yaml: |_comp| serde_yaml::to_value("temp").unwrap(),
        };
        // build.to_yaml = |comp| unsafe {
        //     let val = std::ptr::addr_of!(comp.instance);
        //     let val1 = val.read();
        //     let raw = Box::<dyn Behavior>::into_raw(val1);
        //     let inst: *const T = raw as _;
        //     serde_yaml::to_value::<T>(*inst).unwrap()
        // };
        return build;
    }
}

pub fn register_class<T: Behavior + Default + Serialize + Deserialize<'static> + Copy + Clone>(
    name: String,
    map: &mut Box<HashMap<String, NativeScriptComponent>>,
) {
    let component = NativeScriptComponent::new::<T>();
    map.insert(name.clone(), component);
    println!("Registered {}", name.as_str());
}

// derive_alias! {
//     #[derive(BehaviorTraits!)] = #[derive(Default + Serialize + Deserialize<'static> + Copy + Clone)];
// }

macro_rules! derive_behavior {
    ($i:item) => {
        #[derive(Default, Serialize, Deserialize, Copy, Clone, BehaviorTraits)]
        $i
    };
}
pub(crate) use derive_behavior;
