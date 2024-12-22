use std::io::{stdin, BufReader, BufRead};
use std::fmt::Write;
use std::fs::File;
use std::collections::HashSet;
use std::cmp::min;

mod trie;
mod scc;
mod segtree;
mod graph;
mod kmp;
mod manacher;

fn main() {
    let offline = true;
    let mut input = if offline {
        let file = File::open("input.txt").unwrap();
        let istream: Box<dyn BufRead> = Box::new(BufReader::new(file));
        istream
    } else {
        Box::new(BufReader::new(stdin()))
    }.lines();
    
}

