use std::collections::HashMap;

use piston::input::Key;

use components::input::{InputType, PlayerInput};
use entities::{EntityID, Entity};


pub struct InputSystem {
    components: HashMap<EntityID, PlayerInput>
}

impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem {
            components: HashMap::new()
        }
    }

    pub fn register_component(&mut self, id: EntityID, component: PlayerInput) {
        self.components.insert(id, component);
    }

    pub fn update(&mut self, world: &mut HashMap<EntityID, Entity>, key: Key, dir: InputType) {
        for (id, component) in &mut self.components {
            component.update(world.get_mut(id).unwrap(), key, dir);
        }
    }
}