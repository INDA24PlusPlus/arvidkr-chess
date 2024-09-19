#[cfg(test)]
mod chess_tests {
    use crate::*;
    //use arvidkr_chess::fenstring_to_vec;

    #[test]
    fn checkmate_tests() {
        let mut board: Board = Board::new();


        load_from_fen(&mut board, fenstring_to_vec("3b1q1q/1N2PRQ1/rR3KBr/B4PP1/2Pk1r1b/1P2P1N1/2P2P2/8 b - - 0 0".to_string()));
        assert_eq!(is_over(&mut board), 1);
        assert_eq!(get_amount_moves(&mut board), 0);



        load_from_fen(&mut board, fenstring_to_vec("4rk2/1bp3p1/5pN1/p7/2B1r3/1P4P1/P4P1P/3R2K1 b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 1);
        assert_eq!(get_amount_moves(&mut board), 0);



        load_from_fen(&mut board, fenstring_to_vec("5rkr/8/6Q1/8/8/8/8/6K1 b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 1);
        assert_eq!(get_amount_moves(&mut board), 0);



        load_from_fen(&mut board, fenstring_to_vec("3bkr2/R5N1/8/8/8/8/8/7K b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 1);
        assert_eq!(get_amount_moves(&mut board), 0);



        load_from_fen(&mut board, fenstring_to_vec("3qr3/2p1k1Q1/8/2N1P3/8/8/8/7K b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 1);
        assert_eq!(get_amount_moves(&mut board), 0);



        load_from_fen(&mut board, fenstring_to_vec("rn1q1b1r/ppp1kBpp/3p4/4N1B1/8/2P5/PPP2PPP/R2b1RK1 b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 1);
        assert_eq!(get_amount_moves(&mut board), 0);



        load_from_fen(&mut board, fenstring_to_vec("3q1b1r/4kBpp/3p4/3NN3/8/8/5PPP/6K1 b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 1);
        assert_eq!(get_amount_moves(&mut board), 0);



        load_from_fen(&mut board, fenstring_to_vec("1r1q1r1k/6pp/5pN1/8/2B5/8/1PP5/2K4R b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 1);
        assert_eq!(get_amount_moves(&mut board), 0);


    }


    #[test]
    fn stalemate_tests() {
        let mut board: Board = Board::new();


        load_from_fen(&mut board, fenstring_to_vec("8/8/8/8/8/3k4/3p4/3K4 w - - 2 3".to_string()));
        assert_eq!(is_over(&mut board), 2);


        load_from_fen(&mut board, fenstring_to_vec("k7/2Q5/1K6/8/8/8/8/8 b - - 0 1".to_string()));
        assert_eq!(is_over(&mut board), 2);


        load_from_fen(&mut board, fenstring_to_vec("k7/P7/K7/8/8/8/8/8 b - - 2 5".to_string()));
        assert_eq!(is_over(&mut board), 2);


        load_from_fen(&mut board, fenstring_to_vec("k7/8/1K1Q4/8/8/8/8/8 b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 2);


        load_from_fen(&mut board, fenstring_to_vec("8/8/8/8/8/6K1/6R1/7k b - - 1 1".to_string()));
        assert_eq!(is_over(&mut board), 2);

    }

    #[test]
    fn check_possible_moves() {
        let mut board:  Board = Board::new();

        load_from_fen(&mut board, fenstring_to_vec("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()));
        assert_eq!(get_amount_moves(&mut board), 20);


        load_from_fen(&mut board, fenstring_to_vec("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string()));
        assert_eq!(get_amount_moves(&mut board), 48);


        load_from_fen(&mut board, fenstring_to_vec("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0".to_string()));
        assert_eq!(get_amount_moves(&mut board), 14);


        load_from_fen(&mut board, fenstring_to_vec("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string()));
        assert_eq!(get_amount_moves(&mut board), 6);


        load_from_fen(&mut board, fenstring_to_vec("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string()));
        assert_eq!(get_amount_moves(&mut board), 44);


        load_from_fen(&mut board, fenstring_to_vec("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".to_string()));
        assert_eq!(get_amount_moves(&mut board), 46);


        load_from_fen(&mut board, fenstring_to_vec("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string()));
        assert_eq!(get_amount_moves(&mut board), 14);


        load_from_fen(&mut board, fenstring_to_vec("5b1k/8/8/2pP4/8/K7/8/8 w - c6 0 1".to_string()));
        assert_eq!(get_amount_moves(&mut board), 5);


        load_from_fen(&mut board, fenstring_to_vec("4r1k1/p4pp1/2n2n1B/2b5/N6Q/P2q1N2/1r4PP/R4R1K b - - 1 23".to_string()));
        assert_eq!(get_amount_moves(&mut board), 79);


        load_from_fen(&mut board, fenstring_to_vec("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string()));
        assert_eq!(get_amount_moves(&mut board), 48);

    }
}
