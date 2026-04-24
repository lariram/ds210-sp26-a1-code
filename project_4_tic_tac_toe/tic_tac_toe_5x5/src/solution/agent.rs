use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::board::Cell;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        
        let available_moves = board.moves().len();
        // let max_depth = 4; // set max depth
        let max_depth = if available_moves <= 9 { // check if board is 3x3 or 5x5 to determine best depth
            6
        } else {
            4
        };

        return minimax_helper(board, player, max_depth, i32::MIN, i32::MAX); // call helper function to do the solving, added alpha and beta 

    }
}

fn minimax_helper(board: &mut Board, player: Player, depth: u32, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) { // new helper function that tracks depth, added alpha and beta 
    // alpha = best score of max, starts at negative infinity; beta = best score of min, starts at positive infinity
    if board.game_over() || depth == 0 { // this stops the game or when it naturally ends or when depth count hits 0
        let score: i32 = heuristic(board); // pass through heuristic to get score
        return (score, 0, 0); 
    }

    // keep original initialization code
    let mut best_score: i32 = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };

    // generate all moves:
    let all_moves: Vec<(usize, usize)> = board.moves();

    // set the best and first move:
    //let n = board.get_cells().len();
    // let center = (n/2,n/2);
    let mut best_move = all_moves[0];

    // start by keeping previous loop 
    for mv in all_moves {
        
        board.apply_move(mv, player);
        let (score, _x, _y) = minimax_helper(board, player.flip(), depth - 1, alpha, beta); // call helper function, also subtract 1 from depth; add alpha and beta

        // keep previous method of updating score 
        match player {
        Player::X => {
            if score > best_score {
                best_score = score;
                best_move = mv;
                }
            alpha = std::cmp::max(alpha, best_score);
            if alpha >= beta {
                break; // stop searching this branch
                }
            }
        Player::O => {
            if score < best_score {
                best_score = score;
                best_move = mv;
                }
            beta = std::cmp::min(beta, best_score);
            if alpha >= beta {
                break; // stop searching this branch
                }
            }
        }
        board.undo_move(mv, player);
    }

    let (x, y) = best_move;
    return (best_score, x, y);
}

// A helper function for solving the Heuristic function:
fn score_line(a: &Cell, b: &Cell, c: &Cell) -> i32 {
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
    if x == 3 {
        return 1000        // strong win for X
    } else if x == 2 && empty == 1 {
        return 80        // good opportunity for X
    } else if x == 1 && empty == 2 {
        return 10        // weak opportunity
    } else if o == 3 {
        return -1000       // strong win for O
    } else if o == 2 && empty == 1 {
        return -90       // threat from O, slightly more important than we are x =2 and empty =1.
    } else if o == 1 && empty == 2 {
        return -10       // weak threat
    } else {
        return 0        // empty or irrelevant
    }
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

fn heuristic(board: &Board) -> i32 {
    
    let cells = board.get_cells();              // get 2D board
    let n = cells.len();                       // board size (3 or 5)

    if n == 3 {
        return board.score();
    }

    let mut total_score: i32 = 0;             // accumulated score
    //let mut total_lines: i32 = 0;             // number of 3-cell segments checked

    // loop over every cell as a starting point
    for i in 0..n {
        for j in 0..n {

            // check horizontal segment (i, j) → (i, j+2)
            if j + 2 < n {
                total_score += score_line(&cells[i][j], &cells[i][j + 1], &cells[i][j + 2]);
                //total_lines += 1.0;
            }

            // check vertical segment (i, j) → (i+2, j)
            if i + 2 < n {
                total_score += score_line(&cells[i][j], &cells[i + 1][j], &cells[i + 2][j]);
                //total_lines += 1.0;
            }

            // check diagonal down-right
            if i + 2 < n && j + 2 < n {
                total_score += score_line(
                    &cells[i][j],
                    &cells[i + 1][j + 1],
                    &cells[i + 2][j + 2],
                );
                //total_lines += 1.0;
            }

            // check diagonal down-left
            if i + 2 < n && j >= 2 {
                total_score += score_line(
                    &cells[i][j],
                    &cells[i + 1][j - 1],
                    &cells[i + 2][j - 2],
                );
                //total_lines += 1.0;
            }
        }
    }

    
    // Add a small positional bonus for pieces in flexible locations.
    // This rewards overlap potential without making the heuristic too slow.
    total_score += position_bonus(cells); // CHANGE MADE HERE!
    return total_score;
    
    }


