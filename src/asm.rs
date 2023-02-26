use crate::data::*;
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
    pub fn eval(self, world: &World, ant_id: AntId) -> (Option<Action>, InstrIdx) {
        match self {
            Instr::Turn {
                direction,
                next_instr,
            } => {
                let action = Action::Rotate {
                    direction: direction.apply_to(world.ant(ant_id).direction),
                };
                (Some(action), next_instr)
            }
            Instr::Move {
                success_instr,
                fail_instr,
            } => {
                if world.can_move(ant_id) {
                    (Some(Action::Move), success_instr)
                } else {
                    (None, fail_instr)
                }
            }
            Instr::Direction {
                direction,
                success_instr,
                fail_instr,
            } => {
                let ant = world.ant(ant_id);
                let next_instr = if ant.direction == direction {
                    success_instr
                } else {
                    fail_instr
                };
                (None, next_instr)
            }
            Instr::PickUpFood {
                success_instr,
                fail_instr,
            } => {
                if world.can_pickup_food(ant_id) {
                    (Some(Action::PickUpFood), success_instr)
                } else {
                    (None, fail_instr)
                }
            }
            Instr::DropFood { next_instr } => {
                if world.can_drop_food(ant_id) {
                    (Some(Action::DropFood), next_instr)
                } else {
                    (None, next_instr)
                }
            }
        }
    }
}
