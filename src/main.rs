use std::env;
use std::fs;
use rand::prelude::*;
#[derive(PartialEq)]
struct Item {
    letter: char,
    //indicates what words might be "on" the letter
    owners: Vec<u8>,
}

#[derive(Clone)]
struct Words {
    direction: Orientation,
    content: String,
}

#[derive(Clone)]
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
    let mut words: Vec<Words> = read_input(args[1].clone());

    let mut temp = Words{
        direction: Orientation::None,
        content: String::new(),
    };

    //sorts words from largest to smallest
    for i in 0..words.len(){
        for j in 0..words.len(){
            if words[i].content.len() > words[j].content.len(){
                temp.content = words[j].content.clone();
                words[j].content = words[i].content.clone();
                words[i].content = temp.content.clone();
            }
        }
    }

    for x in &words{
        println!("{}", x.content);
    }
    //note that the width of the square will be => to this length, most likely equivalent and only greater in rare cases
    let length = words[0].content.len();

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
    //board creation
    let mut board: Vec<Vec<Item>> = Vec::new();
    for i in 0..width {
        let mut row: Vec<Item> = Vec::new();
        for _j in 0..width {
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
    //letter integration 
    for i in 0..words.len() - 1{
        if !attach_word(&mut board, &words[i]){
            assemble_board(width.clone(), words.clone(), transparent);
        }
    } 
    board
}

fn attach_word(board: &mut Vec<Vec<Item>>, word: &Words) -> bool{
    let border = word.content.len() + 1 - board.len();
    //check to see which placements have intersections with existing words
    //immediately place the word if the intersection is applicable
    //having it just return false would create a loop
    true
}