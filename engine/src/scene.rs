use bevy_ecs::{prelude::*, query};

use crate::{Behavior, NativeScriptComponent};

pub struct Scene {
    uuid: u128,
    registry: World,
    on_create_schedule: Schedule,
    on_update_schedule: Schedule,
}

struct DeltaTime(f32);

fn create_native_script(mut query: Query<&mut NativeScriptComponent>) {
    query.for_each_mut(|mut component| component.instance.as_mut().unwrap().on_create());
}

fn tick_native_script(res: Res<DeltaTime>, mut query: Query<&mut NativeScriptComponent>) {
    query.for_each_mut(|mut component| component.instance.as_mut().unwrap().on_tick(res.0));
}

impl Scene {
    pub fn new() -> Scene {
        let mut s = Self {
            uuid: uuid::Uuid::new_v4().as_u128(),
            registry: World::new(),
            on_create_schedule: Schedule::default().with_stage(
                "create",
                SystemStage::single_threaded().with_system(create_native_script),
            ),
            on_update_schedule: Schedule::default().with_stage(
                "tick",
                SystemStage::single_threaded().with_system(tick_native_script),
            ),
        };
        s.registry.insert_resource(DeltaTime(16.6));
        return s;
    }
    pub fn create_entity(&mut self) -> Entity {
        self.registry.spawn().id()
    }
    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        self.registry
            .get_entity_mut(entity)
            .unwrap()
            .insert(component);
    }

    pub fn get_component<T: Component>(&mut self, entity: Entity) -> Option<&T> {
        self.registry.get::<T>(entity)
    }
    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<Mut<T>> {
        self.registry.get_mut::<T>(entity)
    }
    pub fn on_create(&mut self){
        self.on_create_schedule.run_once(&mut self.registry);
    }
    pub fn on_tick(&mut self){
        self.on_update_schedule.run_once(&mut self.registry);
    }
}
