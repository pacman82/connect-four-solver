mod bitboard;
mod precalculated;
mod solver;
mod transposition_table;

use self::bitboard::PlayerStones;
use std::{fmt, io, str::FromStr};

use bitboard::{heuristic, AllStones, NonLoosingMoves};
pub use solver::{score, Solver};

/// An integer ranging from 0 to 6 representing a column of the connect four board.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Column(u8);

impl Column {
    /// Column index ranges from 0 to 6
    pub const fn from_index(index: u8) -> Column {
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
        write!(f, "{}", self.0 + 1)
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

/// Implementation of the Connect Four game. The board is implemented as to 64 bit masks. It allows
/// for fast checking of winning conditions and legal moves. Apart from being able to play connect
/// four, this type also features some utility functions which can help with implementations of
/// heuristics and solvers.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ConnectFour {
    /// Bitborad encoding the stones of the player who did insert the last stone. Starts with Player
    /// two.
    last: PlayerStones,
    /// Bitboard encoding all cells containing stones, no matter the player.
    both: AllStones,
}

impl ConnectFour {
    /// Create an empty board
    pub fn new() -> ConnectFour {
        ConnectFour {
            last: PlayerStones::new(),
            both: AllStones::default(),
        }
    }

    /// Inserts a stone for the current player. `true` if move has been legal
    pub fn play(&mut self, column: Column) -> bool {
        // Let's check if the move is legal, otherwise return false.
        if self.both.is_full(column.0) {
            return false;
        }
        // Now we add a stone to the bitmask for both player.
        self.both.insert(column.0);
        // Flip players after adding the stone, so the stone is accounted for the last player
        self.last.flip(self.both);
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
        write!(out, "{self}")
    }

    pub fn legal_moves(&self) -> impl Iterator<Item = Column> + use<'_>{
        (0..7).map(Column::from_index).filter(move |&c| self.is_legal_move(c))
    }

    /// Access any cell of the board and find out whether it is empty, or holding a stone of Player
    /// One or Two.
    fn cell(&self, row: u8, column: u8) -> Cell {
        let players = [Cell::PlayerOne, Cell::PlayerTwo];
        if self.both.is_empty(row, column) {
            Cell::Empty
        } else if self.last.is_empty(row, column) {
            players[self.both.stones() as usize % 2]
        } else {
            players[(self.both.stones() as usize + 1) % 2]
        }
    }

    /// Heurisitc used to decide which moves to explore first, in order to allow for better pruning
    /// of the search tree. Higher means better for the player which put in the last stone.
    fn heuristic(&self) -> u32 {
        heuristic(self.last, self.both)
    }

    /// Number of stones in the board
    pub fn stones(&self) -> u8 {
        self.both.stones()
    }

    /// `true` if the player which did insert the last stone has won the game.
    pub fn is_victory(&self) -> bool {
        self.last.is_win()
    }

    /// Uses the first 49 Bits to uniquely encode the board.
    pub fn encode(&self) -> u64 {
        self.last.key(self.both)
    }

    /// `true` if the current player has winning moves available
    pub fn can_win_in_next_move(&self) -> bool {
        let mut current = self.last;
        current.flip(self.both);
        self.both.possible() & current.winning_positions() != 0
    }

    /// `true` if game has a winner or is a draw.
    pub fn is_over(&self) -> bool {
        self.stones() == 42 || self.is_victory()
    }

    /// List all moves, which prevent the opponet from winning immediately. Only gives valid results
    /// if [`Self::can_win_in_next_move`] is `false`.
    pub fn non_loosing_moves(&self) -> impl Iterator<Item = Column> {
        debug_assert!(!self.can_win_in_next_move());
        let nlm = self.non_loosing_moves_impl();
        (0..7).filter(move |&i| nlm.contains(i)).map(Column::from_index)
    }

    // Only valid to call if `can_win_in_next_move` is `false`.
    fn non_loosing_moves_impl(&self) -> NonLoosingMoves {
        debug_assert!(!self.can_win_in_next_move());
        NonLoosingMoves::new(self.last, self.both)
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
        writeln!(f, "---------------\n 1 2 3 4 5 6 7")
    }
}
