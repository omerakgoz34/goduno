use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Main;

#[methods]
impl Main {
    fn new(_owner: &Node) -> Self {
        return Main;
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("I'm from the script of the \"main\" scene.");
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Main>();
}

godot_init!(init);
