use std::{
    convert::TryFrom,
    iter::Iterator,
    mem::size_of,
};
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    println!("size_of(Digit) = {}", size_of::<sudoku::Digit>());
    println!("size_of(Cell) = {}", size_of::<sudoku::Cell>());
    println!("size_of(Group) = {}", size_of::<sudoku::Group>());
    println!("size_of(Value) = {}", size_of::<sudoku::Value>());
    println!("size_of(Grid) = {}", size_of::<sudoku::Grid>());
    println!("size_of(ValueState) = {}", size_of::<sudoku::ValueState>());
    println!("size_of(GridState) = {}", size_of::<sudoku::GridState>());

    println!("{}", sudoku::Grid::default());

    for &blk in &sudoku::BLOCKS {
        for (index, cell) in blk.into_iter().enumerate() {
            if cell.block() != blk {
                println!("Block {}, index {} => Cell {}, Row {}, Column {}, Block {}", blk.index(), index, cell.index(), cell.row().index(), cell.column().index(), cell.block().index());
            }
        }
    }

    let mut grid = sudoku::Grid::try_from(&[
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

    let mut grid = sudoku::Grid::try_from(&[
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

    grid = sudoku::Grid::try_from(&[
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

    grid = sudoku::Grid::try_from(&[
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

    grid = sudoku::Grid::try_from(&[
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

    grid = sudoku::Grid::try_from(&[
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

    grid = sudoku::Grid::try_from(&[
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

    // for &row in &sudoku::Grid::ROWS {
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

    grid = sudoku::Grid::default();
    let state = grid.bruteforce(&mut rng);
    println!("{}\n{:?} / {:?}", grid, state, grid.state());
}
