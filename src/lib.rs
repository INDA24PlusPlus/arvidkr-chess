mod tests;

pub struct Board {
	board : [char; 64],
	start : i64,
        history: Vec<String>,
        castle : [bool; 4],
        en_passant_square: i64,
        last_capture: i64,
        time_now: i64,

}

impl Board {
	pub fn new() -> Self {
		Self {
			board: ['.'; 64],
			start: 1,
                        history: Vec::new(),
                        castle: [true; 4],
                        en_passant_square: -1,
                        last_capture: 0,
                        time_now: 0,
		}
	}

        fn lineboard(&self) -> String {
                let mut ret : String = String::new();
                if self.en_passant_square == -1 {
                        ret.push_str("XX");
                }
                else {
                        let temp = encode_move(0, self.en_passant_square);
                        let mut counter = 0;
                        for x in temp.chars() {
                                if counter > 1 {
                                        ret.push(x);
                                }
                                counter += 1;
                        }
                }
                if self.start == 1 {ret.push('W');}
                else {ret.push('B');}
                for i in 0..4 {
                        if self.castle[i as usize] {ret.push('1');}
                        else {ret.push('0');}
                }
                for i in 0..64 {
                        ret.push(self.board[i as usize]);
                }
                ret.push('+');
                ret.push_str(&self.last_capture.to_string());
                ret.push('+');
                ret.push_str(&self.time_now.to_string());
                return ret;
        }

        pub fn infoload(&mut self, info: String){
                let mut counter = 0;
                let mut s: String = String::new();
                s.push_str("a1");
                let mut is_valid: bool = true;
                let mut temp_s : String = String::new();
                for x in info.chars() {
                        if counter < 2 {
                                s.push(x);
                                if x == 'X' {is_valid = false;}
                        }
                        else if counter == 2 {
                                if is_valid {
                                        let yo = decode_move(s.clone());
                                        self.en_passant_square = yo.1;
                                }
                                else {self.en_passant_square = -1;}
                                if x == 'W' {self.start = 1;}
                                else {self.start = 0;}
                        } 
                        else if counter < 7 {
                                let y: usize = counter-3;
                                if x == '1' {self.castle[y] = true;}
                                else {self.castle[y] = false;}
                        }
                        else if counter < 71{
                                let y: usize = counter-7;
                                self.board[y] = x;
                        } else if counter > 71{
                                if x == '+' {
                                        let l_cap: i64 = temp_s.clone().parse().unwrap();
                                        self.last_capture = l_cap;
                                        temp_s.clear();
                                } 
                                else {
                                        temp_s.push(x);
                                }
                        }
                        counter += 1;
                }
                let c_mov: i64 = temp_s.parse().unwrap();
                self.time_now = c_mov;
                self.history.push(self.lineboard());
        }

	pub fn init_board(&mut self){
                self.start = 1;
                self.en_passant_square = -1;
                for i in 8..16 {
                        self.board[i as usize] = 'p';
                        let x = i+40;
                        self.board[x as usize] = 'P';
                }

                for i in 16..48 {
                        self.board[i as usize] = '.';
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
                self.history.clear();
                self.history.push(self.lineboard());
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

        pub fn get_board(&self) {
                let index: usize = self.history.len()-1;
                let s1 = self.history[index].clone();
                println!("{}", s1);
        }

	pub fn get_boardinfo(&self) -> String {
		let index: usize = self.history.len()-1;
		return self.history[index].clone();
	}

        pub fn switch_start(&mut self){
                self.start = 1-self.start;
        }

        pub fn make_move(&mut self, from: i64, to: i64, promto: char){
                self.time_now += 1;
                if self.board[from as usize] == 'p' && ((to-from).abs() == 7 || (to-from).abs() == 9) && to == self.en_passant_square{
                        let remov: i64 = to-8;
                        self.board[remov as usize] = '.';
                        self.last_capture = self.time_now;
                }
                else if self.board[from as usize] == 'P' && ((to-from).abs() == 7 || (to-from).abs() == 9) && to == self.en_passant_square{
                        let remov: i64 = to+8;
                        self.board[remov as usize] = '.';
                        self.last_capture = self.time_now;
                }

                if self.board[from as usize] == 'p' || self.board[from as usize] == 'P' {
                        self.last_capture = self.time_now;
                }

                if colour_of(self.board[to as usize]) != 2{
                        self.last_capture = self.time_now;
                }

                self.board[to as usize] = self.board[from as usize];
                self.board[from as usize] = '.';

                if promto != '.' {
                        if self.start == 1 {
                                if promto == 'Q' {self.board[to as usize] = 'q';}
                                else if promto == 'R' {self.board[to as usize] = 'r';}
                                else if promto == 'B' {self.board[to as usize] = 'b';}
                                else if promto == 'N' {self.board[to as usize] = 'n';}
                        }
                        else {
                                if promto == 'Q' {self.board[to as usize] = 'Q';}
                                else if promto == 'R' {self.board[to as usize] = 'R';}
                                else if promto == 'B' {self.board[to as usize] = 'B';}
                                else if promto == 'N' {self.board[to as usize] = 'N';}
                        }
                }
                
                self.start = 1-self.start;

                if (self.board[to as usize] == 'p' || self.board[to as usize] == 'P') && (from-to).abs() == 16 {
                        self.en_passant_square = from + (to-from)/2;
                } else {
                        self.en_passant_square = -1;
                }

                if self.board[to as usize] == 'k' && (from-to).abs() > 1{
                        if to == 2 {
                                self.board[0] = '.';
                                self.board[3] = 'r';
                        } 
                        else if to == 6 {
                                self.board[7] = '.';
                                self.board[5] = 'r';
                        }
                }
                else if self.board[to as usize] == 'K' && (from-to).abs() > 1{
                        if to == 58 {
                                self.board[56] = '.';
                                self.board[59] = 'R';
                        } 
                        else if to == 62 {
                                self.board[63] = '.';
                                self.board[61] = 'R';
                        }
                }

                if self.board[0] != 'r' {
                        self.castle[1] = false;
                }
                if self.board[7] != 'r' {
                        self.castle[0] = false;
                }
                if self.board[56] != 'R'{
                        self.castle[3] = false;
                }
                if self.board[63] != 'R' {
                        self.castle[2] = false;
                }
                if self.board[4] != 'k' {
                        self.castle[0] = false;
                        self.castle[1] = false;
                }
                if self.board[60] != 'K' {
                        self.castle[2] = false;
                        self.castle[3] = false;
                }

                self.history.push(self.lineboard());
        }


}
fn colour_of(piece: char) -> i64 {
        if piece == 'P' || piece == 'N' || piece == 'K' || piece == 'Q' || piece == 'B' || piece == 'R'{
                return 0;
        }
        else if piece == 'p' || piece == 'n' || piece == 'k' || piece == 'q' || piece == 'b' || piece == 'r' {
                return 1;
        }
        return 2;
}

fn is_in_bound(x: i64, y: i64) -> bool {
        if x < 0 || x > 7 || y < 0 || y > 7 {return false;}
        return true;
}

fn checking_legal(board: &mut Board, from: i64, to: i64) -> bool {

        let pers: String = board.lineboard();

        board.make_move(from, to, '.');
        board.switch_start();


        let mut king_pos = -1;

        for i in 0..64 {
                if (board.start == 1 && board.board[i as usize] == 'k') || (board.start == 0 && board.board[i as usize] == 'K') {king_pos = i;}
        }

        let mut ans : bool = true;

        if in_check(board, king_pos) {
                ans = false;
        } 

        board.infoload(pers);

        board.history.pop();
        board.history.pop();

        //println!("posii {}", king_pos);

        if king_pos == to {
                //No kings in proximity from to
                let r: i64 = to/8;
                let c: i64 = to%8;
                let mut square: Vec<i64> = vec![];
                //println!("kingpos: {}", king_pos);
                if is_in_bound(r-1, c-1) {square.push((r-1)*8+c-1);}
                if is_in_bound(r-1, c)   {square.push((r-1)*8+c);  }
                if is_in_bound(r-1, c+1) {square.push((r-1)*8+c+1);}
                if is_in_bound(r+1, c-1) {square.push((r+1)*8+c-1);}
                if is_in_bound(r+1, c)   {square.push((r+1)*8+c);  }
                if is_in_bound(r+1, c+1) {square.push((r+1)*8+c+1);}
                if is_in_bound(r, c-1)   {square.push((r)*8+c-1);  }
                if is_in_bound(r, c+1)   {square.push((r)*8+c+1);  }


                for x in square {
                        //print!("{}, ", x);
                        if (board.start == 1 && board.board[x as usize] == 'K') || (board.start == 0 && board.board[x as usize] == 'k'){
                                return false;
                        }
                
                }
                //println!("");
                //println!("");
                
        }


        return ans;
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

        if sec < 64 && sec >= 0 {
                if colour_of(board.board[fir as usize]) == 2 && colour_of(board.board[sec as usize]) == 2 && ((board.start == 1 && c == 1) || (board.start == 0 && c == 6)) {
                        ret.push(encode_move(pos, sec));
                }
        }
		
	
	let diag_l = (c+col_shift)*8+r-1;
        let mut lcolour = -1;
        if diag_l < 64 && diag_l >= 0 {
                lcolour = colour_of(board.board[diag_l as usize]);
        }
	
	if (lcolour == 1-board.start || diag_l == board.en_passant_square) && !(c+col_shift < 0 || c+col_shift >= 8 || r-1 < 0){
		if is_legal(board, c*8+r, diag_l) {
                        if c+col_shift == 7 || c+col_shift == 0 {
                                let mut p_q = encode_move(c*8+r, diag_l);
                                p_q.push('Q');
                                let mut p_n = encode_move(c*8+r, diag_l);
                                p_n.push('N');
                                let mut p_r = encode_move(c*8+r, diag_l);
                                p_r.push('R');
                                let mut p_b = encode_move(c*8+r, diag_l);
                                p_b.push('B');  
                                ret.push(p_q);
                                ret.push(p_n);
                                ret.push(p_r);
                                ret.push(p_b);                              
                        }
                        else {
			        ret.push(encode_move(c*8+r, diag_l));
                        }
                }
	}

	let diag_m = (c+col_shift)*8+r;
        let mut mcolour = -1;
        if diag_m < 64 && diag_m >= 0 {
                mcolour = colour_of(board.board[diag_m as usize]);
        }
        if mcolour == 2 && !(c+col_shift < 0 || c+col_shift >= 8 || r < 0){
                if is_legal(board, c*8+r, diag_m) {
                        if c+col_shift == 7 || c+col_shift == 0 {
                                let mut p_q = encode_move(c*8+r, diag_m);
                                p_q.push('Q');
                                let mut p_n = encode_move(c*8+r, diag_m);
                                p_n.push('N');
                                let mut p_r = encode_move(c*8+r, diag_m);
                                p_r.push('R');
                                let mut p_b = encode_move(c*8+r, diag_m);
                                p_b.push('B');  
                                ret.push(p_q);
                                ret.push(p_n);
                                ret.push(p_r);
                                ret.push(p_b);                             
                        }
                        else {
                                ret.push(encode_move(c*8+r, diag_m));
                        }
                }
        }

	let diag_r = (c+col_shift)*8+r+1;
        let mut rcolour = -1;
        if diag_r < 64 && diag_r >= 0 {
                rcolour = colour_of(board.board[diag_r as usize]);
        }

        if (rcolour == 1-board.start || diag_r == board.en_passant_square) && !(c+col_shift < 0 || c+col_shift >= 8 || r+1 >= 8){
                if is_legal( board, c*8+r, diag_r) {
                        if c+col_shift == 7 || c+col_shift == 0 {
                                let mut p_q = encode_move(c*8+r, diag_r);
                                p_q.push('Q');
                                let mut p_n = encode_move(c*8+r, diag_r);
                                p_n.push('N');
                                let mut p_r = encode_move(c*8+r, diag_r);
                                p_r.push('R');
                                let mut p_b = encode_move(c*8+r, diag_r);
                                p_b.push('B');  
                                ret.push(p_q);
                                ret.push(p_n);
                                ret.push(p_r);
                                ret.push(p_b);                             
                        }
                        else {
                                ret.push(encode_move(c*8+r, diag_r));
                        }
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

fn knight_moves(board: &mut Board, pos: i64) -> Vec<String> {
        let r: i64 = pos/8;
        let c: i64 = pos%8;
        let mut ret: Vec<String> = Vec::new();
        //up right
        if r+2 < 8 && c+1 < 8 {
                if is_legal(board, pos, (r+2)*8 + c+1) {
                        ret.push(encode_move(pos, (r+2)*8+c+1));
                }
        }
        //up left
        if r+2 < 8 && c-1 >= 0 {
                if is_legal(board, pos, (r+2)*8 + c-1) {
                        ret.push(encode_move(pos, (r+2)*8+c-1));
                }
        }
        //down right
        if r-2 >= 0 && c+1 < 8 {
                if is_legal(board, pos, (r-2)*8 + c+1) {
                        ret.push(encode_move(pos, (r-2)*8+c+1));
                }
        }
        //down left
        if r-2 >= 0 && c-1 >= 0 {
                if is_legal(board, pos, (r-2)*8 + c-1) {
                        ret.push(encode_move(pos, (r-2)*8+c-1));
                }
        }
        //left up
        if r+1 < 8 && c-2 >= 0 {
                if is_legal(board, pos, (r+1)*8 + c-2) {
                        ret.push(encode_move(pos, (r+1)*8+c-2));
                }
        }
        //left down
        if r-1 >= 0 && c-2 >= 0 {
                if is_legal(board, pos, (r-1)*8 + c-2) {
                        ret.push(encode_move(pos, (r-1)*8+c-2));
                }
        }
        //right up
        if r+1 < 8 && c+2 < 8 {
                if is_legal(board, pos, (r+1)*8 + c+2) {
                        ret.push(encode_move(pos, (r+1)*8+c+2));
                }
        }
        //right down
        if r-1 >= 0 && c+2 < 8 {
                if is_legal(board, pos, (r-1)*8 + c+2) {
                        ret.push(encode_move(pos, (r-1)*8+c+2));
                }
        }
        return ret;
}

pub fn print_history(board: &mut Board){
        for x in board.history.clone() {
                println!("{}", x);
        }
}

fn in_check(board: &mut Board, whereking: i64) -> bool {
        board.switch_start();
        let v = all_moves(board, false);
        let s = encode_move(0, whereking);
        let mut t2: String = String::new();
        t2.push(s.chars().nth(2).expect("REASON"));
        t2.push(s.chars().nth(3).expect("REASON"));

        let mut illegal = false;

        for x in v {
                let mut t1: String = String::new();
                t1.push(x.chars().nth(2).expect("REASON"));
                t1.push(x.chars().nth(3).expect("REASON"));
                if t1 == t2 {
                        illegal = true;
                        break;
                }
        }
        board.switch_start();
        return illegal;
}

fn legal_castle(board: &mut Board, pos: i64) -> bool {
        let col: i64 = 1-(pos/8)/7;
        if col == 1 {
                board.board[pos as usize] = 'k';
                let ret = !in_check(board, pos);
                board.board[pos as usize] = '.';
                return ret;
        } 
        board.board[pos as usize] = 'K';
        let ret = !in_check(board, pos);
        board.board[pos as usize] = '.';
        return ret;
}

fn king_moves(board: &mut Board, pos: i64) -> Vec<String> {
        let r: i64 = pos/8;
        let c: i64 = pos%8;
        let mut ret: Vec<String> = Vec::new();
        if r > 0 { //r-1
                if c > 0 {if is_legal(board, pos, (r-1)*8 + c-1) {ret.push(encode_move(pos, (r-1)*8 + c-1));}}
                if c < 7 {if is_legal(board, pos, (r-1)*8 + c+1) {ret.push(encode_move(pos, (r-1)*8 + c+1));}}
                if is_legal(board, pos, (r-1)*8 + c){ret.push(encode_move(pos, (r-1)*8+c));}
        }
        if r < 7 {
                if c > 0 {if is_legal(board, pos, (r+1)*8 + c-1) {ret.push(encode_move(pos, (r+1)*8 + c-1));}}
                if c < 7 {if is_legal(board, pos, (r+1)*8 + c+1) {ret.push(encode_move(pos, (r+1)*8 + c+1));}}
                if is_legal(board, pos, (r+1)*8 + c){ret.push(encode_move(pos, (r+1)*8+c));}
        }
        if c > 0 {if is_legal(board, pos, r*8 + c-1) {ret.push(encode_move(pos, r*8 + c-1));}}
        if c < 7 {if is_legal(board, pos, r*8 + c+1) {ret.push(encode_move(pos, r*8 + c+1));}}

        if pos == 4 {
                if board.castle[1] && board.board[3] == '.' && board.board[2] == '.' && board.board[1] == '.' && board.board[0] == 'r'{
                        if !in_check(board, 4) && legal_castle(board, 3) && legal_castle(board, 2) {
                                ret.push("e1c1".to_string());
                        }
                }
                if board.castle[0] && board.board[5] == '.' && board.board[6] == '.' && board.board[7] == 'r' {
                        if !in_check(board, 4) && legal_castle(board, 5) && legal_castle(board, 6) {
                                ret.push("e1g1".to_string());
                        }
                }
        }
        else if pos == 60 {
                if board.castle[3] && board.board[59] == '.' && board.board[58] == '.' && board.board[57] == '.' && board.board[56] == 'R' {
                        if !in_check(board, 60) && legal_castle(board, 59) && legal_castle(board, 58) {
                                ret.push("e8c8".to_string());
                        }
                }
                if board.castle[2] && board.board[61] == '.' && board.board[62] == '.' && board.board[63] == 'R' {
                        if !in_check(board, 60) && legal_castle(board, 61) && legal_castle(board, 62) {
                                ret.push("e8g8".to_string());
                        }
                }
        }


        return ret;
}

fn queen_moves(board: &mut Board, pos: i64) -> Vec<String> {
	let mut ret: Vec<String> = Vec::new();
        let bishmoves = bishop_moves(board, pos);
        let roomoves = rook_moves(board, pos);
        for x in bishmoves {
                ret.push(x);
        }
        for x in roomoves {
                ret.push(x);
        }
        return ret;
}


fn all_moves(board: &mut Board, king: bool) -> Vec<String> {
	let mut ret: Vec<String> = Vec::new();
	for i in 0..64 {
		if board.start == 1 {
//			println!("White?");
			if board.board[i as usize] == 'p'{
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
			else if board.board[i as usize] == 'r' {
				let temp_vec = rook_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
			} 
                        else if board.board[i as usize] == 'q' {
                                let temp_vec = queen_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
                        }
                        else if board.board[i as usize] == 'n' {
                                let temp_vec = knight_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
                        }
                        else if board.board[i as usize] == 'k' && king {
                                let temp_vec = king_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
                        }
		}
		else {
//			println!("Still black");
			if board.board[i as usize] == 'P'{
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
			else if board.board[i as usize] == 'R' {
				let temp_vec = rook_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
			}
                        else if board.board[i as usize] == 'Q' {
				let temp_vec = queen_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
			}
                        else if board.board[i as usize] == 'N' {
				let temp_vec = knight_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
			}
                        else if board.board[i as usize] == 'K' && king {
				let temp_vec = king_moves(board, i);
				for s in temp_vec {
					ret.push(s);
				}
			}

		}
		
	}	

        return ret;
}

pub fn filtered_moves(board: &mut Board) -> Vec<String> {
        let ret = all_moves(board, true);
        let mut real_ret: Vec<String> = Vec::new();

        for x in ret {
                let duplo = decode_move(x.clone());
                if checking_legal(board, duplo.0, duplo.1){
                        real_ret.push(x);
                }
        }

	return real_ret;
}

pub fn print_all_moves(board: &mut Board){
	let v = filtered_moves(board);
        println!("{}", v.len());
	for s in v {
		println!("{}", s);
	}
}

pub fn make_move(board: &mut Board, movi: String){
        let mut r_move: String = String::new();
        if movi == "O-O" {
                if board.start == 1 {
                        r_move.push_str("e1g1");
                } 
                else {
                        r_move.push_str("e8g8");
                }
        }
        else if movi == "O-O-O" {
                if board.start == 1 {
                        r_move.push_str("e1c1");
                }
                else {
                        r_move.push_str("e8c8");
                }
        }
        else {
                r_move.push_str(&movi);
        }
        
	let decoded_tuple = decode_move(r_move.clone());
	let from = decoded_tuple.0;
	let to = decoded_tuple.1;
	//println!("{}, {}", from, to);
	let allmoves = all_moves(board, true);
	let mut in_allmoves = false;
	for pos in allmoves {
		if pos == r_move {
			in_allmoves = true;
		}
	}	

	if in_allmoves {
                if r_move.chars().count() == 4 {
                        board.make_move(from, to, '.');
                } 
                else {
                        board.make_move(from, to, r_move.chars().nth(4).expect("REASON"));
                }
	} 
	else {
		println!("Invalid Move!");
	}

}

pub fn load_from_info(board: &mut Board, info: String){
        board.infoload(info);
}

fn switchcols(piece: char) -> char {
        if piece == 'p' {return 'P';}
        if piece == 'n' {return 'N';}
        if piece == 'b' {return 'B';}
        if piece == 'r' {return 'R';}
        if piece == 'q' {return 'Q';}
        if piece == 'k' {return 'K';}
        if piece == 'P' {return 'p';}
        if piece == 'N' {return 'n';}
        if piece == 'B' {return 'b';}
        if piece == 'R' {return 'r';}
        if piece == 'Q' {return 'q';}
        return 'k';
}

fn fen_to_boardinfo(fen: Vec<String>) -> String {
        let boardie: String = fen[0].clone();
        let mut s1: String = String::new();
        let mut s2: String = String::new();
        let mut s3: String = String::new();
        let mut s4: String = String::new();
        let mut s5: String = String::new();
        let mut s6: String = String::new();
        let mut s7: String = String::new();
        let mut s8: String = String::new();

        let mut s: String = String::new();
        let mut linechanges = 0;

        for x in boardie.chars() {
                if x == '/' {
                        linechanges += 1;
                        continue;
                }
                if colour_of(x) != 2 {
                        if linechanges == 0 {s1.push(switchcols(x));}
                        if linechanges == 1 {s2.push(switchcols(x));}
                        if linechanges == 2 {s3.push(switchcols(x));}
                        if linechanges == 3 {s4.push(switchcols(x));}
                        if linechanges == 4 {s5.push(switchcols(x));}
                        if linechanges == 5 {s6.push(switchcols(x));}
                        if linechanges == 6 {s7.push(switchcols(x));}
                        if linechanges == 7 {s8.push(switchcols(x));}
                } 
                else {
                        let m: i64 = x.to_string().parse().unwrap();
                        let mut temps: String = String::new();
                        for _ in 0..m{
                                temps.push('.');
                        }
                        if linechanges == 0 {s1.push_str(&temps);}
                        if linechanges == 1 {s2.push_str(&temps);}
                        if linechanges == 2 {s3.push_str(&temps);}
                        if linechanges == 3 {s4.push_str(&temps);}
                        if linechanges == 4 {s5.push_str(&temps);}
                        if linechanges == 5 {s6.push_str(&temps);}
                        if linechanges == 6 {s7.push_str(&temps);}
                        if linechanges == 7 {s8.push_str(&temps);}
                }
        }

        s.push_str(&s8);
        s.push_str(&s7);
        s.push_str(&s6);
        s.push_str(&s5);
        s.push_str(&s4);
        s.push_str(&s3);
        s.push_str(&s2);
        s.push_str(&s1);
 
        let mut starti: String = String::new();
        let starting: String = fen[1].clone();
        if starting == "w" {starti.push('W');} 
        else {starti.push('B');}

        let castling: String = fen[2].clone();
        let mut castlel: String = String::new();
        let mut w_k = false;
        let mut w_q = false;
        let mut b_k = false;
        let mut b_q = false;

        for x in castling.chars() {
                if x == 'K' {w_k = true;}
                if x == 'Q' {w_q = true;}
                if x == 'k' {b_k = true;}
                if x == 'q' {b_q = true;}
        }
        if w_k {castlel.push('1');}
        else  {castlel.push('0');}
        if w_q {castlel.push('1');}
        else  {castlel.push('0');}
        if b_k {castlel.push('1');}
        else  {castlel.push('0');}
        if b_q {castlel.push('1');}
        else  {castlel.push('0');}

        let enpassant = fen[3].clone();
        let capmov = fen[4].clone();
        let halfmov = fen[5].clone();

        let mut ret: String = String::new();
        if enpassant == "-" {
                ret.push_str("XX");
        } 
        else {
                ret.push_str(&enpassant);
        }
        ret.push_str(&starti);
        ret.push_str(&castlel);
        ret.push_str(&s);
        ret.push('+');
        ret.push_str(&capmov);
        ret.push('+');
        ret.push_str(&halfmov);
        return ret;
}

pub fn load_from_fen(board: &mut Board, info: Vec<String>){
        let ninfo = fen_to_boardinfo(info);
        println!("{}", ninfo);
        load_from_info(board, ninfo);
}

pub fn fenstring_to_vec(fen: String) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        let mut temp_s: String = String::new();
        for x in fen.chars() {
                if x == ' '{
                        ret.push(temp_s.clone());
                        temp_s.clear();
                }
                else {
                        temp_s.push(x);
                }
        }
        ret.push(temp_s.clone());
        return ret;
}

pub fn get_amount_moves(board: &mut Board) -> i64 {
        let v = filtered_moves(board);
        return v.len() as i64;
}

fn important_parts(s: String) -> String {
        let mut ret: String = String::new();
        for i in 0..71 {
                ret.push(s.chars().nth(i).unwrap());
        }
        return ret;
}

pub fn is_over(board: &mut Board) -> i64 {
        let testing = filtered_moves(board);
        for x in board.history.clone() {
                let mut counter = 0;
                for y in board.history.clone() {
                        if important_parts(x.clone()) == important_parts(y.clone()) {counter += 1;}
                }
                if counter >= 3 {
                        return 3; //3 fold rep
                }
        }

        let temp: i64 = board.time_now;
        if temp - board.last_capture >= 50 {
                return 4; //50 move rule
        }

        if testing.len() != 0 {
                return 0;
        }

        let mut king_pos = -1;
        for i in 0..64 {
                if (board.start == 1 && board.board[i as usize] == 'k') || (board.start == 0 && board.board[i as usize] == 'K'){
                        king_pos = i;
                }
        }
        if in_check(board, king_pos) {
                return 1; //Checkmate
        }
        return 2;
}

pub fn get_start(board: &mut Board) -> i64 {
        return board.start;
}
