use float_cmp::ApproxEqUlps;
use nalgebra::{self as na, Vector2, Isometry2};
use ncollide::bounding_volume::{AABB2, BoundingVolume, HasBoundingVolume};
use ncollide::shape::*;

use consts::*;
use entities::{Dimensions, EntityID, Entity};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentType {
    Player,
    Ball
}


pub trait PhysicsComponent {
    fn entity_id(&self) -> EntityID;

    fn component_type(&self) -> ComponentType;

    fn update(&mut self, entity: &mut Entity, field: Dimensions);

    fn bounding_volume(&self, entity: &Entity) -> AABB2<f64>;
}


pub struct BallPhysics {
    id: EntityID
}

impl BallPhysics {
    pub fn new(id: EntityID) -> BallPhysics {
        BallPhysics {
            id: id
        }
    }
}

impl PhysicsComponent for BallPhysics {
    fn entity_id(&self) -> EntityID {
        self.id
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Ball
    }

    fn update(&mut self, entity: &mut Entity, field: Dimensions) {
        let angle_rad = entity.orientation.to_radians();

        let dx = entity.velocity * angle_rad.cos();
        let dy = entity.velocity * angle_rad.sin();

        entity.position.x += dx;
        entity.position.y += dy;

        // Get bounding volume for the playing field
        let shape_field = Cuboid::new(Vector2::new(field.width as f64 / 2.0, field.height as f64 / 2.0));
        let pos_field = Isometry2::new(Vector2::new(field.width as f64 / 2.0, field.height as f64 / 2.0), na::zero());
        let bounds_field: AABB2<_> = shape_field.bounding_volume(&pos_field);

        // Get bounding volume for the ball
        let radius_ball = entity.dimensions.width / 2.0;
        let bounds_ball = self.bounding_volume(&entity);

        if !bounds_field.contains(&bounds_ball) {
            // Angel of entry = angle of reflection
            entity.orientation *= -1.0;

            // If the ball hits the left or right wall, let it come back
            if entity.position.x - radius_ball <= 0.0 || entity.position.x + radius_ball >= field.width as f64 {
                entity.orientation += 180.0;
            }
        }

        // Normalize angle
        entity.orientation %= 360.0;
        while entity.orientation <= -180.0 {
            entity.orientation += 180.0;
        }
        while entity.orientation >= 180.0 {
            entity.orientation -= 180.0;
        }
    }

    fn bounding_volume(&self, entity: &Entity) -> AABB2<f64> {
        let radius = entity.dimensions.width / 2.0;
        let shape = Ball2::new(radius);
        let trans = Isometry2::new(Vector2::new(entity.position.x, entity.position.y), na::zero());
        shape.bounding_volume(&trans)
    }
}


pub struct PlayerPhysics {
    id: EntityID,
    t: u64,
    last_orientation: f64
}

impl PlayerPhysics {
    pub fn new(id: EntityID) -> PlayerPhysics {
        PlayerPhysics {
            id: id,
            t: 0,
            last_orientation: 0.0
        }
    }
}

impl PhysicsComponent for PlayerPhysics {
    fn entity_id(&self) -> EntityID {
        self.id
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Player
    }

    fn update(&mut self, entity: &mut Entity, field: Dimensions) {
        if entity.velocity == 0.0 || entity.orientation.approx_ne_ulps(&self.last_orientation, 2) {
            self.t = 0;
        }

        // Limited growth for speed
        let inertial_velocity = PLAYER_VELOCITY - PLAYER_VELOCITY * (-PLAYER_INERTIA * self.t as f64).exp();

        self.t += 1;
        self.last_orientation = entity.orientation;

        let dy = if entity.orientation.approx_eq_ulps(&90.0, 2) {
            inertial_velocity
        } else {
            -inertial_velocity
        };

        // Clip player to playing field
        entity.position.y = (entity.position.y + dy)
            .max(entity.dimensions.height / 2.0 - 1.0)
            .min(field.height - entity.dimensions.height / 2.0);
    }

    fn bounding_volume(&self, entity: &Entity) -> AABB2<f64> {
        let shape = Cuboid::new(Vector2::new(entity.dimensions.width as f64 / 2.0, entity.dimensions.height as f64 / 2.0));
        let trans = Isometry2::new(Vector2::new(entity.position.x, entity.position.y), na::zero());
        shape.bounding_volume(&trans)
    }
}