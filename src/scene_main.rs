use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SceneMain;

#[methods]
impl SceneMain {
    fn new(_owner: &Node) -> Self {
        return SceneMain;
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("I'm from the script of the \"main\" scene.");
    }
}
