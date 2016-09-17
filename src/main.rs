// TODO: Fix collision detection bugs
// TODO: Winner detection
// TODO: Start on [Space]
// TODO: Restartable game
// TODO: Pause when losing focus
// TODO: Points counter
// TODO: AI
// TODO: Code Cleanup


extern crate float_cmp;
extern crate glutin_window;
extern crate graphics;
extern crate nalgebra;
extern crate ncollide;
extern crate piston;
extern crate rand;
extern crate opengl_graphics;

use rand::Rng;
use std::collections::HashMap;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use consts::*;
use components::input::InputType;
use entities::{Dimensions, EntityID, Entity, Position};
use systems::graphics::GraphicsSystem;
use systems::input::InputSystem;
use systems::physics::PhysicsSystem;

mod consts;
mod components;
mod entities;
mod systems;

pub struct App {
    world: HashMap<EntityID, Entity>,
    graphics: GraphicsSystem,
    input: InputSystem,
    physics: PhysicsSystem,
    field: Dimensions
}

impl App {
    fn new(field: Dimensions) -> App {
        let mut graphics = GraphicsSystem::new();
        let mut input = InputSystem::new();
        let mut physics = PhysicsSystem::new();
        let mut world = HashMap::new();

        // Add the ball
        let mut rng = rand::thread_rng();
        let initial_angle = if rng.gen() {
            rng.gen_range(-135.0, 135.0)
        } else {
            rng.gen_range(-45.0, 45.0)
        };
        let ball_x = field.width as f64 / 2.0 - BALL_RADIUS / 2.0;
        let ball_y = field.height as f64 / 2.0 - BALL_RADIUS / 2.0;

        let entity = Entity::new(Position { x: ball_x, y: ball_y },
                                 Dimensions { width: BALL_RADIUS, height: BALL_RADIUS },
                                 initial_angle,
                                 BALL_VELOCITY);

        physics.register_component(entity.id(), Box::new(components::physics::BallPhysics::new(entity.id())));
        graphics.register_component(entity.id(), Box::new(components::graphics::BallGraphics));
        world.insert(entity.id(), entity);

        // Add Player 1
        let player1_x = PLAYER_WALL_DISTANCE;
        let player1_y = field.height / 2.0;

        let entity = Entity::new(Position { x: player1_x, y: player1_y },
                                 Dimensions { width: PLAYER_WIDTH, height: PLAYER_HEIGHT },
                                 0.0,
                                 0.0);
        graphics.register_component(entity.id(), Box::new(components::graphics::StickGraphics));
        physics.register_component(entity.id(), Box::new(components::physics::PlayerPhysics::new(entity.id())));
        input.register_component(entity.id(), components::input::PlayerInput::new(Key::W, Key::S, PLAYER_VELOCITY));
        world.insert(entity.id(), entity);

        // Add player 2
        let player2_x = field.width - PLAYER_WALL_DISTANCE;
        let player2_y = field.height / 2.0;

        let entity = Entity::new(Position { x: player2_x, y: player2_y },
                                 Dimensions { width: PLAYER_WIDTH, height: PLAYER_HEIGHT },
                                 0.0,
                                 0.0);

        graphics.register_component(entity.id(), Box::new(components::graphics::StickGraphics));
        physics.register_component(entity.id(), Box::new(components::physics::PlayerPhysics::new(entity.id())));
        input.register_component(entity.id(), components::input::PlayerInput::new(Key::Up, Key::Down, PLAYER_VELOCITY));
        world.insert(entity.id(), entity);

        App {
            world: world,
            graphics: graphics,
            input: input,
            physics: physics,
            field: field,
        }
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::Graphics;

        gl.clear_color([0.0, 0.0, 0.0, 0.0]);

        gl.draw(args.viewport(), |ctx, gl| {
            self.graphics.render(&self.world, &ctx, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let field = self.field;
        self.physics.update(&mut self.world, field);
    }

    fn input(&mut self, key: Key, dir: InputType) {
        self.input.update(&mut self.world, key, dir);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let size = [800, 600];
    let mut window: Window = WindowSettings::new(
        "RPong",
        size
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut gl = GlGraphics::new(opengl);
    let mut app = App::new(Dimensions { width: size[0] as f64, height: size[1] as f64 });

    let mut events = window.events();
    while let Some(event) = events.next(&mut window) {
        // TODO: When losing focus, pause the game

        // Process inputs
        if let Some(Button::Keyboard(key)) = event.press_args() {
            app.input(key, InputType::Pressed);
        };

        if let Some(Button::Keyboard(key)) = event.release_args() {
            app.input(key, InputType::Released);
        };

        // Process render events
        if let Some(render_args) = event.render_args() {
            app.render(&mut gl, &render_args);
        }

        // Process game update events
        if let Some(update_args) = event.update_args() {
            app.update(&update_args);
        }
    }
}