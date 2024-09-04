use methods::{ZIKZAKZOO_GUEST_ELF};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use zikzakzoo_core::Player;
use zikzakzoo_core::Cell;
use zikzakzoo_core::Board;
use zikzakzoo_core::SimpleRNG;

pub struct GameRound {
    seed: u64,
    player_moves: Vec<usize>,
}

fn main() {
    println!("Welcome to ZiK-ZaK-Zoo!");
    let human = Player { symbol: Cell::Z };
    let computer = Player { symbol: Cell::K };
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let mut rng = SimpleRNG::new(seed);

    let game_round = play_game(&human, &computer, &mut rng);

    println!("\nGame Round Data:");
    println!("Seed used: {}", game_round.seed);
    println!("Player moves: {:?}", game_round.player_moves);

    let input = format_seed_and_moves(game_round.seed, &game_round.player_moves);

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, ZIKZAKZOO_GUEST_ELF).unwrap().receipt;

    // Extract journal of receipt
    let output: bool = receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("Wow it's {} that you won at ZiK-ZaK-ZoO!", output);
}

fn play_game(human: &Player, computer: &Player, rng: &mut SimpleRNG)  -> GameRound {
    let mut board = Board::new();
    let mut current_player = &human.symbol;
    let seed = rng.state;
    let mut player_moves = Vec::new();

        loop {
        display_board(&board);

        let position = if *current_player == human.symbol {
            let move_position = get_human_move(&board);
            player_moves.push(move_position);
            move_position
        } else {
            get_computer_move(&board, rng)
        };

        if board.make_move(position, *current_player) {
            if let Some(winner) = board.check_winner() {
                display_board(&board);
                if winner == human.symbol {
                    println!("You win!");
                } else {
                    println!("Computer wins!");
                }
                break;
            }

            if board.is_full() {
                display_board(&board);
                println!("It's a draw!");
                break;
            }

            current_player = if *current_player == human.symbol { &computer.symbol } else { &human.symbol };
        } else {
            println!("Invalid move. Try again.");
        }
    }
    
    GameRound {
        seed,
        player_moves,
    }
}

fn display_board(board: &Board) {
    for i in 0..3 {
        for j in 0..3 {
            let cell = match board.cells[i * 3 + j] {
                Cell::Empty => (i * 3 + j).to_string(),
                Cell::Z => "Z".to_string(),
                Cell::K => "K".to_string(),
            };
            print!("{}", cell);
            if j < 2 {
                print!("|");
            }
        }
        println!();
        if i < 2 {
            println!("-+-+-");
        }
    }
    println!();
}

fn get_human_move(board: &Board) -> usize {
    loop {
        println!("Enter your move (0-8):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if num < 9 && board.cells[num] == Cell::Empty => return num,
            _ => println!("Invalid move. Please enter a number between 0 and 8 for an empty cell."),
        }
    }
}

fn get_computer_move(board: &Board, rng: &mut SimpleRNG) -> usize {
    let empty_cells = board.get_empty_cells();
    let random_index = rng.rand_range(0, empty_cells.len() - 1);
    empty_cells[random_index]
}

fn format_seed_and_moves(seed: u64, moves: &[usize]) -> String {
    let mut result = seed.to_string();
    for &m in moves {
        result.push(',');
        result.push_str(&m.to_string());
    }
    result
}