use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        if board.game_over() {
            let score = board.score();
            return (score, 0, 0);
        }

        // Initialize "best" depending on whether we are maximizing or minimizing:
        let mut best_score = match player {
            Player::X => i32::MIN, // X wants to maximize, so it should start from minium
            Player::O => i32::MAX, // O wants to minimize, so it should start from maximum.
        };

        // generate a vector containing all possible moves:
        let all_moves = board.moves();

        // initialize the best move as the first move for now:
        let mut best_move = all_moves[0];

        // use a for loop to iterate every moves:
        for mv in all_moves {
            // Clone so we do not destroy the original board state:
            let mut next_board = board.clone();

            // apply the move with my player:
            next_board.apply_move(mv, player);

            // Recurse for the opponent. How?
            let (score, x, y) =
                SolutionAgent::solve(&mut next_board, player.flip(), _time_limit);

            // update the best moves data:
            match player {

                // if the player is X:
                Player::X => {

                    // if score is higher than the old best moves, replace it:
                    if score > best_score {
                        best_score = score;
                        best_move = mv;
                    }
                }

                // if the Player is O:
                Player::O => {
                    // if score is higher than the old best moves, replace it:
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
}
