use std::{
    convert::{TryFrom, TryInto},
    fmt,
    iter::{self, Iterator},
    slice,
    ops::{Index, IndexMut}
};
use rand::prelude::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

const DIGITS: [Digit; 9] = [Digit::One, Digit::Two, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine];

impl From<Digit> for u8 {
    fn from(value: Digit) -> Self {
        Self::from((value as u8) + 1)
    }
}

impl TryFrom<u8> for Digit {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 0 {
            DIGITS.get((value - 1) as usize).copied().ok_or("Digit cannot be greater than 9")
        } else {
            Err("Digit cannot be 0")
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

#[derive(Debug, Clone)]
struct Value {
    cell: Cell,
    options: u16,
}

enum ValueState {
    Defined(Digit),
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

    fn state(&self) -> ValueState {
        if self.options == 0 {
            return ValueState::Impossible
        }

        let mut bits = self.options;
        for &digit in &DIGITS {
            let remaining = bits >> 1;
            if bits & 1 != 0 {
                return if remaining == 0 {
                    ValueState::Defined(digit)
                } else {
                    ValueState::Undefined
                }
            }
            bits = remaining;
        }
        ValueState::Impossible
    }

    fn is_defined(&self) -> bool {
        match self.state() {
            ValueState::Defined(_) => true,
            _ => false,
        }
    }

    fn is_undefined(&self) -> bool {
        match self.state() {
            ValueState::Undefined => true,
            _ => false,
        }
    }

    fn is_impossible(&self) -> bool {
        match self.state() {
            ValueState::Impossible => true,
            _ => false,
        }
    }

    fn digit(&self) -> Option<Digit> {
        match self.state() {
            ValueState::Defined(digit) => Some(digit),
            _ => None
        }
    }

    fn is(&self, digit: Digit) -> bool {
        self.options == 1 << (digit as usize)
    }

    fn set(&mut self, digit: Digit) -> bool {
        let value = 1 << (digit as usize);
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

    fn has_option(&self, digit: Digit) -> bool {
        self.options & (1 << (digit as usize)) != 0
    }

    fn remove_option(&mut self, digit: Digit) -> bool {
        let mask = 1 << (digit as usize);
        let changed = self.options & mask != 0;
        self.options &= !mask;
        changed
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = Digit> + 'a {
        DIGITS.iter().copied().filter(move |&x| self.has_option(x))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state() {
            ValueState::Defined(digit) => write!(f, "{}", digit),
            ValueState::Undefined => write!(f, " "),
            ValueState::Impossible => write!(f, "X"),
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

#[derive(Debug, Clone)]
struct Grid([Value; 81]);

#[derive(Debug)]
enum GridState {
    Complete,
    Incomplete,
    Impossible,
}

fn partial_shuffle<'a, R, T, F>(rng: &mut R, items: &'a mut [T], pred: F) -> &'a[T] where
    R: Rng,
    T: Copy,
    F: Fn(T) -> bool,
{
    let mut len = 0usize;
    for index in 0..items.len() {
        let item = items[index];
        if !pred(item) {
            continue;
        }
        if index > len {
            items[len] = item;
        }
        len += 1;
    }
    &mut items[..len].shuffle(rng);
    &items[..len]
}

impl Grid {
    const CELLS: [Cell; 81] = cells!(cell);
    const ROWS: [Group; 9] = groups!(Row);
    const COLUMNS: [Group; 9] = groups!(Column);
    const BLOCKS: [Group; 9] = groups!(Block);

    fn state(&self) -> GridState {
        self.0.iter().fold(GridState::Complete, |state, value| match (state, value.state()) {
            (GridState::Impossible, _) | (_, ValueState::Impossible) => GridState::Impossible,
            (GridState::Incomplete, _) | (_, ValueState::Undefined) => GridState::Incomplete,
            (GridState::Complete, ValueState::Defined(_)) => GridState::Complete,
        })
    }

    fn options<'a>(&'a self) -> impl Iterator<Item = (Cell, Digit)> + 'a {
        self.0.iter().flat_map(|value| value.iter().map(move |x| (value.cell, x)))
    }

    fn solve(&mut self) -> GridState {
        Solver::new(self).solve()
    }

    fn bruteforce<R: Rng>(&mut self, rng: &mut R) -> bool {
        match self.solve() {
            GridState::Complete => return true,
            GridState::Impossible => return false,
            GridState::Incomplete => (),
        }

        let mut undefined = Grid::CELLS;
        let undefined = partial_shuffle(rng, &mut undefined, |x| self[x].is_undefined());

        for &cell in undefined {
            let value = &self[cell];

            let mut candidates = DIGITS;
            let candidates = partial_shuffle(rng, &mut candidates, |x| value.has_option(x));

            for &digit in candidates {
                let mut attempt = self.clone();
                attempt[cell].set(digit);

                if attempt.bruteforce(rng) {
                    *self = attempt;
                    return true;
                }
            }
        }

        false
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

impl TryFrom<&[u8; 81]> for Grid {
    type Error = &'static str;

    fn try_from(value: &[u8; 81]) -> Result<Self, Self::Error> {
        let mut grid = Grid::default();
        for (index, &value) in value.iter().enumerate() {
            if value > 0 {
                grid[Cell(index as u8)].set(value.try_into()?);
            }
        }
        Ok(grid)
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

    fn solve(&mut self) -> GridState {
        'consumer: loop {
            let mut mask = 1u128;

            for &cell in &Grid::CELLS {
                if self.queue & mask != 0 {
                    self.queue &= !mask;
                    if !self.propagate_cell(cell, mask) {
                        return GridState::Impossible;
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
                            return GridState::Impossible;
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
            GridState::Complete
        } else {
            GridState::Incomplete
        }
    }

    fn propagate_cell(&mut self, cell: Cell, mask: u128) -> bool {
        match self.grid[cell].state() {
            ValueState::Defined(digit) => {
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
            ValueState::Undefined => true,
            ValueState::Impossible => false,
        }
    }

    fn resolve_group(&mut self, group: Group, mask: u128) -> bool {
        let mut defined = 0u8;

        for &digit in &DIGITS {
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
    let mut rng = thread_rng();

    println!("{}", Grid::default());

    for &blk in &Grid::BLOCKS {
        for (index, cell) in blk.into_iter().enumerate() {
            if cell.block() != blk {
                println!("Block {}, index {} => Cell {}, Row {}, Column {}, Block {}", blk.index(), index, cell.0, cell.row().index(), cell.column().index(), cell.block().index());
            }
        }
    }

    let mut grid = Grid::try_from(&[
        1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1,
    ]).unwrap();
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    let mut grid = Grid::try_from(&[
        1, 0, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 2, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 2, 1,
    ]).unwrap();
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::try_from(&[
        1, 2, 3, 4, 0, 6, 7, 8, 9,
        0, 0, 0, 0, 9, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 0, 0,
        0, 0, 0, 5, 7, 2, 0, 0, 0,
        0, 0, 0, 6, 0, 4, 0, 0, 0,
        0, 0, 0, 0, 3, 8, 0, 0, 0,
        0, 0, 0, 0, 2, 0, 0, 0, 0,
        0, 0, 0, 0, 4, 0, 0, 0, 0,
        0, 0, 0, 0, 6, 0, 0, 0, 0,
    ]).unwrap();
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::try_from(&[
        0, 0, 9, 8, 0, 0, 1, 0, 0,
        1, 6, 2, 0, 7, 0, 5, 0, 0,
        0, 3, 0, 1, 2, 9, 7, 0, 0,
        0, 2, 6, 0, 8, 0, 3, 0, 0,
        3, 4, 5, 0, 0, 6, 0, 0, 0,
        0, 0, 1, 7, 4, 3, 0, 0, 6,
        9, 1, 0, 6, 5, 8, 4, 0, 0,
        0, 0, 0, 0, 3, 0, 0, 0, 5,
        2, 0, 4, 9, 0, 0, 0, 8, 0,
    ]).unwrap();
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::try_from(&[
        0, 0, 0, 0, 0, 3, 0, 2, 7,
        0, 0, 0, 0, 6, 7, 0, 0, 0,
        0, 9, 0, 5, 0, 0, 0, 8, 3,
        6, 0, 9, 4, 3, 0, 0, 0, 0,
        0, 5, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 2, 5, 0,
        0, 8, 0, 0, 4, 0, 0, 0, 0,
        0, 3, 7, 0, 0, 2, 0, 9, 0,
        9, 0, 2, 0, 0, 0, 0, 6, 0,
    ]).unwrap();
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::try_from(&[
        0, 0, 5, 0, 0, 0, 7, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        7, 0, 0, 4, 0, 0, 0, 0, 6,
        0, 6, 0, 0, 0, 0, 5, 9, 8,
        4, 0, 0, 0, 8, 0, 0, 0, 0,
        0, 3, 0, 2, 0, 0, 0, 0, 0,
        0, 0, 3, 0, 0, 0, 0, 2, 7,
        0, 0, 0, 0, 4, 0, 0, 0, 0,
        0, 5, 0, 1, 9, 0, 0, 8, 0,
    ]).unwrap();
    let state = grid.solve();
    println!("{}\n{:?} / {:?}", grid, state, grid.state());

    grid = Grid::try_from(&[
        0, 0, 5, 0, 0, 0, 7, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        7, 0, 0, 4, 0, 0, 0, 0, 6,
        0, 6, 0, 0, 0, 0, 5, 9, 8,
        4, 0, 0, 0, 8, 0, 0, 0, 0,
        0, 3, 0, 2, 0, 0, 0, 0, 0,
        0, 0, 3, 0, 0, 0, 0, 2, 7,
        0, 0, 0, 0, 4, 0, 0, 0, 0,
        0, 5, 0, 1, 9, 0, 0, 8, 0,
    ]).unwrap();
    let state = grid.bruteforce(&mut rng);
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

    grid = Grid::default();
    let state = grid.bruteforce(&mut rng);
    println!("{}\n{:?} / {:?}", grid, state, grid.state());
}
