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
            options: 0b111111111,
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

    fn set(&mut self, digit: u8) {
        self.options = 1 << (digit - 1);
    }

    fn reset(&mut self) {
        self.options = 0b111111111;
    }

    fn has_option(&self, digit: u8) -> bool {
        self.options & (1 << (digit - 1)) != 0
    }

    fn remove_option(&mut self, digit: u8) -> bool {
        let mask = 1 << (digit - 1);
        let existed = self.options & mask != 0;
        self.options &= !mask;
        existed
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
        let mut propagated = [false; 81];
        for &cell in &Grid::CELLS {
            if !self.propagate(&mut propagated, cell) {
                return State::Impossible;
            }
        }
        for &groups in &[&Grid::ROWS, &Grid::COLUMNS, &Grid::BLOCKS] {
            for &group in groups {
                if !self.resolve_group(&mut propagated, group) {
                    return State::Impossible;
                }
            }
        }
        self.state()
    }

    fn propagate(&mut self, propagated: &mut [bool; 81], cell: Cell) -> bool {
        if propagated[cell.0 as usize] {
            return true;
        }
        match self[cell].digit() {
            Digit::Defined(digit) => {
                propagated[cell.0 as usize] = true;

                for &neighbor in cell.neighbors() {
                    if !self[neighbor].remove_option(digit) {
                        continue;
                    }

                    if !self.propagate(propagated, neighbor) {
                        return false;
                    }

                    for &group in &[neighbor.row(), neighbor.column(), neighbor.block()] {
                        if !self.resolve_group(propagated, group) {
                            return false;
                        }
                    }
                }

                true
            },
            Digit::Undefined => true,
            Digit::Impossible => false,
        }
    }

    fn resolve_group(&mut self, propagated: &mut [bool; 81], group: Group) -> bool {
        for digit in 1..=9 {
            let mut canditates = group.into_iter().filter(|&x| self[x].has_option(digit));
            if let Some(cell) = canditates.next() {
                if canditates.next().is_none() && self[cell].is_undefined() {
                    self[cell].set(digit);
                    if !self.propagate(propagated, cell) {
                        return false;
                    }

                    //TODO: re-resolve groups?
                }
            }
        }
        true
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
    grid.solve();
    println!("{}", grid);

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
    grid.solve();
    println!("{}", grid);

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
    grid.solve();
    println!("{}", grid);

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
    grid.solve();
    println!("{}", grid);

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
    grid.solve();
    println!("{}", grid);

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
