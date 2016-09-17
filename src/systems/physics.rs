use std::collections::HashMap;

use ncollide::bounding_volume::BoundingVolume;

use consts::*;
use components::physics::{ComponentType, PhysicsComponent};
use entities::{Dimensions, EntityID, Entity};


pub struct PhysicsSystem {
    components: HashMap<EntityID, Box<PhysicsComponent>>
}

impl PhysicsSystem {
    pub fn new() -> PhysicsSystem {
        PhysicsSystem {
            components: HashMap::new()
        }
    }

    pub fn register_component(&mut self, id: EntityID, component: Box<PhysicsComponent>) {
        self.components.insert(id, component);
    }

    pub fn update(&mut self, world: &mut HashMap<EntityID, Entity>, field: Dimensions) {
        // Detect inter-entity collisions
        for (i, (id1, component1)) in self.components.iter().enumerate() {
            for (id2, component2) in self.components.iter().skip(i + 1) {
                assert!(id1 != id2);

                let intersect = {
                    let entity1 = world.get(id1).unwrap();
                    let entity2 = world.get(id2).unwrap();

                    self.components_intersect(entity1, &**component1, entity2, &**component2)
                };

                if intersect {
                    match (component1.component_type(), component2.component_type()) {
                        (ComponentType::Player, ComponentType::Ball) => {
                            let player = world.get(id1).unwrap().clone();
                            let ball = world.get_mut(id2).unwrap();
                            self.handle_collision(&player, ball);
                        },
                        (ComponentType::Ball, ComponentType::Player) => {
                            let player = world.get(id2).unwrap().clone();
                            let ball = world.get_mut(id1).unwrap();
                            self.handle_collision(&player, ball);
                        },
                        _ => panic!("Impossible collision")
                    }
                }
            }
        }

        // Update each entity
        for (id, component) in &mut self.components {
            component.update(world.get_mut(id).unwrap(), field);
        }
    }

    fn components_intersect(&self,
                            entity1: &Entity,
                            component1: &PhysicsComponent,
                            entity2: &Entity,
                            component2: &PhysicsComponent) -> bool {
        let bounding1 = component1.bounding_volume(entity1);
        let bounding2 = component2.bounding_volume(entity2);

        bounding1.loosened(BALL_VELOCITY).intersects(&bounding2)
    }

    fn handle_collision(&self,
                        player: &Entity,
                        ball: &mut Entity) {
        assert!(player.id() != ball.id());

        let ball_radius = ball.dimensions.width / 2.0;

        if ball.position.y - ball_radius > player.position.y + player.dimensions.height / 2.0 {
            // Ball hits top of player
            println!("Player top hit");

            ball.orientation *= -1.0;
        } else if ball.position.y + ball_radius < player.position.y - player.dimensions.height / 2.0 {
            // Ball hits bottom of player
            println!("Player bottom hit");

            ball.orientation *= -1.0;
        } else {
            // Ball hits player's stick's face
            println!("Player face hit");

            let angle_factor = (player.position.y - ball.position.y) / (player.dimensions.height / 2.0);
            let new_angle = angle_factor * MAX_RETURN_ANGLE;

            if ball.orientation.abs() < 90.0 {
                // Inverse the direction
                ball.orientation = 180.0 - new_angle;
            } else {
                ball.orientation = -new_angle;
            }

            if BALL_SPEEDUP_ENABLED {
                ball.velocity = BALL_VELOCITY + player.velocity * BALL_MAX_SPEEDUP;
            }
        }
    }
}