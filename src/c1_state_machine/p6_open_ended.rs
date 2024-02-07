//! Now is your chance to get creative. Choose a state machine that interests you and model it here.
//! Get as fancy as you like. The only constraint is that it should be simple enough that you can
//! realistically model it in an hour or two.
//!
//! Here are some ideas:
//! - Board games:
//!   - Chess
//!   - Checkers
//!   - Tic tac toe
//! - Beaurocracies:
//!   - Beauro of Motor Vehicles - maintains driving licenses and vehicle registrations.
//!   - Public Utility Provider - Customers open accounts, consume the utility, pay their bill
//!     periodically, maybe utility prices fluctuate
//!   - Land ownership registry
//! - Tokenomics:
//!   - Token Curated Registry
//!   - Prediction Market
//!   - There's a game where there's a prize to be split among players and the prize grows over
//!     time. Any player can stop it at any point and take most of the prize for themselves.
//! - Social Systems:
//!   - Social Graph
//!   - Web of Trust
//!   - Reputation System

use super::StateMachine;
// use std::io;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ValueInCell{
	Empty,
	X,
	O
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MoveMaker {
	board: [[ValueInCell; 3]; 3],
	player1: ValueInCell,
	player2: ValueInCell,
	is_next_player1: bool,
	winner: Option<ValueInCell>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Action {
	MakeMove(usize,usize),
	CheckWin,
}

impl StateMachine for MoveMaker {
	type State = MoveMaker;
	type Transition = Action;

	fn next_state(starting: &Self::State, t: &Self::Transition) -> Self::State {
		let mut new_state = starting.clone();

        match t {
            Action::MakeMove(row, col) => {
                if new_state.is_next_player1 {
                    new_state.board[*row][*col] = new_state.player1.clone();
                } else {
                    new_state.board[*row][*col] = new_state.player2.clone();
                }

                // Toggle player turn
                new_state.is_next_player1 = !new_state.is_next_player1;
            }
            Action::CheckWin => {
                new_state.winner = new_state.check_winner();
            }
        }

        new_state
    }
}

impl MoveMaker {
    // Constructor
    pub fn new(player1: ValueInCell, player2: ValueInCell) -> Self {
        MoveMaker {
            board: [[ValueInCell::Empty; 3]; 3],
            player1,
            player2,
            is_next_player1: true,
            winner: None,
        }
    }

    // Check for a winner
    fn check_winner(&self) -> Option<ValueInCell> {
        // Check rows
        for i in 0..3 {
            if self.board[i][0] != ValueInCell::Empty
                && self.board[i][0] == self.board[i][1]
                && self.board[i][0] == self.board[i][2]
            {
                return Some(self.board[i][0]);
            }
        }

        // Check columns
        for i in 0..3 {
            if self.board[0][i] != ValueInCell::Empty
                && self.board[0][i] == self.board[1][i]
                && self.board[0][i] == self.board[2][i]
            {
                return Some(self.board[0][i]);
            }
        }

        // Check diagonals
        if self.board[0][0] != ValueInCell::Empty
            && self.board[0][0] == self.board[1][1]
            && self.board[0][0] == self.board[2][2]
        {
            return Some(self.board[0][0]);
        }

        if self.board[0][2] != ValueInCell::Empty
            && self.board[0][2] == self.board[1][1]
            && self.board[0][2] == self.board[2][0]
        {
            return Some(self.board[0][2]);
        }

        // Check for a draw
        if self.board.iter().all(|row| row.iter().all(|&cell| cell != ValueInCell::Empty)) {
            return Some(ValueInCell::Empty);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = MoveMaker::new(ValueInCell::X, ValueInCell::O);
        assert_eq!(game.board, [[ValueInCell::Empty; 3]; 3]);
        assert_eq!(game.player1, ValueInCell::X);
        assert_eq!(game.player2, ValueInCell::O);
        assert_eq!(game.is_next_player1, true);
        assert_eq!(game.winner, None);
    }

    #[test]
    fn test_make_move() {
        let mut game = MoveMaker::new(ValueInCell::X, ValueInCell::O);
        assert_eq!(game.board[0][0], ValueInCell::Empty);

        game = MoveMaker::next_state(&game, &Action::MakeMove(0, 0));
        assert_eq!(game.board[0][0], ValueInCell::X);
        assert_eq!(game.is_next_player1, false);

        game = MoveMaker::next_state(&game, &Action::MakeMove(1, 1));
        assert_eq!(game.board[1][1], ValueInCell::O);
        assert_eq!(game.is_next_player1, true);
    }

    #[test]
    fn test_check_win() {
        let mut game = MoveMaker::new(ValueInCell::X, ValueInCell::O);
        assert_eq!(game.winner, None);

        game.board = [
            [ValueInCell::X, ValueInCell::X, ValueInCell::X],
            [ValueInCell::Empty, ValueInCell::Empty, ValueInCell::Empty],
            [ValueInCell::Empty, ValueInCell::Empty, ValueInCell::Empty],
        ];
        assert_eq!(MoveMaker::next_state(&game, &Action::CheckWin).winner, Some(ValueInCell::X));

        game.board = [
            [ValueInCell::O, ValueInCell::Empty, ValueInCell::Empty],
            [ValueInCell::O, ValueInCell::Empty, ValueInCell::Empty],
            [ValueInCell::O, ValueInCell::Empty, ValueInCell::Empty],
        ];
        assert_eq!(MoveMaker::next_state(&game, &Action::CheckWin).winner, Some(ValueInCell::O));

        game.board = [
            [ValueInCell::X, ValueInCell::Empty, ValueInCell::Empty],
            [ValueInCell::Empty, ValueInCell::X, ValueInCell::Empty],
            [ValueInCell::Empty, ValueInCell::Empty, ValueInCell::X],
        ];
        assert_eq!(MoveMaker::next_state(&game, &Action::CheckWin).winner, Some(ValueInCell::X));

        game.board = [
            [ValueInCell::X, ValueInCell::O, ValueInCell::X],
            [ValueInCell::X, ValueInCell::O, ValueInCell::O],
            [ValueInCell::O, ValueInCell::X, ValueInCell::X],
        ];
        assert_eq!(MoveMaker::next_state(&game, &Action::CheckWin).winner, Some(ValueInCell::Empty));
    }
}