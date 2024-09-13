use arvidkr_chess;

fn main(){
	let mut board : arvidkr_chess:: Board = arvidkr_chess::Board::new();
	board.init_board();
	board.print_board();
	arvidkr_chess::make_move(&mut board, "d2d4".to_string());
	board.print_board(); 
}
