use std::cell::Cell;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntityID(pub i64);


#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}


#[derive(Clone, Copy, Debug)]
pub struct Entity {
    // Center point!
    pub position: Position,
    pub dimensions: Dimensions,
    pub orientation: f64,
    pub velocity: f64,
    id: EntityID,
}

impl Entity {
    pub fn new(pos: Position, dim: Dimensions, orientation: f64, velocity: f64) -> Entity {
        thread_local! {
            static NEXT_ID: Cell<i64> = Cell::new(0);
        }

        NEXT_ID.with(|id| {
            let instance = Entity {
                position: pos,
                dimensions: dim,
                orientation: orientation,
                velocity: velocity,
                id: EntityID(id.get()),
            };

            id.set(id.get() + 1);

            instance
        })
    }

    pub fn id(&self) -> EntityID {
        self.id
    }
}