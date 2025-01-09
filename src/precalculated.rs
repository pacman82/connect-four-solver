//! Use the `precalculate` binary in order learn the constants to set here and generate the `
//! scores.dat` file.`
use crate::ConnectFour;

/// `0` Would indicate that no preclaculated scores are available. If during the development cycle
/// you messed up, and it does not compile because of invalid contents in `scores.dat`, you can set
/// this to `0` in order to ignore precalculated scores.
/// `1` indicates that up to one stones everything is precalculated, i.e. the first position of the
/// board. `2` would indicate that up to two stones everything is precalculated, i.e. every board
/// with one stone in it, and so on.
const NUM_STONES_PRECALCULATED_UP_TO: u8 = 5;

const PRECALCULATED_INPUT_BYTES: &[u8] = include_bytes!("./scores.dat");

/// Number of unique postions with precalculated scores. Look at the ouput of preallocated to learn
/// this number.
const NUM_SCORES_PRECALCULATED: usize = PRECALCULATED_INPUT_BYTES.len() / (8 + 1);
static PRECALCULATED: [(u64, i8); NUM_SCORES_PRECALCULATED] = load_precalculated();

const fn load_precalculated() -> [(u64, i8); NUM_SCORES_PRECALCULATED] {
    let input_bytes = PRECALCULATED_INPUT_BYTES;
    let mut result = [(0, 0); NUM_SCORES_PRECALCULATED];
    let mut index = 0;
    let length = 8 + 1; // 8 bytes for the board, 1 byte for the score
    loop {
        if index == NUM_SCORES_PRECALCULATED {
            break;
        }
        let encoded_board = u64::from_le_bytes([
            input_bytes[index * length],
            input_bytes[index * length + 1],
            input_bytes[index * length + 2],
            input_bytes[index * length + 3],
            input_bytes[index * length + 4],
            input_bytes[index * length + 5],
            input_bytes[index * length + 6],
            input_bytes[index * length + 7],
        ]);
        let score = input_bytes[index * length + 8] as i8;

        result[index] = (encoded_board, score);
        index += 1;
    }
    result
}

/// It can take seconds to minutes to calculate the score of a board with few stones in it. To
/// keep it fast, we precalculated the scores for a bunch of boards. If there is a precalculated
/// score for the board score is returned with `Some(score)`, otherwise `None` is returned.
pub fn precalculated_score(board: &ConnectFour) -> Option<i8> {
    if board.stones() >= NUM_STONES_PRECALCULATED_UP_TO {
        return None;
    }
    let index = PRECALCULATED
        .binary_search_by_key(&board.encode(), |(k, _)| *k)
        .expect("Must be precalculated");
    Some(PRECALCULATED[index].1)
}
