use std::collections::HashMap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AntData {
    color: Color,
    direction: Direction,
    position: Position,
    instr_pointer: InstrIdx,
    carries_food: bool,
}

impl AntData {
    fn new(color: Color, position: Position) -> Self {
        AntData {
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
    ants: Vec<AntData>,
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

#[derive(Debug, Clone, Copy)]
pub struct Ant<'a> {
    id: AntId,

    data: &'a AntData,
}

impl Ant<'_> {
    pub fn id(&self) -> AntId {
        self.id
    }

    pub fn position(&self) -> Position {
        self.data.position
    }

    pub fn direction(&self) -> Direction {
        self.data.direction
    }

    pub fn color(&self) -> Color {
        self.data.color
    }

    pub fn carries_food(&self) -> bool {
        self.data.carries_food
    }

    pub fn instr_pointer(&self) -> usize {
        self.data.instr_pointer
    }
}

impl PartialEq for Ant<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Ant<'_> {}

pub struct AntMut<'a> {
    id: AntId,

    grid: &'a mut Grid,
    data: &'a mut AntData,
}

impl<'a> AntMut<'a> {
    pub fn id(&self) -> AntId {
        self.id
    }

    pub fn position(&self) -> Position {
        self.data.position
    }

    pub fn direction(&self) -> Direction {
        self.data.direction
    }

    pub fn color(&self) -> Color {
        self.data.color
    }

    pub fn carries_food(&self) -> bool {
        self.data.carries_food
    }

    pub fn instr_pointer(&self) -> usize {
        self.data.instr_pointer
    }

    pub fn move_forward(&mut self) -> Result<(), WorldError> {
        let new_position = self.data.position.translate(self.data.direction);
        let new_cell = self
            .grid
            .cell_at_mut(new_position)
            .ok_or(WorldError::OutOfBounds)?;
        new_cell.try_put_ant(self.id)?;
        let old_cell = self.grid.cell_at_mut(self.data.position).unwrap();
        old_cell.clear_ant();
        self.data.position = new_position;
        Ok(())
    }

    pub fn rotate(&mut self, direction: Direction) {
        self.data.direction = direction;
    }

    pub fn pickup_food(&mut self) -> Result<(), WorldError> {
        let cell = self.grid.cell_at_mut(self.data.position).unwrap();
        if self.data.carries_food {
            return Err(WorldError::AntCarriesFood);
        }
        cell.try_pickup_food()?;
        self.data.carries_food = true;
        Ok(())
    }

    pub fn drop_food(&mut self) -> Result<(), WorldError> {
        if !self.data.carries_food {
            return Err(WorldError::AntHasNoFood);
        }
        self.data.carries_food = false;
        let cell = self.grid.cell_at_mut(self.data.position).unwrap();
        cell.try_drop_food().unwrap();
        Ok(())
    }

    pub fn update_instr_pointer(&mut self, new_pointer: usize) {
        self.data.instr_pointer = new_pointer;
    }
}

impl PartialEq for AntMut<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for AntMut<'_> {}

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

    pub fn swarm(&self, color: Color) -> impl Iterator<Item = Ant<'_>> {
        self.swarm_ids(color).map(|id| self.ant(id))
    }

    pub fn swarm_ids(&self, color: Color) -> impl Iterator<Item = AntId> + '_ {
        self.swarms.get(&color).unwrap().iter().copied()
    }

    fn swarm_mut(&mut self, color: Color) -> &mut Vec<AntId> {
        self.swarms.get_mut(&color).unwrap()
    }

    pub fn add_ant(&mut self, color: Color, position: Position) -> Result<AntId, WorldError> {
        let id = self.ants.len();
        let cell = self
            .grid
            .cell_at_mut(position)
            .ok_or(WorldError::OutOfBounds)?;
        cell.try_put_ant(id)?;
        self.ants.push(AntData::new(color, position));
        self.swarm_mut(color).push(id);
        Ok(id)
    }

    pub fn ant(&self, id: AntId) -> Ant<'_> {
        Ant {
            id,
            data: &self.ants[id],
        }
    }

    pub fn ant_mut(&mut self, id: AntId) -> AntMut<'_> {
        AntMut {
            id,
            grid: &mut self.grid,
            data: &mut self.ants[id],
        }
    }

    pub fn ants(&self) -> impl Iterator<Item = Ant<'_>> {
        (0..self.ants.len()).map(|id| self.ant(id))
    }

    pub fn ant_ids(&self) -> impl Iterator<Item = AntId> {
        0..self.ants.len()
    }

    pub fn cell_of(&self, id: AntId) -> &Cell {
        self.grid.cell_at(self.ant(id).position()).unwrap()
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

            let add_result = world.add_ant(Color::Red, pos);
            assert!(add_result.is_ok());
            let id = add_result.unwrap();

            assert_eq!(world.ant(id).id(), id);
            assert_eq!(world.swarm(Color::Red).next().map(|ant| ant.id()), Some(id));
            assert_eq!(world.swarm(Color::Black).next(), None);
            assert_eq!(world.grid().ant_at(pos), Some(id));
        }

        #[test]
        fn add_ant_into_wall() {
            let blocked_pos = Position { x: 6, y: 7 };
            let mut grid = Grid::new(10, 15);
            *grid.cell_at_mut(blocked_pos).unwrap() = Cell::Wall;

            let mut world = World::new(grid);
            assert_eq!(
                world.add_ant(Color::Red, blocked_pos),
                Err(WorldError::Wall)
            );
        }

        #[test]
        fn add_ant_into_occupied() {
            let mut world = World::new(Grid::new(10, 15));

            let pos = Position { x: 6, y: 7 };
            assert!(world.add_ant(Color::Red, pos).is_ok());
            assert_eq!(world.add_ant(Color::Red, pos), Err(WorldError::Occupied));
        }

        #[test]
        fn rotate_ant() {
            let mut world = World::new(Grid::new(10, 15));

            let id = world.add_ant(Color::Red, Position { x: 6, y: 7 }).unwrap();
            world.ant_mut(id).rotate(Direction::DownRight);

            assert_eq!(world.ant(id).direction(), Direction::DownRight);
        }

        #[test]
        fn move_ant_ok() {
            let mut world = World::new(Grid::new(10, 15));

            let pos = Position { x: 6, y: 7 };
            let new_pos = pos.translate(Direction::default());

            let id = world.add_ant(Color::Red, pos).unwrap();

            assert!(world.ant_mut(id).move_forward().is_ok());
            assert_eq!(world.grid().ant_at(pos), None);
            assert_eq!(world.grid().ant_at(new_pos), Some(id));
            assert_eq!(world.ant(id).position(), new_pos);
        }

        #[test]
        fn move_ant_out_of_bounds() {
            let mut world = World::new(Grid::new(10, 15));

            let id = world.add_ant(Color::Red, Position { x: 9, y: 7 }).unwrap();

            assert_eq!(
                world.ant_mut(id).move_forward(),
                Err(WorldError::OutOfBounds)
            );
        }

        #[test]
        fn move_ant_into_occupied() {
            let mut world = World::new(Grid::new(10, 15));

            let pos = Position { x: 6, y: 7 };
            let new_pos = pos.translate(Direction::default());

            let id = world.add_ant(Color::Red, pos).unwrap();
            world.add_ant(Color::Red, new_pos).unwrap();

            assert_eq!(world.ant_mut(id).move_forward(), Err(WorldError::Occupied));
        }

        #[test]
        fn move_ant_into_wall() {
            let mut grid = Grid::new(10, 15);
            let pos = Position { x: 6, y: 7 };
            let new_pos = pos.translate(Direction::default());
            *grid.cell_at_mut(new_pos).unwrap() = Cell::Wall;

            let mut world = World::new(grid);
            let id = world.add_ant(Color::Red, pos).unwrap();

            assert_eq!(world.ant_mut(id).move_forward(), Err(WorldError::Wall));
        }

        #[test]
        fn pickup_food() {
            let mut grid = Grid::new(10, 15);
            let pos = Position { x: 6, y: 7 };
            *grid.cell_at_mut(pos).unwrap() = Cell::FreeCell {
                ant_id: None,
                food: 5,
            };

            let mut world = World::new(grid);
            let id = world.add_ant(Color::Red, pos).unwrap();

            assert_eq!(world.ant_mut(id).pickup_food(), Ok(()));
            assert!(world.ant(id).carries_food());
            assert_eq!(world.grid().cell_at(pos).unwrap().food(), 4);
            assert_eq!(
                world.ant_mut(id).pickup_food(),
                Err(WorldError::AntCarriesFood)
            );
        }

        #[test]
        fn pickup_food_from_empty_cell() {
            let mut grid = Grid::new(10, 15);
            let pos = Position { x: 6, y: 7 };
            *grid.cell_at_mut(pos).unwrap() = Cell::FreeCell {
                ant_id: None,
                food: 0,
            };

            let mut world = World::new(grid);
            let id = world.add_ant(Color::Red, pos).unwrap();

            assert_eq!(
                world.ant_mut(id).pickup_food(),
                Err(WorldError::CellHasNoFood)
            );
        }

        #[test]
        fn drop_food() {
            let mut grid = Grid::new(10, 15);
            let pos = Position { x: 6, y: 7 };
            *grid.cell_at_mut(pos).unwrap() = Cell::FreeCell {
                ant_id: None,
                food: 5,
            };

            let mut world = World::new(grid);
            let id = world.add_ant(Color::Red, pos).unwrap();
            world.ant_mut(id).pickup_food().unwrap();

            assert_eq!(world.ant_mut(id).drop_food(), Ok(()));
            assert!(!world.ant(id).carries_food());
            assert_eq!(world.grid().cell_at(pos).unwrap().food(), 5);
            assert_eq!(world.ant_mut(id).drop_food(), Err(WorldError::AntHasNoFood));
        }
    }
}
