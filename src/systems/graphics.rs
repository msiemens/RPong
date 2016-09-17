use std::collections::HashMap;

use graphics;
use opengl_graphics::GlGraphics;

use components::graphics::GraphicsComponent;
use entities::{EntityID, Entity};


pub struct GraphicsSystem {
    components: HashMap<EntityID, Box<GraphicsComponent>>
}

impl GraphicsSystem {
    pub fn new() -> GraphicsSystem {
        GraphicsSystem {
            components: HashMap::new()
        }
    }

    pub fn register_component(&mut self, id: EntityID, component: Box<GraphicsComponent>) {
        self.components.insert(id, component);
    }

    pub fn render(&mut self, world: &HashMap<EntityID, Entity>, ctx: &graphics::Context, gl: &mut GlGraphics) {
        for (id, component) in &mut self.components {
            component.render(world.get(id).unwrap(), ctx, gl);
        }
    }
}