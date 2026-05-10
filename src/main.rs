use std::env;
struct Item {
    letter: char,
    //indicates what words might be "on" the letter
    owners: Vec<u8>,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    // cargo run -- (word file) (target file) (special args)


}

fn assemble_board(width: usize, words: Vec<String>, flag: bool) -> Vec<Vec<Item>>{
    let width = 3;
    let mut board: Vec<Vec<Item>> = Vec::new();
    for i in 0..width {
        let mut row: Vec<Item> = Vec::new();
        for j in 0..width {
            let mut item = Item {
                letter: '/',
                owners: [].to_vec(),
            };
            row.push(item);
        }
        board.push(row);
    }

    for i in 0..width { 
        let s = format!("{}{}{}", board[i][0].letter, board[i][1].letter, board[i][2].letter);
        println!("{s}");
    }

    board
}
