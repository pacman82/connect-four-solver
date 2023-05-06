mod bitboard;
mod solver;

use self::bitboard::PlayerStones;
use std::{fmt, io, str::FromStr};

use bitboard::AllStones;
pub use solver::{score, score2};

/// An integer ranging from 0 to 6 representing a column of the connect four board.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Column(u8);

impl Column {
    /// Column index ranges from 0 to 6
    pub fn from_index(index: u8) -> Column {
        assert!(index < 7);
        Column(index)
    }
}

impl FromStr for Column {
    type Err = &'static str;
    fn from_str(source: &str) -> Result<Column, Self::Err> {
        match source.as_bytes().first() {
            Some(v @ b'1'..=b'7') => Ok(Column(v - b'1')),
            _ => Err("Only digits from 1 to 7 count as valid moves."),
        }
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Column: {}", self.0)
    }
}

/// State of a field in a four in a row board
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    /// Field is not captured by either player
    Empty,
    /// Field contains a stone from Player 1
    PlayerOne,
    /// Field contains a stone from Player 2
    PlayerTwo,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ConnectFour {
    /// Bitborad encoding the stones of the current player.
    current: PlayerStones,
    /// Bitboard encoding all cells containing stones, no matter the player.
    both: AllStones,
}

impl ConnectFour {
    /// Create an empty board
    pub fn new() -> ConnectFour {
        ConnectFour {
            current: PlayerStones::new(),
            both: AllStones::default(),
        }
    }

    /// Inserts a stone for the current player. `true` if move has been legal
    pub fn play(&mut self, column: Column) -> bool {
        // Let's check if the move is legal, otherwise return false.
        if self.both.is_full(column.0) {
            return false;
        }
        // After playing a stone, it is the others player turn, so we flip our bitmask representing
        // the stones of our current player.
        self.current.flip(self.both);
        // Now we add a stone to the bitmask for both player. Since we already flipped the bitmask
        // for the current player, we do so only for the bitmask containing stones of both players.
        self.both.insert(column.0);
        true
    }

    /// `true` if the column is not full.
    pub fn is_legal_move(&self, column: Column) -> bool {
        !self.both.is_full(column.0)
    }

    /// Create a game state from a sequence of moves. Each move represented as a number from 1 to 7
    /// standing for the column the player put in their stones.
    pub fn from_move_list(move_list: &str) -> ConnectFour {
        let mut game = ConnectFour::new();
        for c in move_list
            .as_bytes()
            .iter()
            .map(|c| c - b'1')
            .map(Column::from_index)
        {
            if !game.play(c) {
                panic!("Illegal move in String describing Connect Four Game")
            }
        }
        game
    }

    /// Prints out a text representation of a board to `out`
    pub fn print_to(&self, mut out: impl io::Write) -> io::Result<()> {
        for row in (0..6).rev() {
            for field in (0..7).map(|column| self.cell(row, column)) {
                let c = match field {
                    Cell::PlayerOne => 'X',
                    Cell::PlayerTwo => 'O',
                    Cell::Empty => ' ',
                };
                write!(out, "|{}", c)?;
            }
            writeln!(out, "|")?;
        }
        writeln!(out, "---------------\n 1 2 3 4 5 6 7")
    }

    /// Access any cell of the board and find out whether it is empty, or holding a stone of Player
    /// One or Two.
    fn cell(&self, row: u8, column: u8) -> Cell {
        let players = [Cell::PlayerOne, Cell::PlayerTwo];
        if self.both.is_empty(row, column) {
            Cell::Empty
        } else if self.current.is_empty(row, column) {
            players[(self.both.stones() as usize + 1) % 2]
        } else {
            players[self.both.stones() as usize % 2]
        }
    }

    /// Number of stones in the board
    pub fn stones(&self) -> u8 {
        self.both.stones()
    }

    /// `true` if the player which did insert the last stone has one the game.
    pub fn is_victory(&self) -> bool {
        let mut last_players_stones = self.current;
        last_players_stones.flip(self.both);
        last_players_stones.is_win()
    }
}

impl fmt::Display for ConnectFour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in (0..6).rev() {
            for field in (0..7).map(|column| self.cell(row, column)) {
                let c = match field {
                    Cell::PlayerOne => 'X',
                    Cell::PlayerTwo => 'O',
                    Cell::Empty => ' ',
                };
                write!(f, "|{}", c)?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "---------------\n 0 1 2 3 4 5 6")
    }
}
