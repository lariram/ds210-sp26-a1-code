use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::board::Cell;

// Your solution
pub struct SolutionAgent {}

pub const DEFAULT_WEIGHTS_X: [i32; 6] = [10000, 2910, -3472, 128, 4080, 1947]; // best X numbers
pub const DEFAULT_WEIGHTS_O: [i32; 6] = [10000, 683, 1063, 335, -490, -835]; // best O numbers

// training command to train as x: cargo run -p tic_tac_toe_5x5 --bin main -- --x solution --o test --layout 5
// training command to train as o: cargo run -p tic_tac_toe_5x5 --bin main -- --x test --o solution --layout 5

// Put your solution here.
impl Agent for SolutionAgent {
    
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {

        let available_moves = board.moves().len();
        // let max_depth = 4; // set max depth
        let max_depth = if available_moves <= 9 { // check if board is 3x3 or 5x5 to determine best depth
            7
        } else if available_moves <= 14 {
            6 // Change: late game has fewer choices, so we can safely search deeper.
        } else if available_moves <= 18 {
            5 // Change: middle game can search a little deeper than the opening.
        } else {
            4 // Change: early game has many choices, so keep depth smaller to avoid timeout.
        };

        let weights = match player {
            Player::X => &DEFAULT_WEIGHTS_X,
            Player::O => &DEFAULT_WEIGHTS_O,
        }; 
       
        return minimax_helper(board, player, max_depth, i32::MIN, i32::MAX, weights); // call helper function to do the solving, add default weights for machine learning
    }
}


fn minimax_helper(board: &mut Board, player: Player, depth: u32, mut alpha: i32, mut beta: i32, weights: &[i32; 6]) -> (i32, usize, usize) { // new helper function that tracks depth 
    if board.game_over() {
        return (board.score() * 10000, 0, 0);
    }

    if depth == 0 {
        return (heuristic(board, weights), 0, 0);
    }

    // keep original initialization code
    let mut best_score: i32 = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };

    // generate all moves:
    let mut all_moves: Vec<(usize, usize)> = board.moves();

    // sort the moves from smallest to largest, pass by reference for each move (only after depths of 2 so save time):
    if depth >= 2 {
        all_moves.sort_by_key(|&mv| {
            board.apply_move(mv, player); // Try this move temporarily.

            let score = heuristic(board, weights); // CHANGE: use weights + heuristic to score

            board.undo_move(mv, player); // Undo the temporary move so the board returns to normal

            match player {
                Player::X => -score, // X searches moves with higher scores first.
                Player::O => score, // O searches moves with lower scores first.
            }
        });
    }
    // set the best and first move:
    // let n = board.get_cells().len();
    // let center = (n/2,n/2);
    let mut best_move = all_moves[0];

    // start by keeping previous loop 
    for mv in all_moves {
        
        board.apply_move(mv, player);

        let (score, _x, _y) = 
        minimax_helper(board, player.flip(), depth - 1, alpha, beta, weights); // call helper function, also subtract 1 from depth

        board.undo_move(mv, player); // Change: undo immediately so pruning cannot leave the board changed.

        // keep previous method of updating score 
        match player {
        Player::X => {
            if score > best_score {
                best_score = score;
                best_move = mv;
                }
            alpha = alpha.max(best_score); // Change: update alpha because X wants the highest score.

                if beta <= alpha {
                    break; // Change: stop searching because O already has a better option elsewhere.
                }
            }
        Player::O => {
            if score < best_score {
                best_score = score;
                best_move = mv;
                }
            beta = beta.min(best_score); // Change: update beta because O wants the lowest score.

                if beta <= alpha {
                    break; // Change: stop searching because X already has a better option elsewhere.
                }
            }
        }
        
    }

    let (x, y) = best_move;
    return (best_score, x, y);
}

// A helper function for solving the Heuristic function:
fn score_line(a: &Cell, b: &Cell, c: &Cell, weights: &[i32; 6]) -> i32 {
    let mut x = 0;
    let mut o = 0;
    let mut empty = 0;

    // count contents of the 3 cells
    for cell in [a, b, c] {
        match cell {
            Cell::X => x += 1,          // count X
            Cell::O => o += 1,          // count O
            Cell::Empty => empty += 1,  // count empty
            Cell::Wall => return 0,   // walls block the line
        }
    }

    // if both players present, line is blocked
    if x > 0 && o > 0 {
        return 0;
    }

    // scoring (small values to keep total stable)
    if x == 3 { return weights[1]; }
    else if x == 2 && empty == 1 { return weights[2]; }
    else if x == 1 && empty == 2 { return weights[3]; }
    else if o == 3 { return -weights[1]; }
    else if o == 2 && empty == 1 { return -weights[2]; }
    else if o == 1 && empty == 2 { return -weights[3]; }
    else { return 0; }
}

// This function gives a small bonus based on where pieces are placed on the board.
// Idea: cells closer to the center are more valuable because they can form more lines.
fn position_bonus(cells: &Vec<Vec<Cell>>) -> i32 {

    // Get board size (3 or 5)
    let n = cells.len();  

    // Find the center index (for 5x5 → center = 2)
    let center = (n / 2) as i32;  

    // This will store the total bonus score
    let mut bonus = 0;  

    // Loop through every row
    for i in 0..n {

        // Loop through every column
        for j in 0..n {

            // Compute how far this cell is from the center
            // Example: center (2,2), cell (0,0) → distance = 4
            // abs() computes for absolute value
            let distance_from_center =
                (i as i32 - center).abs() + (j as i32 - center).abs();

            // Convert distance into a score
            // Closer to center → bigger value
            // Farther → smaller value
            // choose 6, because this is the maximum possible distance in 5x5
            let value = 6 - distance_from_center;  

            // Check what is inside the cell
            match cells[i][j] {

                // If it's X, add the value (good for X)
                Cell::X => bonus += value,  

                // If it's O, subtract the value (bad for X)
                Cell::O => bonus -= value,  

                // If it's empty or wall, ignore
                _ => {}
            }
        }
    }

    // Return total positional bonus
    return bonus
}

// add a helper to score 4-cell patterns.
fn score_four_line(a: &Cell, b: &Cell, c: &Cell, d: &Cell, weights: &[i32; 6]) -> i32 { 
    let mut x = 0; // count how many X cells are in this 4-cell line.
    let mut o = 0; // count how many O cells are in this 4-cell line.
    let mut empty = 0; // count how many empty cells are in this 4-cell line.

    for cell in [a, b, c, d] { // check each cell in the 4-cell line.
        match cell { // update the correct counter based on the cell type.
            Cell::X => x += 1, // found one X.
            Cell::O => o += 1, // found one O.
            Cell::Empty => empty += 1, //  found one empty space.
            Cell::Wall => return 0, // walls block the line, so it has no value.
        }
    }

    if x > 0 && o > 0 { // if both players are in the line, neither can use it cleanly.
        return 0; // mixed lines are blocked, so give no score.
    }

    if x == 3 && empty == 1 { return weights[4]; }
    else if o == 3 && empty == 1 { return -weights[4]; }
    else if x == 2 && empty == 2 { return weights[5]; }
    else if o == 2 && empty == 2 { return -weights[5]; }
    else { return 0; }
}

fn heuristic(board: &Board, weights: &[i32; 6]) -> i32 {
    let cells = board.get_cells();      // get 2D board
    let n = cells.len();                        // board size (3 or 5)

    let mut total_score: i32 = board.score() * weights[0];            // accumulated score
    //let mut total_lines: i32 = 0;             // number of 3-cell segments checked

    // loop over every cell as a starting point
    for i in 0..n {
        for j in 0..n {

            // check horizontal segment (i, j) → (i, j+2)
            if j + 2 < n {
                total_score += score_line(&cells[i][j], &cells[i][j + 1], &cells[i][j + 2], weights,);
                //total_lines += 1.0;
            }

            // check vertical segment (i, j) → (i+2, j)
            if i + 2 < n {
                total_score += score_line(&cells[i][j], &cells[i + 1][j], &cells[i + 2][j], weights,);
                //total_lines += 1.0;
            }

            // check diagonal down-right
            if i + 2 < n && j + 2 < n {
                total_score += score_line(
                    &cells[i][j],
                    &cells[i + 1][j + 1],
                    &cells[i + 2][j + 2],
                    weights,
                );
                //total_lines += 1.0;
            }

            // check diagonal down-left
            if i + 2 < n && j >= 2 {
                total_score += score_line(
                    &cells[i][j],
                    &cells[i + 1][j - 1],
                    &cells[i + 2][j - 2],
                    weights,
                );
                //total_lines += 1.0;
            }

            if j + 3 < n { // Make sure a horizontal 4-cell line fits on the board.
                total_score += score_four_line( // Add the value of this horizontal 4-cell pattern.
                    &cells[i][j], // First cell in the horizontal 4-cell line.
                    &cells[i][j + 1], // Second cell in the horizontal 4-cell line.
                    &cells[i][j + 2], // Third cell in the horizontal 4-cell line.
                    &cells[i][j + 3], // Fourth cell in the horizontal 4-cell line.
                    weights,
                );
            }

            if i + 3 < n { // Make sure a vertical 4-cell line fits on the board.
                total_score += score_four_line( // Add the value of this vertical 4-cell pattern.
                    &cells[i][j], // First cell in the vertical 4-cell line.
                    &cells[i + 1][j], // Second cell in the vertical 4-cell line.
                    &cells[i + 2][j], // Third cell in the vertical 4-cell line.
                    &cells[i + 3][j], // Fourth cell in the vertical 4-cell line.
                    weights,
                );
            }

            if i + 3 < n && j + 3 < n { // Make sure a down-right diagonal 4-cell line fits.
                total_score += score_four_line( // Add the value of this down-right diagonal pattern.
                    &cells[i][j], // First cell in the down-right diagonal line.
                    &cells[i + 1][j + 1], // Second cell in the down-right diagonal line.
                    &cells[i + 2][j + 2], // Third cell in the down-right diagonal line.
                    &cells[i + 3][j + 3], // Fourth cell in the down-right diagonal line.
                    weights,
                );
            }

            if i + 3 < n && j >= 3 { // Make sure a down-left diagonal 4-cell line fits.
                total_score += score_four_line( // Add the value of this down-left diagonal pattern.
                    &cells[i][j], // First cell in the down-left diagonal line.
                    &cells[i + 1][j - 1], // Second cell in the down-left diagonal line.
                    &cells[i + 2][j - 2], // Third cell in the down-left diagonal line.
                    &cells[i + 3][j - 3], // Fourth cell in the down-left diagonal line.
                    weights,
                );
            }
        }
    }

    
    // Add a small positional bonus for pieces in flexible locations.
    // This rewards overlap potential without making the heuristic too slow.
    total_score += position_bonus(cells); // CHANGE MADE HERE!
    return total_score;
    
    }



use rand::Rng;

pub fn train_agent<L: tic_tac_toe_stencil::layout::Layout>(layout: L, train_for: Player) {
    let player_name = match train_for {
        Player::X => "X",
        Player::O => "O",
    };

    println!("Starting Machine Learning Training for Player {}...", player_name);
    
    // 1. Automatically grab the correct starting weights
    let mut best_weights = match train_for {
        Player::X => DEFAULT_WEIGHTS_X.clone(),
        Player::O => DEFAULT_WEIGHTS_O.clone(),
    };
    
    let mut rng = rand::thread_rng();
    let mut board = Board::new(layout);

    // Run 100 generations
    for generation in 0..200 {
        let mut test_weights = best_weights.clone();
        let mutate_index = rng.gen_range(1..6);
        test_weights[mutate_index] += rng.gen_range(-300..301);

        let mut turn = Player::X;
        let mut moves_made = Vec::new(); 
        
        while !board.game_over() && board.moves().len() > 0 {
            // 2. Automatically give the mutation to the player we are training
            let current_weights = if turn == train_for {
                &test_weights // The mutation plays for the target player
            } else {
                &best_weights // The baseline plays for the opponent
            };
            
            let (_score, x, y) = minimax_helper(&mut board, turn, 3, i32::MIN, i32::MAX, current_weights);
            
            board.apply_move((x, y), turn);
            moves_made.push(((x, y), turn)); 
            turn = turn.flip();
        }

        // 3. Automatically use the correct win condition
        let is_mutation_better = match train_for {
            Player::X => board.score() >= 0, // X wants positive or 0
            Player::O => board.score() <= 0, // O wants negative or 0
        };

        if is_mutation_better { 
            println!("Gen {}: Mutation WIN for {}! New best weights: {:?}", generation, player_name, test_weights);
            best_weights = test_weights;
        } else {
            println!("Gen {}: Mutation failed.", generation);
        }

        while let Some((mv, p)) = moves_made.pop() {
            board.undo_move(mv, p);
        }
    }
    
    println!("Training Complete! Hardcode these weights into DEFAULT_WEIGHTS_{}:", player_name);
    println!("{:?}", best_weights);
}