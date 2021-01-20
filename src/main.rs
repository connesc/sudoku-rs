use std::fmt;
use std::iter;
use std::iter::Iterator;
use std::slice;
use std::ops::{Index,IndexMut};

macro_rules! cells {
    ($($a:ident)*) => {
        cells!(@rows ($($a)*) -> (0 1 2 3 4 5 6 7 8))
    };
    (@rows ($($a:ident)*) -> ($($i:tt)*)) => {
        cells!(@cells ($($a)*) -> ($(($i (0 1 2 3 4 5 6 7 8)))*))
    };
    (@cells (cell) -> ($(($i:tt ($($j:expr)*)))*)) => {
        [$($(Cell::in_row($i, $j),)*)*]
    };
    (@cells (value) -> ($(($i:tt ($($j:expr)*)))*)) => {
        [$($(Value::new(Cell::in_row($i, $j)),)*)*]
    };
    (@cells (group $f:ident) -> ($(($i:tt ($($j:expr)*)))*)) => {
        [$([$(Cell::$f($i, $j),)*],)*]
    };
    (@cells (neighbors) -> ($(($i:tt ($($j:expr)*)))*)) => {
        [$($(cells!(@neighbors $i $j (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19)),)*)*]
    };
    (@neighbors $i:tt $j:tt ($($k: tt)*)) => {
        [$(Cell::in_row($i, $j).neighbor($k),)*]
    };
}

macro_rules! groups {
    ($f:ident) => {
        groups!(@groups ($f) -> (0 1 2 3 4 5 6 7 8))
    };
    (@groups ($f:ident) -> ($($i:tt)*)) => {
        [$(Group::$f($i),)*]
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell(u8);

impl Cell {
    const NEIGHBORS: [[Cell; 20]; 81] = cells!(neighbors);

    const fn row(self) -> Group {
        Group::Row(self.0 / 9)
    }

    const fn column(self) -> Group {
        Group::Column(self.0 % 9)
    }

    const fn block(self) -> Group {
        Group::Block((self.0 / 27) * 3 + (self.0 % 9) / 3)
    }

    const fn neighbors(self) -> &'static [Cell; 20] {
        &Self::NEIGHBORS[self.0 as usize]
    }

    const fn in_row(row: u8, index: u8) -> Cell {
        Cell(row * 9 + index)
    }

    const fn in_column(column: u8, index: u8) -> Cell {
        Cell(index * 9 + column)
    }

    const fn in_block(block: u8, index: u8) -> Cell {
        Cell(((block / 3) * 3 + index / 3) * 9 + (block % 3) * 3 + index % 3)
    }

    const fn neighbor(self, index: u8) -> Cell {
        match index {
            0..=7 => {
                self.row().cells()[index as usize + if index < self.column().index() { 0 } else { 1 }]
            },
            8..=15 => {
                let index = index - 8;
                self.column().cells()[index as usize + if index < self.row().index() { 0 } else { 1 }]
            },
            _ => {
                let index = index - 16;
                let index_row = index / 2;
                let index_column = index % 2;
                self.block().cells()[
                    (index_row as usize + if index_row < (self.row().index() % 3) { 0 } else { 1 }) * 3
                    + index_column as usize + if index_column < self.column().index() % 3 { 0 } else { 1 }
                ]
            },
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row(), self.column())
    }
}

struct Value {
    cell: Cell,
    options: u16,
}

enum Digit {
    Defined(u8),
    Undefined,
    Impossible,
}

impl Value {
    fn new(cell: Cell) -> Self {
        Value{
            cell,
            options: (1 << 9) - 1,
        }
    }

    fn digit(&self) -> Digit {
        if self.options == 0 {
            return Digit::Impossible
        }

        let mut bits = self.options;
        for digit in 1..=9 {
            let remaining = bits >> 1;
            if bits & 1 != 0 {
                return if remaining == 0 {
                    Digit::Defined(digit)
                } else {
                    Digit::Undefined
                }
            }
            bits = remaining;
        }
        Digit::Impossible
    }

    fn is_defined(&self) -> bool {
        match self.digit() {
            Digit::Defined(_) => true,
            _ => false,
        }
    }

    fn is_undefined(&self) -> bool {
        match self.digit() {
            Digit::Undefined => true,
            _ => false,
        }
    }

    fn is_impossible(&self) -> bool {
        match self.digit() {
            Digit::Impossible => true,
            _ => false,
        }
    }

    fn is(&self, digit: u8) -> bool {
        self.options == 1 << (digit - 1)
    }

    fn set(&mut self, digit: u8) -> bool {
        let value = 1 << (digit - 1);
        let changed = self.options != value;
        self.options = value;
        changed
    }

    fn reset(&mut self) {
        self.options = (1 << 9) - 1;
    }

    fn empty(&mut self) {
        self.options = 0;
    }

    fn has_option(&self, digit: u8) -> bool {
        self.options & (1 << (digit - 1)) != 0
    }

    fn remove_option(&mut self, digit: u8) -> bool {
        let mask = 1 << (digit - 1);
        let changed = self.options & mask != 0;
        self.options &= !mask;
        changed
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.digit() {
            Digit::Defined(digit) => write!(f, "{}", digit),
            Digit::Undefined => write!(f, " "),
            Digit::Impossible => write!(f, "X"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Group {
    Row(u8),
    Column(u8),
    Block(u8),
}

impl Group {
    const ROWS: [[Cell; 9]; 9] = cells!(group in_row);
    const COLUMNS: [[Cell; 9]; 9] = cells!(group in_column);
    const BLOCKS: [[Cell; 9]; 9] = cells!(group in_block);

    const fn index(self) -> u8 {
        match self { Group::Row(index) | Group::Column(index) | Group::Block(index) => index }
    }

    const fn cells(self) -> &'static[Cell; 9] {
        match self {
            Group::Row(index) => &Group::ROWS[index as usize],
            Group::Column(index) => &Group::COLUMNS[index as usize],
            Group::Block(index) => &Group::BLOCKS[index as usize],
        }
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index() + 1)
    }
}

impl iter::IntoIterator for Group {
    type Item = Cell;
    type IntoIter = iter::Copied<slice::Iter<'static, Cell>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells().into_iter().copied()
    }
}

struct Grid([Value; 81]);

#[derive(Debug)]
enum State {
    Complete,
    Incomplete,
    Impossible,
}

impl Grid {
    const CELLS: [Cell; 81] = cells!(cell);
    const ROWS: [Group; 9] = groups!(Row);
    const COLUMNS: [Group; 9] = groups!(Column);
    const BLOCKS: [Group; 9] = groups!(Block);

    fn new(values: &[u8; 81]) -> Grid {
        let mut grid = Grid::default();
        for (index, &value) in values.iter().enumerate() {
            if value > 0 {
                grid[Cell(index as u8)].set(value);
            }
        }
        grid
    }

    fn state(&self) -> State {
        self.0.iter().fold(State::Complete, |state, value| match (state, value.digit()) {
            (State::Impossible, _) | (_, Digit::Impossible) => State::Impossible,
            (State::Incomplete, _) | (_, Digit::Undefined) => State::Incomplete,
            (State::Complete, Digit::Defined(_)) => State::Complete,
        })
    }

    fn solve(&mut self) -> State {
        Solver::new(self).solve()
    }
}

impl Index<Cell> for Grid {
    type Output = Value;

    fn index(&self, cell: Cell) -> &Self::Output {
        &self.0[cell.0 as usize]
    }
}

impl IndexMut<Cell> for Grid {
    fn index_mut(&mut self, cell: Cell) -> &mut Self::Output {
        &mut self.0[cell.0 as usize]
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid(cells!(value))
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &row in &Grid::ROWS {
            if row.index() % 3 == 0 {
                write!(f, "+-----------+-----------+-----------+\n")?;
            } else {
                write!(f, "|           |           |           |\n")?;
            }
            for (column, cell) in row.into_iter().enumerate() {
                write!(f, "{} {} ", if column % 3 == 0 { '|' } else { ' ' }, self[cell])?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "+-----------+-----------+-----------+")
    }
}

struct Solver<'a> {
    grid: &'a mut Grid,
    queue: u128,
    done: u128,
}

impl<'a> Solver<'a> {
    fn new(grid: &mut Grid) -> Solver {
        Solver {
            grid,
            queue: (1 << 108) - 1,
            done: 0,
        }
    }

    fn solve(&mut self) -> State {
        'consumer: loop {
            let mut mask = 1u128;

            for &cell in &Grid::CELLS {
                if self.queue & mask != 0 {
                    self.queue &= !mask;
                    if !self.propagate_cell(cell, mask) {
                        return State::Impossible;
                    }
                    if self.queue == 0 {
                        break 'consumer;
                    }
                }
                mask <<= 1;
            }

            for &groups in &[&Grid::ROWS, &Grid::COLUMNS, &Grid::BLOCKS] {
                for &group in groups {
                    if self.queue & mask != 0 {
                        self.queue &= !mask;
                        if !self.resolve_group(group, mask) {
                            return State::Impossible;
                        }
                        if self.queue == 0 {
                            break 'consumer;
                        }
                    }
                    mask <<= 1;
                }
            }
        };

        if self.done == (1 << 108) - 1 {
            State::Complete
        } else {
            State::Incomplete
        }
    }

    fn propagate_cell(&mut self, cell: Cell, mask: u128) -> bool {
        match self.grid[cell].digit() {
            Digit::Defined(digit) => {
                for &neighbor in cell.neighbors() {
                    if !self.grid[neighbor].remove_option(digit) {
                        continue;
                    }

                    self.enqueue_cell(neighbor);

                    for &group in &[neighbor.row(), neighbor.column(), neighbor.block()] {
                        self.enqueue_group(group);
                    }
                }

                self.done |= mask;

                true
            },
            Digit::Undefined => true,
            Digit::Impossible => false,
        }
    }

    fn resolve_group(&mut self, group: Group, mask: u128) -> bool {
        let mut defined = 0u8;

        for digit in 1..=9 {
            let mut canditates = group.into_iter().filter(|&x| self.grid[x].has_option(digit));
            match canditates.next() {
                Some(cell) => {
                    if canditates.next().is_some() {
                        continue;
                    }

                    defined += 1;

                    if !self.grid[cell].set(digit) {
                        continue;
                    }

                    self.enqueue_cell(cell);

                    for &group in &[cell.row(), cell.column(), cell.block()] {
                        self.enqueue_group(group);
                    }
                },
                None => {
                    for cell in group {
                        let value = &mut self.grid[cell];
                        if value.is_undefined() {
                            value.empty();
                        }
                    }
                    return false;
                }
            }
        }

        if defined == 9 {
            self.queue &= !mask;
            self.done |= mask;
        }

        true
    }

    fn enqueue_cell(&mut self, cell: Cell) {
        let mask = 1u128 << cell.0;
        if self.done & mask == 0 {
            self.queue |= mask;
        }
    }

    fn enqueue_group(&mut self, group: Group) {
        let mask = 1u128 << match group {
            Group::Row(index) => 81 + index,
            Group::Column(index) => 90 + index,
            Group::Block(index) => 99 + index,
        };
        if self.done & mask == 0 {
            self.queue |= mask;
        }
    }
}

fn main() {
    println!("{}", Grid::default());

    for &blk in &Grid::BLOCKS {
        for (index, cell) in blk.into_iter().enumerate() {
            if cell.block() != blk {
                println!("Block {}, index {} => Cell {}, Row {}, Column {}, Block {}", blk.index(), index, cell.0, cell.row().index(), cell.column().index(), cell.block().index());
            }
        }
    }

    let mut grid = Grid::new(&[
        1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1,
    ]);
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    let mut grid = Grid::new(&[
        1, 0, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 2, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 2, 1,
    ]);
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::new(&[
        1, 2, 3, 4, 0, 6, 7, 8, 9,
        0, 0, 0, 0, 9, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 0, 0,
        0, 0, 0, 5, 7, 2, 0, 0, 0,
        0, 0, 0, 6, 0, 4, 0, 0, 0,
        0, 0, 0, 0, 3, 8, 0, 0, 0,
        0, 0, 0, 0, 2, 0, 0, 0, 0,
        0, 0, 0, 0, 4, 0, 0, 0, 0,
        0, 0, 0, 0, 6, 0, 0, 0, 0,
    ]);
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::new(&[
        0, 0, 9, 8, 0, 0, 1, 0, 0,
        1, 6, 2, 0, 7, 0, 5, 0, 0,
        0, 3, 0, 1, 2, 9, 7, 0, 0,
        0, 2, 6, 0, 8, 0, 3, 0, 0,
        3, 4, 5, 0, 0, 6, 0, 0, 0,
        0, 0, 1, 7, 4, 3, 0, 0, 6,
        9, 1, 0, 6, 5, 8, 4, 0, 0,
        0, 0, 0, 0, 3, 0, 0, 0, 5,
        2, 0, 4, 9, 0, 0, 0, 8, 0,
    ]);
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::new(&[
        0, 0, 0, 0, 0, 3, 0, 2, 7,
        0, 0, 0, 0, 6, 7, 0, 0, 0,
        0, 9, 0, 5, 0, 0, 0, 8, 3,
        6, 0, 9, 4, 3, 0, 0, 0, 0,
        0, 5, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 2, 5, 0,
        0, 8, 0, 0, 4, 0, 0, 0, 0,
        0, 3, 7, 0, 0, 2, 0, 9, 0,
        9, 0, 2, 0, 0, 0, 0, 6, 0,
    ]);
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::new(&[
        0, 0, 5, 0, 0, 0, 7, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        7, 0, 0, 4, 0, 0, 0, 0, 6,
        0, 6, 0, 0, 0, 0, 5, 9, 8,
        4, 0, 0, 0, 8, 0, 0, 0, 0,
        0, 3, 0, 2, 0, 0, 0, 0, 0,
        0, 0, 3, 0, 0, 0, 0, 2, 7,
        0, 0, 0, 0, 4, 0, 0, 0, 0,
        0, 5, 0, 1, 9, 0, 0, 8, 0,
    ]);
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    // for &row in &Grid::ROWS {
    //     for &cell in row {
    //         print!("Cell {} =>", cell);
    //         // for index in 0..20 {
    //         //     let neighbor = cell.neighbor(index);
    //         //     print!(" {}", neighbor);
    //         // }
    //         for &neighbor in cell.neighbors() {
    //             print!(" {}", neighbor)
    //         }
    //         println!();
    //     }
    // }
}
