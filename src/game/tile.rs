use glium::backend::Facade;

use crate::drawable::Drawable;

#[derive(Clone, Copy)]
pub enum Entity {
    NoEntity,
    Apple,
    Snake
}

#[derive(Clone, Copy)]
pub struct Tile {
    x: i32,
    y: i32,
    entity: Entity
}

impl Tile {
    pub fn new(x: i32, y: i32, entity: Entity) -> Tile {
        Tile {
            x,
            y,
            entity
        }
    }
}

pub fn draw_tiles(display: &dyn Facade, tiles: &Vec<Vec<Tile>>) -> Vec<Drawable> {
    let mut drawables: Vec<Drawable> = Vec::new();

    #[allow(non_snake_case)]
    for tArray in tiles.iter() {
        for tile in tArray.iter() {
            let posX = tile.x * 80;
            let posY = tile.y * 80;

            match &tile.entity {
                Entity::NoEntity => {
                    drawables.push(Drawable::new(display, (posX, posY), (0.2, 0.2, 0.2)));
                },
                Entity::Apple => {
                    drawables.push(Drawable::new(display, (posX, posY), (1.0, 0.0, 0.0)));
                },
                Entity::Snake => {
                    drawables.push(Drawable::new(display, (posX, posY), (0.0, 1.0, 0.0)));
                }
            }
        }
    };

    drawables
}