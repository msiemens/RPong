use piston::input::Key;

use entities::Entity;


#[derive(Clone, Copy, Eq, PartialEq)]
pub enum InputType {
    Pressed,
    Released
}


pub struct PlayerInput {
    up_key: Key,
    down_key: Key,
    velocity: f64
}

impl PlayerInput {
    pub fn new(up_key: Key, down_key: Key, velocity: f64) -> PlayerInput {
        PlayerInput {
            up_key: up_key,
            down_key: down_key,
            velocity: velocity
        }
    }

    pub fn update(&self, entity: &mut Entity, key: Key, dir: InputType) {
        if dir == InputType::Released && (key == self.up_key || key == self.down_key) {
            entity.velocity = 0.0;
        } else if key == self.up_key {
            entity.velocity = self.velocity;
            entity.orientation = -90.0;
        } else if key == self.down_key {
            entity.velocity = self.velocity;
            entity.orientation = 90.0;
        }
    }
}
