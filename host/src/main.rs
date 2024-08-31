use methods::{HELLO_GUEST_ELF};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Empty,
    Z,
    K,
}

struct Board {
    cells: [Cell; 9],
}

struct Player {
    symbol: Cell,
    wins: u32,
    losses: u32,
    draws: u32,
}

struct SimpleRNG {
    state: u64,
}

impl SimpleRNG {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        SimpleRNG { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn rand_range(&mut self, min: usize, max: usize) -> usize {
        (self.next() % (max - min + 1) as u64) as usize + min
    }
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    fn make_move(&mut self, position: usize, player: Cell) -> bool {
        if position < 9 && self.cells[position] == Cell::Empty {
            self.cells[position] = player;
            true
        } else {
            false
        }
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Cell::Empty)
    }

    fn check_winner(&self) -> Option<Cell> {
        const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        for combo in WINNING_COMBINATIONS.iter() {
            if self.cells[combo[0]] != Cell::Empty
                && self.cells[combo[0]] == self.cells[combo[1]]
                && self.cells[combo[1]] == self.cells[combo[2]]
            {
                return Some(self.cells[combo[0]]);
            }
        }
        None
    }

    fn get_empty_cells(&self) -> Vec<usize> {
        self.cells.iter().enumerate()
            .filter(|(_, &cell)| cell == Cell::Empty)
            .map(|(index, _)| index)
            .collect()
    }
}

fn main() {
    println!("Welcome to Tic-tac-toe!");
    let mut human = Player { symbol: Cell::Z, wins: 0, losses: 0, draws: 0 };
    let mut computer = Player { symbol: Cell::K, wins: 0, losses: 0, draws: 0 };
    let mut rng = SimpleRNG::new();

    loop {
        play_game(&mut human, &mut computer, &mut rng);
        display_scores(&human, &computer);

        if !play_again() {
            break;
        }
    }
    let how_many_wins = 3;
    let input: bool = human.wins == how_many_wins;
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, HELLO_GUEST_ELF).unwrap().receipt;

    // Extract journal of receipt
    let output: bool = receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("Wow it's {} that you won 3 times in a row", output);
}

fn play_game(human: &mut Player, computer: &mut Player, rng: &mut SimpleRNG) {
    let mut board = Board::new();
    let mut current_player = &human.symbol;

    loop {
        display_board(&board);

        let position = if *current_player == human.symbol {
            get_human_move(&board)
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
                update_scores(human, computer, Some(winner));
                break;
            }

            if board.is_full() {
                display_board(&board);
                println!("It's a draw!");
                update_scores(human, computer, None);
                break;
            }

            current_player = if *current_player == human.symbol { &computer.symbol } else { &human.symbol };
        } else {
            println!("Invalid move. Try again.");
        }
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

fn update_scores(human: &mut Player, computer: &mut Player, winner: Option<Cell>) {
    match winner {
        Some(cell) if cell == human.symbol => {
            human.wins += 1;
            computer.losses += 1;
        }
        Some(_) => {
            computer.wins += 1;
            human.losses += 1;
        }
        None => {
            human.draws += 1;
            computer.draws += 1;
        }
    }
}

fn display_scores(human: &Player, computer: &Player) {
    println!("\nScores:");
    println!("You - Wins: {}, Losses: {}, Draws: {}", human.wins, human.losses, human.draws);
    println!("Computer - Wins: {}, Losses: {}, Draws: {}", computer.wins, computer.losses, computer.draws);
}

fn play_again() -> bool {
    loop {
        println!("\nDo you want to play again? (y/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}