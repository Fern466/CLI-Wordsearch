use std::env;
use std::fs;
use rand::prelude::*;
struct Item {
    letter: char,
    //indicates what words might be "on" the letter
    owners: Vec<u8>,
}

struct Words {
    direction: Orientation,
    content: String,
}

enum Orientation {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
    None,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    // cargo run -- (word file) (target file) (special args)
    let words: Vec<Words> = read_input(args[1].clone());

    //The board must be at minimum have a width equivalent to the length of the largest element
    let mut length = 0;
    for x in &words{
        let len = x.content.len();
        if len > length{
            length = len;
        }
    }

    let board = assemble_board(length, words, false);
    for i in 0..board.len() { 
        let mut s = String::new();
        for j in 0..board.len(){
            s.push(board[i][j].letter);
        }
        println!("{s}");
    }
}

fn read_input(s: String) -> Vec<Words>{
    let mut words: Vec<Words> = Vec::new();
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
            let word = Words {
                direction: Orientation::None,
                content: String::from_utf8(temp).expect("Found invalid UTF8"),
            };                                    
            words.push(word);
            temp = Vec::new();
            i += 2;
        }
        i += 1;
    }
    words
}

fn assemble_board(width: usize, words: Vec<Words>, transparent: bool) -> Vec<Vec<Item>>{
    let mut rng = rand::rng();
    let mut board: Vec<Vec<Item>> = Vec::new();
    for i in 0..width {
        let mut row: Vec<Item> = Vec::new();
        for j in 0..width {
            let mut letter = '/';
            if !transparent{
                letter = rng.sample(rand::distr::Alphabetic).to_ascii_lowercase() as char;
            }
            let mut item = Item {
                letter: letter,
                owners: [].to_vec(),
            };
            row.push(item);
        }
        board.push(row);
    }
    board
}