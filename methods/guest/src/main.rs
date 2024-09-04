use risc0_zkvm::guest::env;
use std::str::FromStr;
use zikzakzoo_core::Cell;
use zikzakzoo_core::Board;
use zikzakzoo_core::SimpleRNG;

pub fn verify_player_win(input: &str) -> bool {
    let mut parts = input.split(',');
    
    // Parse the seed
    let seed = match parts.next().and_then(|s| u64::from_str(s).ok()) {
        Some(s) => s,
        None => return false, // Invalid seed
    };

    let mut rng = SimpleRNG::new(seed);
    let mut board = Board::new();
    let current_player = Cell::Z;

    // Process moves
    for move_str in parts {
        let player_move = match usize::from_str(move_str) {
            Ok(m) if m < 9 => m,
            _ => return false, // Invalid move
        };

        // Player's move
        if !board.make_move(player_move, current_player) {
            return false; // Invalid move
        }

        if let Some(winner) = board.check_winner() {
            return winner == Cell::Z; // Player wins
        }

        // Computer's move
        let empty_cells = board.get_empty_cells();
        if empty_cells.is_empty() {
            return false; // Draw
        }
        let computer_move = empty_cells[rng.rand_range(0, empty_cells.len() - 1)];
        board.make_move(computer_move, Cell::K);

        if board.check_winner() == Some(Cell::K) {
            return false; // Computer wins
        }
    }

    false // Game not finished or draw
}

fn main() {
    // read the input (string representing the SEED and the moves, comma-separated)
    let input: String = env::read();
    
    //verify game
    let result = verify_player_win(&input);
    
    // writing to the journal true if player actually won (and false if there was an issue with input, or if it was a loss or draw
    env::commit(&result);
}