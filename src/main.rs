use arvidkr_chess;
use std::io;
use std::io::BufRead;

fn main(){
	let mut board : arvidkr_chess:: Board = arvidkr_chess::Board::new();
	let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
	board.init_board();
	board.print_board();
	loop {
		println!("Do something!, 'makemove [move]', 'printboard', 'reset', 'printmoves', 'boardinfo', 'terminate', 'printhistory', 'fenload [FEN]', 'infoload [board_info]'");
		let line = iterator.next().unwrap().unwrap();
		let sv1:Vec<&str> = line.split_whitespace().collect();
		if sv1.len() == 1 {
			let mut terminated = false;
			for x in sv1.iter() {
				if *x == "printboard" {board.print_board();}
				else if *x == "reset" {board.init_board();}
				else if *x == "printmoves" {
					arvidkr_chess::print_all_moves(&mut board);
				}
				else if *x == "boardinfo" {
					board.get_board();
				}
				else if *x == "terminate" {terminated = true;} 
				else if *x == "printhistory" {arvidkr_chess::print_history(&mut board);}
				else {
					println!("Invalid!");
				}
			}
			if terminated {break;}
		} 
		else if sv1.len() == 2{
			let mut first = -1;
			for x in sv1.iter(){
				if first == 0 {
					arvidkr_chess::make_move(&mut board, x.to_string());
				}
				if first == 1 {
					arvidkr_chess::load_from_info(&mut board, x.to_string());
				}
				if *x == "makemove" {first = 0;}
				else if *x == "infoload" {first = 1;}
				
			}
		} 
		else {
			let mut counter = 0;
			let mut v : Vec<String> = vec![];
			for x in sv1.iter(){
				if counter >= 1 {
					v.push((*x).to_string());
				}

				counter += 1;
			}
			arvidkr_chess::load_from_fen(&mut board, v.clone());
		}
		println!("");
		let state = arvidkr_chess::is_over(&mut board);
		if state == 1 {
			println!("Checkmate!");
			if arvidkr_chess::get_start(&mut board) == 0 {
				println!("WHITE WINS");
			}
			else {
				println!("BLACK WINS");
			}
			break;
		}
		else if state == 2 {
			println!("Stalemate!");
			println!("DRAW");
			break;
		}
		else if state == 3 {
			println!("Threefold repetition!");
			println!("DRAW");
			break;
		}
		else if state == 4 {
			println!("50 move rule");
			println!("DRAW");
		} 

		println!("");
		println!("");
	}
}
