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
            7
        } else {
            4
        };

        return minimax_helper(board, player, max_depth); // call helper function to do the solving

    }
}

fn minimax_helper(board: &mut Board, player: Player, depth: u32) -> (i32, usize, usize) { // new helper function that tracks depth 
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

        let (score, _x, _y) = 
        minimax_helper(board, player.flip(), depth - 1); // call helper function, also subtract 1 from depth

        // keep previous method of updating score 
        match player {
        Player::X => {
            if score > best_score {
                best_score = score;
                best_move = mv;
                }
            }
        Player::O => {
            if score < best_score {
                best_score = score;
                best_move = mv;
                }
            }
        }
        board.undo_move(mv, player);
    }

    let (x, y) = best_move;
    return (best_score, x, y);
}

// A helper function for solving the Heuristic function:
fn score_line(a: &Cell, b: &Cell, c: &Cell) -> f32 {
    let mut x = 0;
    let mut o = 0;
    let mut empty = 0;

    // count contents of the 3 cells
    for cell in [a, b, c] {
        match cell {
            Cell::X => x += 1,          // count X
            Cell::O => o += 1,          // count O
            Cell::Empty => empty += 1,  // count empty
            Cell::Wall => return 0.0,   // walls block the line
        }
    }

    // if both players present, line is blocked
    if x > 0 && o > 0 {
        return 0.0;
    }

    // scoring (small values to keep total stable)
    if x == 3 {
        return 1.0        // strong win for X
    } else if x == 2 && empty == 1 {
        return 0.5        // good opportunity for X
    } else if x == 1 && empty == 2 {
        return 0.2        // weak opportunity
    } else if o == 3 {
        return -1.0       // strong win for O
    } else if o == 2 && empty == 1 {
        return -0.5       // threat from O
    } else if o == 1 && empty == 2 {
        return -0.2       // weak threat
    } else {
        return 0.0        // empty or irrelevant
    }
}

//fn heuristic(board: &mut Board) -> i32 { // dedicated heuristic (evaluation) function
//    return board.score();
//}

fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells();              // get 2D board
    let n = cells.len();                        // board size (3 or 5)

    let mut total_score: f32 = 0.0;             // accumulated score
    //let mut total_lines: f32 = 0.0;             // number of 3-cell segments checked

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

    // return the total score directly:
    return total_score as i32
    //if total_lines == 0.0 {
       // return 0; // avoid division by zero (shouldn't happen, but safe)
    //}

    // normalize score into range [-1, 1]
    //let normalized = total_score / total_lines;

    // convert float to integer in [-1, 1]
    //if normalized > 0.0 {
    //    1
    //} else if normalized < 0.0 {
    //    -1
    //} else {
    //    0
    }

