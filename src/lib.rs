use gdnative::prelude::*;

mod scene_main;

fn init(handle: InitHandle) {
    handle.add_class::<scene_main::SceneMain>();
}

godot_init!(init);
