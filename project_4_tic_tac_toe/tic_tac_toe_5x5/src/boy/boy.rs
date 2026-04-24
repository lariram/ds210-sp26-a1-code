use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let moves_left = board.moves().len();

        // Use deeper search near the end because branching factor is smaller.
        let max_depth = if moves_left <= 7 {
            7
        } else if moves_left <= 12 {
            6
        } else if moves_left <= 18 {
            5
        } else {
            4
        };

        minimax(
            board,
            player,
            max_depth,
            i32::MIN + 1,
            i32::MAX - 1,
        )
    }
}

// Minimax with alpha-beta pruning.
// X maximizes the score, O minimizes the score.
fn minimax(
    board: &mut Board,
    player: Player,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
) -> (i32, usize, usize) {
    if board.game_over() || depth == 0 {
        return (heuristic(board), 0, 0);
    }

    let mut moves = board.moves();

    // Sort promising moves first so alpha-beta pruning cuts more branches.
    order_moves(board, &mut moves, player);

    let mut best_move = moves[0];

    match player {
        Player::X => {
            let mut best_score = i32::MIN + 1;

            for mv in moves {
                board.apply_move(mv, player);

                let (score, _, _) = minimax(
                    board,
                    player.flip(),
                    depth - 1,
                    alpha,
                    beta,
                );

                board.undo_move(mv, player);

                if score > best_score {
                    best_score = score;
                    best_move = mv;
                }

                alpha = alpha.max(best_score);

                // O already has a better option elsewhere, so stop searching.
                if beta <= alpha {
                    break;
                }
            }

            (best_score, best_move.0, best_move.1)
        }

        Player::O => {
            let mut best_score = i32::MAX - 1;

            for mv in moves {
                board.apply_move(mv, player);

                let (score, _, _) = minimax(
                    board,
                    player.flip(),
                    depth - 1,
                    alpha,
                    beta,
                );

                board.undo_move(mv, player);

                if score < best_score {
                    best_score = score;
                    best_move = mv;
                }

                beta = beta.min(best_score);

                // X already has a better option elsewhere, so stop searching.
                if beta <= alpha {
                    break;
                }
            }

            (best_score, best_move.0, best_move.1)
        }
    }
}

// Move ordering improves alpha-beta pruning.
// We prefer moves that immediately improve the current player’s position.
fn order_moves(board: &mut Board, moves: &mut Vec<(usize, usize)>, player: Player) {
    moves.sort_by_key(|&mv| {
        board.apply_move(mv, player);
        let value = heuristic(board);
        board.undo_move(mv, player);

        match player {
            Player::X => -value, // Larger X score first.
            Player::O => value,  // Smaller O score first.
        }
    });
}

// Strong heuristic for 5x5 scoring.
// Positive means good for X. Negative means good for O.
fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells();
    let n = cells.len();

    let mut score = 0;

    // Real completed score matters most.
    score += board.score() * 1000;

    // Evaluate every possible 3-cell scoring window.
    for i in 0..n {
        for j in 0..n {
            if j + 2 < n {
                score += evaluate_window(&cells[i][j], &cells[i][j + 1], &cells[i][j + 2]);
            }

            if i + 2 < n {
                score += evaluate_window(&cells[i][j], &cells[i + 1][j], &cells[i + 2][j]);
            }

            if i + 2 < n && j + 2 < n {
                score += evaluate_window(
                    &cells[i][j],
                    &cells[i + 1][j + 1],
                    &cells[i + 2][j + 2],
                );
            }

            if i + 2 < n && j >= 2 {
                score += evaluate_window(
                    &cells[i][j],
                    &cells[i + 1][j - 1],
                    &cells[i + 2][j - 2],
                );
            }
        }
    }

    // Reward pieces in flexible positions.
    score += position_control_score(cells);

    // Reward empty cells that could create many future scoring windows.
    score += future_potential_score(cells);

    score
}

// Scores a 3-cell segment.
// Bigger weights make the agent strongly prefer creating/blocking 2-in-a-row threats.
fn evaluate_window(a: &Cell, b: &Cell, c: &Cell) -> i32 {
    let mut x_count = 0;
    let mut o_count = 0;
    let mut empty_count = 0;

    for cell in [a, b, c] {
        match cell {
            Cell::X => x_count += 1,
            Cell::O => o_count += 1,
            Cell::Empty => empty_count += 1,
            Cell::Wall => return 0, // Walls break scoring windows.
        }
    }

    // Mixed windows cannot become a score for either player.
    if x_count > 0 && o_count > 0 {
        return 0;
    }

    match (x_count, o_count, empty_count) {
        // Completed scoring windows.
        (3, 0, 0) => 1000,
        (0, 3, 0) => -1000,

        // Immediate threats/opportunities.
        (2, 0, 1) => 120,
        (0, 2, 1) => -120,

        // Early potential.
        (1, 0, 2) => 20,
        (0, 1, 2) => -20,

        _ => 0,
    }
}

// Gives value to center and near-center pieces.
// Central cells usually participate in more possible 3-cell lines.
fn position_control_score(cells: &Vec<Vec<Cell>>) -> i32 {
    let n = cells.len();
    let center = (n / 2) as i32;

    let mut score = 0;

    for i in 0..n {
        for j in 0..n {
            let distance = (i as i32 - center).abs() + (j as i32 - center).abs();
            let value = 8 - distance;

            match cells[i][j] {
                Cell::X => score += value,
                Cell::O => score -= value,
                _ => {}
            }
        }
    }

    score
}

// Estimates how useful each empty cell is for future scoring.
// Empty cells that lie inside many unblocked 3-cell windows are strategically valuable.
fn future_potential_score(cells: &Vec<Vec<Cell>>) -> i32 {
    let n = cells.len();
    let mut score = 0;

    for i in 0..n {
        for j in 0..n {
            if cells[i][j] != Cell::Empty {
                continue;
            }

            let x_potential = potential_if_played(cells, i, j, Cell::X);
            let o_potential = potential_if_played(cells, i, j, Cell::O);

            score += x_potential;
            score -= o_potential;
        }
    }

    score
}

// Measures how many useful scoring windows would involve this cell
// if the chosen player placed there.
fn potential_if_played(cells: &Vec<Vec<Cell>>, row: usize, col: usize, player_cell: Cell) -> i32 {
    let n = cells.len();
    let mut total = 0;

    let directions = [
        (0_i32, 1_i32),
        (1_i32, 0_i32),
        (1_i32, 1_i32),
        (1_i32, -1_i32),
    ];

    for (dr, dc) in directions {
        // The played cell can appear as position 0, 1, or 2 in a 3-cell window.
        for offset in 0..3 {
            let start_r = row as i32 - offset * dr;
            let start_c = col as i32 - offset * dc;

            let mut x_count = 0;
            let mut o_count = 0;
            let mut valid = true;

            for k in 0..3 {
                let r = start_r + k * dr;
                let c = start_c + k * dc;

                if r < 0 || c < 0 || r >= n as i32 || c >= n as i32 {
                    valid = false;
                    break;
                }

                let cell = if r as usize == row && c as usize == col {
                    &player_cell
                } else {
                    &cells[r as usize][c as usize]
                };

                match cell {
                    Cell::X => x_count += 1,
                    Cell::O => o_count += 1,
                    Cell::Wall => {
                        valid = false;
                        break;
                    }
                    Cell::Empty => {}
                }
            }

            if !valid || (x_count > 0 && o_count > 0) {
                continue;
            }

            // Strongly reward moves that create 2-in-a-row or completed triples.
            total += match (x_count, o_count) {
                (3, 0) | (0, 3) => 80,
                (2, 0) | (0, 2) => 25,
                (1, 0) | (0, 1) => 5,
                _ => 1,
            };
        }
    }

    total
}