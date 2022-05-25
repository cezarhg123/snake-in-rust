use self::tile::{Tile, Entity};

pub mod tile;

#[derive(PartialEq)]
pub enum Input {
    NoInput,
    Up,
    Down,
    Right,
    Left
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    pub fn value(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0)
        }
    }

    pub fn opposite_value(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::West => (1, 0),
            Direction::East => (-1, 0)
        }
    }
}

pub struct Game {
    pub tiles: Vec<Vec<Tile>>,
    pub crnt_direction: Direction,
    pub tail: (i32, i32, Direction),
    pub head: (i32, i32, Direction),
    movements: Vec<Direction>
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            tiles: Vec::new(),
            crnt_direction: Direction::East,
            tail: (1, 4, Direction::East),
            head: (3, 4, Direction::East),
            movements: Vec::new()
        };

        for x in 0..10 {
            let mut temp_vec: Vec<Tile> = Vec::new();
            for y in 0..10 {
                temp_vec.push(Tile::new(x, y, Entity::NoEntity));
            }
            game.tiles.push(temp_vec);
        };
        
        //"spawn" snake
        game.tiles[1][4].entity = Entity::Snake;
        game.tiles[2][4].entity = Entity::Snake;
        game.tiles[3][4].entity = Entity::Snake;

        //"spawn" apple
        game.tiles[5][4].entity = Entity::Apple;

        game
    }

    pub fn change_direction(&mut self, input: &mut Input) {
        match input {
            Input::Up => {
                if self.crnt_direction != Direction::South {
                    self.crnt_direction = Direction::North;
                    self.movements.push(Direction::North);
                }
                *input = Input::NoInput;
            },
            Input::Down => {
                if self.crnt_direction != Direction::North {
                    self.crnt_direction = Direction::South;
                    self.movements.push(Direction::South);
                }
                *input = Input::NoInput;
            },
            Input::Left => {
                if self.crnt_direction != Direction::East {
                    self.crnt_direction = Direction::West;
                    self.movements.push(Direction::West);
                }
                *input = Input::NoInput;
            },
            Input::Right => {
                if self.crnt_direction != Direction::West {
                    self.crnt_direction = Direction::East;
                    self.movements.push(Direction::East);
                }
                *input = Input::NoInput;
            },
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        self.head.0 += self.crnt_direction.value().0;
        self.head.1 += self.crnt_direction.value().1;
        self.head.2 = self.crnt_direction;

        {   //HEAD MECHANICS
            let x: &usize = &self.head.0.try_into().unwrap();
            let y: &usize = &self.head.1.try_into().unwrap();
            self.tiles[*x][*y] = Tile::new(self.head.0, self.head.1, Entity::Snake); 
        }

        {   //TAIL MECHANICS
            {
                let x: usize = (self.tail.0 + self.tail.2.value().0).try_into().unwrap();
                let y: usize = (self.tail.1 + self.tail.2.value().1).try_into().unwrap();

                if self.tiles[x][y].entity == Entity::NoEntity {
                    self.tail.2 = *self.movements.get(0).unwrap();
                    self.movements.pop().unwrap();
                }
            }

            self.tail.0 += self.tail.2.value().0;
            self.tail.1 += self.tail.2.value().1;

            let x: usize = (self.tail.0 + self.tail.2.opposite_value().0).try_into().unwrap();
            let y: usize = (self.tail.1 + self.tail.2.opposite_value().1).try_into().unwrap();
            self.tiles[x][y] = Tile::new(x.try_into().unwrap(), y.try_into().unwrap(), Entity::NoEntity);
        }
    }
}