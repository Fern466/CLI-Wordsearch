use std::env;
use std::fs;
struct Item {
    letter: char,
    //indicates what words might be "on" the letter
    owners: Vec<u8>,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    // cargo run -- (word file) (target file) (special args)
    let words: Vec<String> = read_input(args[1].clone());

    let width = 5;
    let board = assemble_board(width, words, false);
    for i in 0..width { 
        let mut s = String::new();
        for j in 0..width{
            s.push(board[i][j].letter);
        }
        println!("{s}");
    }
}

fn read_input(s: String) -> Vec<String>{
    let mut words: Vec<String> = Vec::new();
    let s = s + ".txt";
    let contents = fs::read_to_string(s).expect("Couldn't read file");
    let contents: &[u8] = contents.as_bytes(); 
    let mut temp: Vec<u8> = Vec::new();    
    let mut i = 0;
    while i < contents.len() - 1{
        let snippet: u8 = contents[i];
        if snippet.is_ascii_alphabetic(){
            temp.push(contents[i]);
        }
        if contents[i] == 92 && contents[i + 1] == 110 {                                      
            words.push(String::from_utf8(temp).expect("Found invalid UTF8"));
            temp = Vec::new();
            i += 2;
        }
        i += 1;
    }
    println!("{:?}", words);
    words
}

fn assemble_board(width: usize, words: Vec<String>, transparent: bool) -> Vec<Vec<Item>>{
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
    board
}
