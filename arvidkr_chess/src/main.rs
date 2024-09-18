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
		println!("Do something!, 'makemove [move]', 'printboard', 'reset', 'printmoves', 'boardinfo', 'terminate', 'printhistory', 'infoload [board_info]'");
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
		else {
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
		println!("");
		println!("");
		println!("");
	}
}
