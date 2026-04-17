use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        
        let max_depth = 4; // set max depth
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
    let all_moves: Vec<(usize, usize)> = board.moves();
    let mut best_move: (usize, usize) = all_moves[0];

    // start by keeping previous loop 
    for mv in all_moves {
        let mut next_board: Board = board.clone();
        next_board.apply_move(mv, player);

        let (score, _x, _y) = 
        minimax_helper(&mut next_board, player.flip(), depth - 1); // call helper function, also subtract 1 from depth

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
    }

    let (x, y) = best_move;
    return (best_score, x, y);
}

fn heuristic(board: &Board) -> i32 { // dedicated heuristic (evaluation) function
    return board.score();
}

