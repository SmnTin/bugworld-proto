use std::collections::HashMap;

use crate::asm::*;
use crate::world::*;

pub trait Renderer {
    fn render(&mut self, world: &World);
}

pub struct Simulator {
    world: World,
    programs: HashMap<Color, Program>,
    renderer: Box<dyn Renderer>,
}

impl Simulator {
    pub fn new(
        world: World,
        programs: HashMap<Color, Program>,
        renderer: Box<dyn Renderer>,
    ) -> Self {
        Self {
            world,
            programs,
            renderer,
        }
    }

    pub fn step(&mut self) {
        self.step_brains();
        self.renderer.render(&self.world);
    }

    fn step_brains(&mut self) {
        let ants = self.world.ant_ids();
        for ant_id in ants {
            self.step_single_brain(ant_id);
        }
    }

    fn step_single_brain(&mut self, ant_id: AntId) {
        let ant = self.world.ant_mut(ant_id);
        let program = self.programs.get(&ant.color()).unwrap();
        program[ant.instr_pointer()].eval(ant);
    }
}
