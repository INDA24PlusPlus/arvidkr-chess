pub struct Board {
	board : [char; 64],
	start : i64,

}

impl Board {
	pub fn new() -> Self {
		Self {
			board: ['.'; 64],
			start: 1,
		}
	}

	pub fn init_board(&mut self){
                for i in 8..16 {
                        self.board[i as usize] = 'p';
                        let x = i+40;
                        self.board[x as usize] = 'P';
                }

                self.board[56] = 'R'; self.board[63] = 'R';
                self.board[0] = 'r'; self.board[7] = 'r';
                self.board[57] = 'N'; self.board[62] = 'N';
                self.board[1] = 'n'; self.board[6] = 'n';
                self.board[58] = 'B'; self.board[61] = 'B';
                self.board[2] = 'b'; self.board[5] = 'b';
                self.board[3] = 'q';
                self.board[59] = 'Q';
                self.board[4] = 'k';
                self.board[60] = 'K';
        }

	pub fn print_board(&self){
                if self.start == 1 {
                        println!("WHITE TO MOVE");
                }
                else {
                        println!("BLACK TO MOVE");
                }
                for i in 0..8 {
                        for j in 0..8 {
                                let x = i*8 + j;
                                print!("{}", self.board[x as usize]);
                        }
			print!("\n");
                }
                println!();
        }

        pub fn make_move(&mut self, from: i64, to: i64){
                self.start = 1-self.start;
                self.board[to as usize] = self.board[from as usize];
                self.board[from as usize] = '.';
        }


}
fn colour_of(piece: char) -> i64 {
        if piece == '.' {
                return 2;
        }
        else if piece == 'p' || piece == 'n' || piece == 'k' || piece == 'q' || piece == 'b' || piece == 'b' || piece == 'r' {
                return 1;
        }
        return 0;
}


fn is_legal(board: &mut Board, from: i64, to: i64) -> bool {
        if to < 0 || to > 63 {
                return false;   
        }
        let colour1 = colour_of(board.board[from as usize]);
        let colour2 = colour_of(board.board[to as usize]);

        if colour1 == 2 || colour1 == colour2 {
                return false;
        }
        return true; 
}


fn column_decode(cx: char) -> i64 {
	if cx == 'a' {
		return 0;
	}
	if cx == 'b' {
		return 1;
	}
	if cx == 'c' {
		return 2;
	}
	if cx == 'd' {
		return 3;
	}
	if cx == 'e' {
		return 4;
	}
	if cx == 'f' {
		return 5;
	}
	if cx == 'g' {
		return 6;
	}
	return 7;
}

fn row_decode(cx: char) -> i64 {
	if cx == '1' {
                return 0;
        }
        if cx == '2' {
                return 1;
        }
        if cx == '3' {
                return 2;
        }
        if cx == '4' {
                return 3;
        }
        if cx == '5' {
                return 4;
        }
        if cx == '6' {
                return 5;
        }
        if cx == '7' {
                return 6;
        }
        return 7;
	
}


fn decode_move(movi: String) -> (i64, i64){
	let x = column_decode(movi.chars().nth(0).unwrap()) + 8*row_decode(movi.chars().nth(1).unwrap());
	let y = column_decode(movi.chars().nth(2).unwrap()) + 8*row_decode(movi.chars().nth(3).unwrap());

	return (x, y);
}



fn encode_move(from: i64, to: i64) -> String {
	let column_encoding: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
	let row_encoding: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8'];

	let mut ret: String = String::new();
	let mut i: i64 = from%8;
	let c1 = column_encoding[i as usize];
	i = from/8;
	let r1 = row_encoding[i as usize];
	i = to%8;
	let c2 = column_encoding[i as usize];
	i = to/8;
	let r2 = row_encoding[i as usize];
	ret.push(c1);
	ret.push(r1);
	ret.push(c2);
	ret.push(r2);
	return ret;
}

fn pawn_moves(board: &mut Board, pos: i64) -> Vec<String> {
	let mut ret: Vec<String> = Vec::new();
	let col_shift = 2*board.start-1;
	let r: i64 = pos%8;
	let c: i64 = pos/8;
	let fir = pos+8*col_shift;
	let sec = pos+16*col_shift;
	
	if colour_of(board.board[fir as usize]) == 2 && colour_of(board.board[sec as usize]) == 2 && ((board.start == 1 && c == 1) || (board.start == 0 && c == 6)) {
		ret.push(encode_move(pos, sec));
	}
		
	
	let diagL = (c+col_shift)*8+r-1;
	let lcolour = colour_of(board.board[diagL as usize]);
	if lcolour == 1-board.start && !(c+col_shift < 0 || c+col_shift >= 8 || r-1 < 0){
		if is_legal(board, c*8+r, diagL) {
			ret.push(encode_move(c*8+r, diagL));
		}
	}

	let diagM = (c+col_shift)*8+r;
	let mcolour = colour_of(board.board[diagM as usize]);
        if mcolour == 2 && !(c+col_shift < 0 || c+col_shift >= 8 || r < 0){
                if is_legal(board, c*8+r, diagM) {
                        ret.push(encode_move(c*8+r, diagM));
                }
        }

	let diagR = (c+col_shift)*8+r+1;
	let rcolour = colour_of(board.board[diagR as usize]);
        if rcolour == 1-board.start && !(c+col_shift < 0 || c+col_shift >= 8 || r+1 >= 8){
                if is_legal( board, c*8+r, diagR) {
                        ret.push(encode_move(c*8+r, diagR));
                }
        }
	return ret;
}

fn rook_moves(board: &mut Board, pos: i64) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let c: i64 = pos%8;
        let r: i64 = pos/8;
        let mut left: bool = false;
        let mut right: bool = false;
        let mut up: bool = false;
        let mut down: bool = false;

        for i in 1..8 {
                //left up, c--, r++
                if c-i >= 0 && !left {
                        let x = r*8 + c-i;
                        let col = colour_of(board.board[x as usize]);
                        if col == board.start {
                                left = true;
                        }
                        else if col != 2 {
                                ret.push(encode_move(pos, x));
                                left = true;
                        } else {
                                ret.push(encode_move(pos, x));
                        }
                }

                //Right up, c++, r++
                if c+i < 8 && !right {
                        let x = r*8 + c+i;
                        let col = colour_of(board.board[x as usize]);
                        if col == board.start {
			        right = true;
                        }
                        else if col != 2 {
                                ret.push(encode_move(pos, x));
                                right = true;
                        } else {
                                ret.push(encode_move(pos, x));
                        }
                }

                //Left down, c--, r--
                if r-i >= 0 && !up {
                        let x = (r-i)*8 + c;
                        let col = colour_of(board.board[x as usize]);
                        if col == board.start {
                                up = true;
                        }
                        else if col != 2 {
                                ret.push(encode_move(pos, x));
                                up = true;
                        } else {
                                ret.push(encode_move(pos, x));
                        }
                }

                //Right down, c++, r--
                if r+i < 8 && !down {
                        let x = (r+i)*8 + c;
                        let col = colour_of(board.board[x as usize]);
                        if col == board.start {
                                down = true;
                        }
                        else if col != 2 {
			        ret.push(encode_move(pos, x));
                                down = true;
                        } else {
                                ret.push(encode_move(pos, x));
                        }
                }




        }

        return ret;
}


fn bishop_moves(board: &mut Board, pos: i64) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let c: i64 = pos%8;
        let r: i64 = pos/8;
        let mut left_up: bool = false;
	let mut right_up: bool = false;
	let mut left_down: bool = false;
	let mut right_down: bool = false;

	for i in 1..8 {
		//left up, c--, r++
		if c-i >= 0 && r+i < 8 && !left_up {
			let x = (r+i)*8 + c-i;
			let col = colour_of(board.board[x as usize]);
			if col == board.start {
				left_up = true;
			} 
			else if col != 2 {
				ret.push(encode_move(pos, x));
				left_up = true;
			} else {
				ret.push(encode_move(pos, x));
			}	
		}
	
		//Right up, c++, r++
		if c+i < 8 && r+i < 8 && !right_up {
                        let x = (r+i)*8 + c+i;
                        let col = colour_of(board.board[x as usize]);
                        if col == board.start {
                                right_up = true;
                        }    
                        else if col != 2 { 
                                ret.push(encode_move(pos, x));
                                right_up = true;
                        } else {
                                ret.push(encode_move(pos, x));
                        }
                }

		//Left down, c--, r--
		if c-i >= 0 && r-i >= 0 && !left_down {
                        let x = (r-i)*8 + c-i;
                        let col = colour_of(board.board[x as usize]);
                        if col == board.start {
                                left_down = true;
                        }    
                        else if col != 2 { 
                                ret.push(encode_move(pos, x));
                                left_down = true;
                        } else {
                                ret.push(encode_move(pos, x));
                        }
                }

		//Right down, c++, r--
		if c+i < 8 && r-i >= 0 && !right_down {
                        let x = (r-i)*8 + c+i;
                        let col = colour_of(board.board[x as usize]);
                        if col == board.start {
                                right_down = true;
                        }    
                        else if col != 2 { 
                                ret.push(encode_move(pos, x));
                                right_down = true;
                        } else {
                                ret.push(encode_move(pos, x));
                        }
                }



	
	}

	return ret;
}

fn all_moves(board: &mut Board) -> Vec<String> {
	let mut ret: Vec<String> = Vec::new();
	for i in 0..64 {
		if board.start == 1 {
//			println!("White?");
			if (board.board[i as usize] == 'p'){
				let temp_vec = pawn_moves(board, i);
				for s in temp_vec {
                                	ret.push(s);
                        	}

			} 
			else if board.board[i as usize] == 'b'{
				let temp_vec = bishop_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
			}
		}
		else {
//			println!("Still black");
			if (board.board[i as usize] == 'P'){
		//		print!("{}", board.board[i as usize]);
				let temp_vec = pawn_moves(board, i);
				for s in temp_vec {
        	                        ret.push(s);
	                        }

			} 
			else if board.board[i as usize] == 'B'{
                                let temp_vec = bishop_moves(board, i);
                                for s in temp_vec {
                                        ret.push(s);
                                }
                        }

		}
		
	}	
	return ret;

}

pub fn print_all_moves(board: &mut Board){
	let v = all_moves(board);
	for s in v {
		println!("{}", s);
	}	
}


pub fn make_move(board: &mut Board, movi: String){
	let decoded_tuple = decode_move(movi.clone());
	let from = decoded_tuple.0;
	let to = decoded_tuple.1;
	println!("{}, {}", from, to);
	let mut allmoves: Vec<String> = Vec::new();
	let mut in_allmoves = false;
	for pos in allmoves {
		in_allmoves |= pos == movi;
	}	

	if in_allmoves {
		board.make_move(from, to);
	} 
	else {
		println!("Invalid Move!");
	}
	board.make_move(from, to);

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
