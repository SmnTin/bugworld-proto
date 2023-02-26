use crate::data::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ant {
    pub color: Color,
    pub direction: Direction,
    pub position: Position,
    pub instr_pointer: InstrIdx,
    pub carries_food: bool,
}

impl Ant {
    pub fn new(color: Color, position: Position) -> Self {
        Ant {
            color,
            position,
            direction: Direction::default(),
            instr_pointer: 0,
            carries_food: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Wall,
    FreeCell { ant_id: Option<AntId>, food: u32 },
}

impl Default for Cell {
    fn default() -> Self {
        Cell::FreeCell {
            ant_id: None,
            food: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CellError {
    Occupied,
    Wall,
    NoFood,
}

impl Cell {
    pub fn clear_ant(&mut self) -> Option<AntId> {
        if let Cell::FreeCell { ref mut ant_id, .. } = self {
            let old_ant_id = *ant_id;
            *ant_id = None;
            old_ant_id
        } else {
            None
        }
    }

    pub fn try_put_ant(&mut self, ant_id: AntId) -> Result<(), CellError> {
        match self {
            Cell::Wall => Err(CellError::Wall),

            Cell::FreeCell {
                ant_id: Some(_), ..
            } => Err(CellError::Occupied),

            Cell::FreeCell {
                ant_id: ref mut ant_id_ref,
                ..
            } => {
                *ant_id_ref = Some(ant_id);
                Ok(())
            }
        }
    }

    pub fn ant(&self) -> Option<AntId> {
        match self {
            Cell::FreeCell {
                ant_id: Some(ant_id),
                ..
            } => Some(*ant_id),
            _ => None,
        }
    }

    pub fn has_ant(&self) -> bool {
        self.ant().is_some()
    }

    pub fn food(&self) -> u32 {
        match self {
            Cell::FreeCell { food, .. } => *food,
            _ => 0,
        }
    }

    pub fn has_food(&self) -> bool {
        self.food() > 0
    }

    pub fn try_pickup_food(&mut self) -> Result<(), CellError> {
        match self {
            Cell::Wall => Err(CellError::Wall),

            Cell::FreeCell {
                food: ref mut food_ref,
                ..
            } => {
                if *food_ref > 0 {
                    *food_ref -= 1;
                    Ok(())
                } else {
                    Err(CellError::NoFood)
                }
            }
        }
    }

    pub fn try_drop_food(&mut self) -> Result<(), CellError> {
        match self {
            Cell::Wall => Err(CellError::Wall),

            Cell::FreeCell {
                food: ref mut food_ref,
                ..
            } => {
                *food_ref += 1;
                Ok(())
            }
        }
    }

    pub fn free_to_move(&self) -> bool {
        match self {
            Cell::Wall => false,
            Cell::FreeCell { ant_id, .. } => ant_id.is_none(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let row = vec![Cell::default(); width];
        let cells = vec![row; height];
        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cell_at(&self, position: Position) -> Option<&Cell> {
        if !self.in_bounds(position) {
            return None;
        }
        Some(&self.cells[position.y as usize][position.x as usize])
    }

    pub fn cell_at_mut(&mut self, position: Position) -> Option<&mut Cell> {
        if !self.in_bounds(position) {
            return None;
        }
        Some(&mut self.cells[position.y as usize][position.x as usize])
    }

    pub fn ant_at(&self, position: Position) -> Option<AntId> {
        self.cell_at(position).and_then(|cell| match cell {
            Cell::FreeCell {
                ant_id: Some(ant_id),
                ..
            } => Some(*ant_id),
            _ => None,
        })
    }

    pub fn in_bounds(&self, position: Position) -> bool {
        position.y >= 0
            && position.x >= 0
            && position.y < self.height as i32
            && position.x < self.width as i32
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct World {
    ants: Vec<Ant>,
    swarms: HashMap<Color, Vec<AntId>>,
    grid: Grid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldError {
    OutOfBounds,
    Wall,
    Occupied,
    CellHasNoFood,
    AntHasNoFood,
    AntCarriesFood,
}

impl From<CellError> for WorldError {
    fn from(value: CellError) -> Self {
        match value {
            CellError::Wall => WorldError::Wall,
            CellError::Occupied => WorldError::Occupied,
            CellError::NoFood => WorldError::CellHasNoFood,
        }
    }
}

impl World {
    pub fn new(grid: Grid) -> Self {
        let mut swarms = HashMap::new();
        swarms.insert(Color::Black, Vec::new());
        swarms.insert(Color::Red, Vec::new());

        World {
            ants: Vec::new(),
            swarms,
            grid,
        }
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn swarm(&self, color: Color) -> impl Iterator<Item = AntId> + '_ {
        self.swarms.get(&color).unwrap().iter().copied()
    }

    fn swarm_mut(&mut self, color: Color) -> &mut Vec<AntId> {
        self.swarms.get_mut(&color).unwrap()
    }

    pub fn add_ant(&mut self, ant: Ant) -> Result<AntId, WorldError> {
        let id = self.ants.len();
        let cell = self
            .grid
            .cell_at_mut(ant.position)
            .ok_or(WorldError::OutOfBounds)?;
        cell.try_put_ant(id)?;
        self.ants.push(ant);
        self.swarm_mut(ant.color).push(id);
        Ok(id)
    }

    pub fn ant(&self, id: AntId) -> Ant {
        self.ants[id]
    }

    fn ant_mut(&mut self, id: AntId) -> &mut Ant {
        &mut self.ants[id]
    }

    pub fn ants(&self) -> impl Iterator<Item = Ant> + '_ {
        self.ants.iter().copied()
    }

    pub fn cell_of(&self, id: AntId) -> &Cell {
        self.grid.cell_at(self.ant(id).position).unwrap()
    }

    pub fn can_move(&self, id: AntId) -> bool {
        let ant = self.ant(id);
        let new_position = ant.position.translate(ant.direction);
        self.grid
            .cell_at(new_position)
            .map_or(false, Cell::free_to_move)
    }

    pub fn move_ant(&mut self, id: AntId) -> Result<(), WorldError> {
        let ant = self.ants.get_mut(id).unwrap();
        let new_position = ant.position.translate(ant.direction);
        let new_cell = self
            .grid
            .cell_at_mut(new_position)
            .ok_or(WorldError::OutOfBounds)?;
        new_cell.try_put_ant(id)?;
        let old_cell = self.grid.cell_at_mut(ant.position).unwrap();
        old_cell.clear_ant();
        ant.position = new_position;
        Ok(())
    }

    pub fn rotate_ant(&mut self, id: AntId, direction: Direction) {
        self.ant_mut(id).direction = direction;
    }

    pub fn can_pickup_food(&self, id: AntId) -> bool {
        let ant = self.ant(id);
        let cell = self.cell_of(id);
        !ant.carries_food && cell.has_food()
    }

    pub fn pickup_food(&mut self, id: AntId) -> Result<(), WorldError> {
        let ant = &mut self.ants[id];
        let cell = self.grid.cell_at_mut(ant.position).unwrap();
        if ant.carries_food {
            return Err(WorldError::AntCarriesFood);
        }
        cell.try_pickup_food()?;
        ant.carries_food = true;
        Ok(())
    }

    pub fn can_drop_food(&self, id: AntId) -> bool {
        let ant = self.ant(id);
        ant.carries_food
    }

    pub fn drop_food(&mut self, id: AntId) -> Result<(), WorldError> {
        let ant = &mut self.ants[id];
        if !ant.carries_food {
            return Err(WorldError::AntHasNoFood);
        }
        ant.carries_food = false;
        let cell = self.grid.cell_at_mut(ant.position).unwrap();
        cell.try_drop_food().unwrap();
        Ok(())
    }

    pub fn apply(&mut self, id: AntId, action: Action) -> Result<(), WorldError> {
        match action {
            Action::Move => self.move_ant(id),
            Action::Rotate { direction } => {
                self.rotate_ant(id, direction);
                Ok(())
            }
            Action::PickUpFood => self.pickup_food(id),
            Action::DropFood => self.drop_food(id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cell {
        use super::*;

        #[test]
        fn put_ant_twice() {
            let mut cell = Cell::default();
            assert_eq!(cell.try_put_ant(0), Ok(()));
            assert_eq!(cell.try_put_ant(1), Err(CellError::Occupied));
        }

        #[test]
        fn put_ant_into_wall() {
            let mut cell = Cell::Wall;
            assert_eq!(cell.try_put_ant(0), Err(CellError::Wall));
        }

        #[test]
        fn clear_wall() {
            let mut cell = Cell::Wall;
            assert_eq!(cell.clear_ant(), None);
        }

        #[test]
        fn clear() {
            let mut cell = Cell::default();
            cell.try_put_ant(0).unwrap();
            assert_eq!(cell.clear_ant(), Some(0));
            assert_eq!(cell.clear_ant(), None);
        }

        #[test]
        fn has_ant() {
            let mut cell = Cell::default();
            assert_eq!(cell.has_ant(), false);
            cell.try_put_ant(0).unwrap();
            assert_eq!(cell.has_ant(), true);
            cell.clear_ant();
            assert_eq!(cell.has_ant(), false);
        }

        #[test]
        fn ant() {
            let mut cell = Cell::default();
            assert_eq!(cell.ant(), None);
            cell.try_put_ant(0).unwrap();
            assert_eq!(cell.ant(), Some(0));
            cell.clear_ant();
            assert_eq!(cell.ant(), None);
        }

        #[test]
        fn pickup_food() {
            let mut cell = Cell::default();
            assert_eq!(cell.try_pickup_food(), Err(CellError::NoFood));
            cell.try_drop_food().unwrap();
            assert_eq!(cell.try_pickup_food(), Ok(()));
            assert_eq!(cell.try_pickup_food(), Err(CellError::NoFood));
        }

        #[test]
        fn has_food() {
            let mut cell = Cell::default();
            assert_eq!(cell.has_food(), false);
            cell.try_drop_food().unwrap();
            assert_eq!(cell.has_food(), true);
            cell.try_pickup_food().unwrap();
            assert_eq!(cell.has_food(), false);
        }
    }

    mod grid {
        use super::*;

        #[test]
        fn new() {
            let grid = Grid::new(10, 15);
            assert_eq!(grid.width(), 10);
            assert_eq!(grid.height(), 15);
        }

        #[test]
        fn cell_at_mutate() {
            let mut grid = Grid::new(10, 10);
            let pos = Position { x: 0, y: 0 };
            let new_cell = Cell::FreeCell {
                ant_id: None,
                food: 5,
            };

            let cell = grid.cell_at_mut(pos).unwrap();
            *cell = new_cell.clone();
            assert_eq!(grid.cell_at(pos), Some(&new_cell));
        }

        #[test]
        fn cell_at() {
            let grid = Grid::new(10, 10);
            assert_eq!(
                grid.cell_at(Position { x: 0, y: 5 }),
                Some(&Cell::default())
            );
            assert_eq!(grid.cell_at(Position { x: -1, y: 0 }), None);
        }

        #[test]
        fn cell_at_mut() {
            let mut grid = Grid::new(10, 10);
            assert_eq!(
                grid.cell_at_mut(Position { x: 0, y: 5 }),
                Some(&mut Cell::default())
            );
            assert_eq!(grid.cell_at_mut(Position { x: -1, y: 0 }), None);
        }

        #[test]
        fn ant_at() {
            let mut grid = Grid::new(10, 10);
            let pos = Position { x: 5, y: 5 };
            assert_eq!(grid.ant_at(pos), None);
            grid.cell_at_mut(pos).unwrap().try_put_ant(0).unwrap();
            assert_eq!(grid.ant_at(pos), Some(0));
        }

        #[test]
        fn in_bounds() {
            let grid = Grid::new(10, 15);
            assert_eq!(grid.in_bounds(Position { x: 0, y: 0 }), true);
            assert_eq!(grid.in_bounds(Position { x: 9, y: 14 }), true);
            assert_eq!(grid.in_bounds(Position { x: 8, y: 15 }), false);
            assert_eq!(grid.in_bounds(Position { x: 10, y: 9 }), false);
            assert_eq!(grid.in_bounds(Position { x: 0, y: -1 }), false);
            assert_eq!(grid.in_bounds(Position { x: -1, y: 0 }), false);
            assert_eq!(grid.in_bounds(Position { x: -4, y: -4 }), false);
        }
    }

    mod world {
        use super::*;

        #[test]
        fn new() {
            let world = World::new(Grid::new(10, 15));
            assert_eq!(world.grid().width(), 10);
            assert_eq!(world.grid().height(), 15);
            assert_eq!(world.swarm(Color::Red).next(), None);
            assert_eq!(world.swarm(Color::Black).next(), None);
        }

        #[test]
        fn add_ant() {
            let mut world = World::new(Grid::new(10, 15));

            let pos = Position { x: 5, y: 5 };
            let ant = Ant::new(Color::Red, pos);

            let add_result = world.add_ant(ant);
            assert!(add_result.is_ok());
            let id = add_result.unwrap();

            assert_eq!(world.ant(id), ant);
            assert_eq!(world.swarm(Color::Red).next(), Some(id));
            assert_eq!(world.swarm(Color::Black).next(), None);
            assert_eq!(world.grid().ant_at(pos), Some(id));
        }

        #[test]
        fn add_ant_into_wall() {
            let blocked_pos = Position { x: 6, y: 7 };
            let mut grid = Grid::new(10, 15);
            *grid.cell_at_mut(blocked_pos).unwrap() = Cell::Wall;

            let ant = Ant::new(Color::Red, blocked_pos);
            let mut world = World::new(grid);
            assert_eq!(world.add_ant(ant), Err(WorldError::Wall));
        }

        #[test]
        fn add_ant_into_occupied() {
            let mut world = World::new(Grid::new(10, 15));

            let pos = Position { x: 6, y: 7 };
            let ant = Ant::new(Color::Red, pos);
            assert!(world.add_ant(ant).is_ok());
            assert_eq!(world.add_ant(ant), Err(WorldError::Occupied));
        }

        #[test]
        fn rotate_ant() {
            let mut world = World::new(Grid::new(10, 15));

            let ant = Ant::new(Color::Red, Position { x: 6, y: 7 });
            let id = world.add_ant(ant).unwrap();
            world.rotate_ant(id, Direction::DownRight);

            assert_eq!(world.ant(id).direction, Direction::DownRight);
        }

        #[test]
        fn move_ant_ok() {
            let mut world = World::new(Grid::new(10, 15));

            let pos = Position { x: 6, y: 7 };
            let new_pos = pos.translate(Direction::default());

            let ant = Ant::new(Color::Red, pos);
            let id = world.add_ant(ant).unwrap();

            assert!(world.can_move(id));
            assert!(world.move_ant(id).is_ok());
            assert_eq!(world.grid().ant_at(pos), None);
            assert_eq!(world.grid().ant_at(new_pos), Some(id));
            assert_eq!(world.ant(id).position, new_pos);
        }

        #[test]
        fn move_ant_out_of_bounds() {
            let mut world = World::new(Grid::new(10, 15));

            let ant = Ant::new(Color::Red, Position { x: 9, y: 7 });
            let id = world.add_ant(ant).unwrap();

            assert!(!world.can_move(id));
            assert_eq!(world.move_ant(id), Err(WorldError::OutOfBounds));
        }

        #[test]
        fn move_ant_into_occupied() {
            let mut world = World::new(Grid::new(10, 15));

            let pos = Position { x: 6, y: 7 };
            let new_pos = pos.translate(Direction::default());

            let ant = Ant::new(Color::Red, pos);
            let another_ant = Ant::new(Color::Red, new_pos);
            let id = world.add_ant(ant).unwrap();
            world.add_ant(another_ant).unwrap();

            assert!(!world.can_move(id));
            assert_eq!(world.move_ant(id), Err(WorldError::Occupied));
        }

        #[test]
        fn move_ant_into_wall() {
            let mut grid = Grid::new(10, 15);
            let pos = Position { x: 6, y: 7 };
            let new_pos = pos.translate(Direction::default());
            *grid.cell_at_mut(new_pos).unwrap() = Cell::Wall;

            let ant = Ant::new(Color::Red, pos);
            let mut world = World::new(grid);
            let id = world.add_ant(ant).unwrap();

            assert!(!world.can_move(id));
            assert_eq!(world.move_ant(id), Err(WorldError::Wall));
        }

        #[test]
        fn pickup_food() {
            let mut grid = Grid::new(10, 15);
            let pos = Position { x: 6, y: 7 };
            *grid.cell_at_mut(pos).unwrap() = Cell::FreeCell {
                ant_id: None,
                food: 5,
            };

            let ant = Ant::new(Color::Red, pos);
            let mut world = World::new(grid);
            let id = world.add_ant(ant).unwrap();

            assert!(world.can_pickup_food(id));
            assert_eq!(world.pickup_food(id), Ok(()));
            assert!(world.ant(id).carries_food);
            assert_eq!(world.grid().cell_at(pos).unwrap().food(), 4);
            assert!(!world.can_pickup_food(id));
            assert_eq!(world.pickup_food(id), Err(WorldError::AntCarriesFood));
        }

        #[test]
        fn pickup_food_from_empty_cell() {
            let mut grid = Grid::new(10, 15);
            let pos = Position { x: 6, y: 7 };
            *grid.cell_at_mut(pos).unwrap() = Cell::FreeCell {
                ant_id: None,
                food: 0,
            };

            let ant = Ant::new(Color::Red, pos);
            let mut world = World::new(grid);
            let id = world.add_ant(ant).unwrap();

            assert!(!world.can_pickup_food(id));
            assert_eq!(world.pickup_food(id), Err(WorldError::CellHasNoFood));
        }

        #[test]
        fn drop_food() {
            let mut grid = Grid::new(10, 15);
            let pos = Position { x: 6, y: 7 };
            *grid.cell_at_mut(pos).unwrap() = Cell::FreeCell {
                ant_id: None,
                food: 5,
            };

            let ant = Ant::new(Color::Red, pos);
            let mut world = World::new(grid);
            let id = world.add_ant(ant).unwrap();
            world.pickup_food(id).unwrap();

            assert!(world.can_drop_food(id));
            assert_eq!(world.drop_food(id), Ok(()));
            assert!(!world.ant(id).carries_food);
            assert_eq!(world.grid().cell_at(pos).unwrap().food(), 5);
            assert!(!world.can_drop_food(id));
            assert_eq!(world.drop_food(id), Err(WorldError::AntHasNoFood));
        }
    }
}
