#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Right,
    DownRight,
    DownLeft,
    Left,
    UpLeft,
    UpRight,
}

impl Into<u32> for Direction {
    fn into(self) -> u32 {
        match self {
            Direction::Right => 0,
            Direction::DownRight => 1,
            Direction::DownLeft => 2,
            Direction::Left => 3,
            Direction::UpLeft => 4,
            Direction::UpRight => 5,
        }
    }
}

impl TryFrom<u32> for Direction {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, ()> {
        match value {
            0 => Ok(Direction::Right),
            1 => Ok(Direction::DownRight),
            2 => Ok(Direction::DownLeft),
            3 => Ok(Direction::Left),
            4 => Ok(Direction::UpLeft),
            5 => Ok(Direction::UpRight),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn translate(&self, direction: Direction) -> Self {
        match direction {
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::DownRight => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::DownLeft => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::UpLeft => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::UpRight => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
        }
    }
}

pub type AntId = usize;
pub type InstrIdx = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    Move,
    Rotate { direction: Direction },
    DropFood,
    PickUpFood,
}
