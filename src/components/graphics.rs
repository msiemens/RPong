use graphics;
use opengl_graphics::GlGraphics;

use entities::Entity;


pub trait GraphicsComponent {
    fn render(&self, entity: &Entity, ctx: &graphics::Context, gl: &mut GlGraphics);

    fn bounding_box(&self, entity: &Entity) -> graphics::types::Rectangle {
        // [x, y, w, h]
        [entity.position.x - entity.dimensions.width / 2.0,
            entity.position.y - entity.dimensions.height / 2.0,
            entity.dimensions.width,
            entity.dimensions.height]
    }
}


pub struct BallGraphics;

impl GraphicsComponent for BallGraphics {
    fn render(&self, entity: &Entity, ctx: &graphics::Context, gl: &mut GlGraphics) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let e = graphics::ellipse::Ellipse::new_border(WHITE, 0.5).color(BLACK);
        e.draw(self.bounding_box(entity), &Default::default(), ctx.transform, gl);
    }
}


pub struct StickGraphics;

impl GraphicsComponent for StickGraphics {
    fn render(&self, entity: &Entity, ctx: &graphics::Context, gl: &mut GlGraphics) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let e = graphics::rectangle::Rectangle::new_border(WHITE, 0.5).color(BLACK);
        e.draw(self.bounding_box(entity), &Default::default(), ctx.transform, gl);
    }
}