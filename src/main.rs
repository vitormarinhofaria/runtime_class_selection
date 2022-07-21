// use std::{
//     any::Any,
//     collections::{BTreeMap, HashMap},
//     hash::Hash,
//     sync::*,
// };

// use bevy_ecs::prelude::*;
// use ron::ser::PrettyConfig;
// use serde::{Deserialize, Serialize};

// fn on_create_behavior(mut query: Query<&'static mut NativeScriptComponent>) {
//     query.for_each_mut(|mut b| {
//         b.instance = (b.instantiate)();
//         b.instance.as_mut().on_create();
//     });
// }
// fn on_tick_system(mut query: Query<&'static mut NativeScriptComponent>) {
//     query.for_each_mut(|mut b| {
//         b.instance.as_mut().on_tick(15f32);
//     });
// }
// // fn save_world(scene: Res<std::fs::File>, query: Query<&mut NativeScriptComponent>) {
// //     query.for_each(|ns| {
// //         serde_yaml::to_writer(scene.as_ref(), &ns.instance.as_ref().unwrap().serialize()).unwrap();
// //     });
// // }
// fn main() {
//     let mut world = World::new();
//     {
//         let script_component = NativeScriptComponent::new::<Camera>();
//         world.spawn().insert_bundle((
//             script_component,
//             components::Transform::default(),
//             components::Tag {
//                 value: String::from("My Camera"),
//             },
//         ));
//     }

//     for arg in std::env::args().skip(1) {
//         match arg.as_str() {
//             "Camera" => {
//                 let script_component = NativeScriptComponent::new::<Camera>();
//                 let random = rand::random::<u32>();
//                 world.spawn().insert_bundle((
//                     script_component,
//                     components::Transform {
//                         position: components::Vec3f {
//                             x: 10f32,
//                             y: 15f32,
//                             z: 20f32,
//                         },
//                         rotation: components::Vec3f {
//                             x: 10f32,
//                             y: 15f32,
//                             z: 20f32,
//                         },
//                         scale: components::Vec3f {
//                             x: 1f32,
//                             y: 1f32,
//                             z: 1f32,
//                         },
//                     },
//                     components::Tag {
//                         value: format!("Camera {}", random),
//                     },
//                 ));
//             }
//             "Coisa" => {
//                 let script_component = NativeScriptComponent::new::<Coisa>();
//                 let random = rand::random::<u32>();
//                 world.spawn().insert_bundle((
//                     script_component,
//                     components::Transform {
//                         position: components::Vec3f {
//                             x: 25f32,
//                             y: 30f32,
//                             z: 35f32,
//                         },
//                         rotation: components::Vec3f {
//                             x: 25f32,
//                             y: 30f32,
//                             z: 35f32,
//                         },
//                         scale: components::Vec3f {
//                             x: 1f32,
//                             y: 1f32,
//                             z: 1f32,
//                         },
//                     },
//                     components::Tag {
//                         value: format!("Coisa {}", random),
//                     },
//                 ));
//             }
//             _ => (),
//         }
//     }
//     let mut on_create_schedule = Schedule::default();
//     on_create_schedule.add_stage(
//         "create",
//         SystemStage::parallel().with_system(on_create_behavior),
//     );
//     on_create_schedule.run_once(&mut world);
//     let mut on_update_schedule = Schedule::default();
//     on_update_schedule.add_stage(
//         "update",
//         SystemStage::parallel().with_system(on_tick_system),
//     );

//     world.insert_resource(std::fs::File::create("scene.yaml").unwrap());
//     on_update_schedule.run_once(&mut world);
//     // loop {
//     //     on_update_schedule.run(&mut world);
//     //     std::thread::sleep(std::time::Duration::from_secs(1));
//     // }

//     let mut yaml = Scene {
//         entities: Vec::new(),
//         name: String::from("My Scene"),
//     };

//     for i in 0..world.entities().len() {
//         let ent = world.get_entity(Entity::from_raw(i)).unwrap();
//         let mut smap = serde_yaml::Mapping::new();

//         smap.insert(
//             serde_yaml::to_value("Entity").unwrap(),
//             serde_yaml::to_value(rand::random::<u64>()).unwrap(),
//         );

//         let tag_component = ent.get::<components::Tag>();
//         if let Some(tag) = tag_component {
//             smap.insert(
//                 serde_yaml::to_value("Tag").unwrap(),
//                 serde_yaml::to_value(tag).unwrap(),
//             );
//         }

//         let transform_component = ent.get::<components::Transform>();
//         if let Some(transform) = transform_component {
//             smap.insert(
//                 serde_yaml::to_value("Transform").unwrap(),
//                 serde_yaml::to_value(transform).unwrap(),
//             );
//         }

//         let native_script_comp = ent.get::<NativeScriptComponent>();
//         if let Some(comp) = native_script_comp {
//             smap.insert(
//                 serde_yaml::to_value("NativeScriptComponent").unwrap(),
//                 //comp.instance.as_ref().unwrap().to_yaml(),
//                 (comp.to_yaml)(comp),
//             );
//         }

//         yaml.entities.push(smap);
//     }

//     serde_yaml::to_writer(
//         std::fs::File::create("scene.yaml").unwrap(),
//         &serde_yaml::to_value(yaml).unwrap(),
//     )
//     .expect("Failed to save scene to file");
// }

use std::{
    collections::HashMap,
    io::{Read, Write},
};

use engine::NativeScriptComponent;
use libloading::Symbol;

// #[derive(Serialize, Deserialize)]
// struct Scene {
//     pub name: String,
//     pub entities: Vec<serde_yaml::Mapping>,
// }
use windows::Win32::System::LibraryLoader::*;
use windows::{core::PCSTR, Win32::Foundation::*};
struct GameDll {
    pub lib: HINSTANCE ,
    //pub lib: Option<libloading::Library>, //pub init: Option<Symbol<'a, unsafe extern "C" fn(&mut HashMap<String, NativeScriptComponent>)>>
}
impl GameDll {
    pub fn load(name: String, map: &mut Box<HashMap<String, NativeScriptComponent>>) -> GameDll {
        unsafe {
            let gdll = GameDll {
                lib: LoadLibraryA(PCSTR(b"game_code.dll\0".as_ptr())).unwrap(),
            };
            let game_main = GetProcAddress(gdll.lib, PCSTR(b"game_main\0".as_ptr())).expect("faio procaddr");
            let function: extern "C" fn(&mut Box<HashMap<String, NativeScriptComponent>>) = std::mem::transmute(game_main);
            function(map);
            return gdll;
        }
    }
}
impl Drop for GameDll {
    fn drop(&mut self) {
        println!("Calling drop");
        unsafe {
            FreeLibrary(self.lib);
        }
    }
}

fn main() {
    println!(
        "workdir: {}",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let mut map = Box::new(HashMap::<String, NativeScriptComponent>::new());
    let mut s1 = engine::scene::Scene::new();
    let mut dll = GameDll::load("game_code.dll".to_string(), &mut map);
    //load_dll_func(&mut map);

    for arg in map.as_ref(){
        println!("{}", arg.0);
    }

    s1.on_create();
    'r: loop {
        let mut buf = String::new();
        std::io::stdout().write_all(b"Command: \n").unwrap();
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut buf).unwrap();

        println!("Read: {:?}", buf.as_bytes());
        println!("expect: {:?}", b"reload");
        std::io::stdout().flush().unwrap();
        match &buf.as_bytes()[0..buf.len() - 2] {
            b"reload" => {
                std::mem::drop(dll);
                std::io::stdout().write_all(b"Dll name: \n").unwrap();
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut buf).unwrap();
                //map = HashMap::new();
                map.clear();
                dll = GameDll::load("game_code.dll".to_string(), &mut map);
                //load_dll_func(&mut map);
            }
            b"quit" => break 'r,
            _ => s1.on_tick(),
        }
    }
}
