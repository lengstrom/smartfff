extern crate ncurses;
extern crate linenoise;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

// Nothing for now
fn autocomplete_callback(input: &str) -> Vec<String> {
    let ret = vec!["wot"];
    return ret.iter().map(|s| s.to_string()).collect();
}

fn main() {
    linenoise::set_callback(autocomplete_callback);
    
    // let val = linenoise::input("> ");
    let (tx, rx) = mpsc::channel();

    let input_handle = thread::spawn( move || {
        loop {
            // get input from line
        }
    });
}

fn get_selection(rx : mpsc::Receiver<Result<&str>>) -> 
