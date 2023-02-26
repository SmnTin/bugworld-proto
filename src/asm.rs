use crate::data::*;

pub enum TurnDirection {
    Left,
    Right,
}

pub enum Instr {
    Turn {
        direction: TurnDirection,
        next_instr: InstrIdx,
    },
    Move {
        success_instr: InstrIdx,
        fail_instr: InstrIdx,
    },
    Flip {
        range: usize,
        success_instr: InstrIdx,
        fail_instr: InstrIdx,
    },
    Direction {
        direction: Direction,
        success_instr: InstrIdx,
        fail_instr: InstrIdx,
    },
}
