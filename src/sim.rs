use std::collections::HashMap;

use crate::asm::*;
use crate::world::*;

pub trait Renderer {
    fn render(&mut self, world: &World);
}

struct Interpreter {
    program: Program,
    color: Color,
}

impl Interpreter {
    fn step_brains(&mut self, world: &mut World) {
        let ant_ids: Vec<_> = world.swarm_ids(self.color).collect();
        for ant_id in ant_ids {
            let mut ant = world.ant_mut(ant_id);
            let instr = self.program[ant.instr_pointer()];
            let next_instr = instr.eval(&mut ant);
            ant.update_instr_pointer(next_instr)
        }
    }
}

pub struct Simulator {
    world: World,
    interpreters: Vec<Interpreter>,
    renderer: Box<dyn Renderer>,
}

impl Simulator {
    pub fn new(
        world: World,
        programs: HashMap<Color, Program>,
        renderer: Box<dyn Renderer>,
    ) -> Self {
        let interpreters = programs
            .into_iter()
            .map(|(color, program)| Interpreter { program, color })
            .collect();
        Self {
            world,
            interpreters,
            renderer,
        }
    }

    pub fn step(&mut self) {
        for interpreter in &mut self.interpreters {
            interpreter.step_brains(&mut self.world);
        }
        self.renderer.render(&self.world);
    }
}
