pub enum MinesweeperTypes {
    Minesweeper5x5Easy,
    Minesweeper5x5Hard,
}

impl MinesweeperTypes {
    pub fn url(&self) -> String {
        match *self {
            MinesweeperTypes::Minesweeper5x5Easy => String::from("https://www.puzzle-minesweeper.com/minesweeper-5x5-easy/"),
            MinesweeperTypes::Minesweeper5x5Hard => String::from("https://www.puzzle-minesweeper.com/minesweeper-5x5-hard/"),
        }
    }
}