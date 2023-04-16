mod minesweeper;
mod puzzle_enum;
mod requests;

use requests::get_puzzle::get_puzzle;
use requests::utils::get_login_header;
use puzzle_enum::PuzzleTypes;
use minesweeper::configurations::MinesweeperTypes::Minesweeper5x5Easy;

use crate::requests::utils::write_samples_to_file;

fn main() {
    println!("Hello World");
    let login = get_login_header();
    let puzzle = PuzzleTypes::MinesweeperConfig(Minesweeper5x5Easy);
    println!("{:?}", get_puzzle(&puzzle, &login));
    write_samples_to_file(puzzle, 30, "test_puzzles/minesweeper/minesweeper_5x5_easy.txt");
}