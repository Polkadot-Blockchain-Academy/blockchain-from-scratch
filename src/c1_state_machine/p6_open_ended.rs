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
use std::io;
enum ValueInCell{
	X,
	O
}
pub struct MoveMaker {
	board: [[ValueInCell; 3]; 3],
	player1: ValueInCell,
	player2: ValueInCell,
	is_next_player1: bool,
	winner: Option<ValueInCell>
}

pub enum Action {
	make_move(i32,i32),
	check_win,
}

impl StateMachine for MoveMaker {
	type State = MoveMaker;
	type Transition = Action;

	fn next_state(starting: &Self::State, t: &Self::Transition) -> Self::State {
		match t{
			&Action::print_board(board: &[[char; 3]; 3]) => {
				for i in 0..3{
					for j in 0..3{
						print!("{}", board[i][j]);
					}
					println!();
				}

			&Action::make_move(board: &mut[[char; 3]; 3], player: &mut char) => {
				let mut x = String::new();
				let mut y = String::new();
				println!("Please enter the row for your move:");
				io::stdin().read_line(&mut x).unwrap();
				println!("Please enter the column for your move:");
				io::stdin().read_line(&mut y).unwrap();
				
				let x: usize = x.trim().parse().unwrap();
				let y: usize = y.trim().parse().unwrap();
			
				if x >= 3 || y >= 3{
					println!("Invalid move!");
					make_move(board, player);
				}
				if board[x][y] != 'X' && board[x][y] != 'O'{
					board[x][y] = *player;
			
					if *player == 'X'{
						*player = 'O';
					}
					else{
						*player = 'X';
					}
				}
				else{
					println!("Invalid move!");
					make_move(board, player);
				}
			}

			&Action::check_win(board: &[[char; 3]; 3], player: char) => {
				for i in 0..3{
					if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
						return true;
					}
				}
				for i in 0..3 {
					if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
						return true;
					}
				}
				if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
					return true;
				}
				if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
					return true;
				}
				return false;
			}

		}
	}
}

#[test]
fn playing(){
	let mut board = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
	let mut player = 'X';

	loop{
		print_board(&board);
		make_move(&mut board, &mut player);

		if check_win(&board, player){
			println!("Player {} wins!", player);
			break;
		}
	}
}

fn print_board(board: &[[char; 3]; 3]){
	for i in 0..3{
		for j in 0..3{
			print!("{}", board[i][j]);
		}
		println!();
	}
}
fn make_move(board: &mut[[char; 3]; 3], player: &mut char){
	let mut x = String::new();
	let mut y = String::new();
	println!("Please enter the row for your move:");
	io::stdin().read_line(&mut x).unwrap();
	println!("Please enter the column for your move:");
	io::stdin().read_line(&mut y).unwrap();
	
	let x: usize = x.trim().parse().unwrap();
	let y: usize = y.trim().parse().unwrap();

	if x >= 3 || y >= 3{
		println!("Invalid move!");
		make_move(board, player);
	}
	if board[x][y] != 'X' && board[x][y] != 'O'{
		board[x][y] = *player;

		if *player == 'X'{
			*player = 'O';
		}
		else{
			*player = 'X';
		}
	}
	else{
		println!("Invalid move!");
		make_move(board, player);
	}
}

fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
	for i in 0..3{
		if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
			return true;
		}
	}
	for i in 0..3 {
		if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
			return true;
		}
	}
	if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
		return true;
	}
	if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
		return true;
	}
	return false;
}