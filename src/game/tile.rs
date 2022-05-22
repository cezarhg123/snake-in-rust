enum Entity {
    None,
    Apple,
    Snake
}

pub struct Tile {
    x: i32,
    y: i32,
    entity: Entity
}