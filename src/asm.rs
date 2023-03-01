use crate::world::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    pub fn apply_to(self, direction: Direction) -> Direction {
        let d = match self {
            TurnDirection::Left => -1,
            TurnDirection::Right => 1,
        };
        let direction: u32 = direction.into();
        let direction = direction as i32;
        let direction = (direction + d + 6) % 6;
        let direction = direction as u32;
        Direction::try_from(direction).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    Turn {
        direction: TurnDirection,
        next_instr: InstrIdx,
    },
    Move {
        success_instr: InstrIdx,
        fail_instr: InstrIdx,
    },
    Direction {
        direction: Direction,
        success_instr: InstrIdx,
        fail_instr: InstrIdx,
    },
    PickUpFood {
        success_instr: InstrIdx,
        fail_instr: InstrIdx,
    },
    DropFood {
        next_instr: InstrIdx,
    },
}

impl Instr {
    pub fn eval(self, mut ant: AntMut) {
        let next_instr = match self {
            Instr::Turn {
                direction,
                next_instr,
            } => {
                let new_direction = direction.apply_to(ant.direction());
                ant.rotate(new_direction);
                next_instr
            }
            Instr::Move {
                success_instr,
                fail_instr,
            } => {
                if ant.move_forward().is_ok() {
                    success_instr
                } else {
                    fail_instr
                }
            }
            Instr::Direction {
                direction,
                success_instr,
                fail_instr,
            } => {
                if ant.direction() == direction {
                    success_instr
                } else {
                    fail_instr
                }
            }
            Instr::PickUpFood {
                success_instr,
                fail_instr,
            } => {
                if ant.pickup_food().is_ok() {
                    success_instr
                } else {
                    fail_instr
                }
            }
            Instr::DropFood { next_instr } => {
                let _ = ant.drop_food();
                next_instr
            }
        };
        ant.update_instr_pointer(next_instr);
    }
}

pub type Program = Vec<Instr>;
