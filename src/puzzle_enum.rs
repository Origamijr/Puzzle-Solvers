use crate::minesweeper::configurations::MinesweeperTypes;

pub enum PuzzleTypes {
    MinesweeperConfig(MinesweeperTypes)
}

impl PuzzleTypes {
    pub fn url(&self) -> String {
        match self {
            PuzzleTypes::MinesweeperConfig(c) => c.url()
        }
    }
}