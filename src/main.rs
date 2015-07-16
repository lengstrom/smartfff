extern crate ncurses;
extern crate rust-linenoise;

use ncurses::*;
use rust-linenoise::*;
use std::char;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() {
    // Setup ncurses
    initscr();
    raw();

    // Allow for keyboard
    keypad(stdscr, true);

    let (tx, rx) = mpsc::channel();

    let input_handle = thread::spawn( move || {
        while (true) {
            let char_code : u32 = (getch() as u32);
            tx.send(char_code);
        }
    });
    
    refresh();
    getch();
    endwin();
}

fn get_character(rx : mpsc::Receiver<u32>) -> Option<char> {
    let attempt = rx.try_recv();
    match (attempt) {
        Some(char_code) => char::from_u32(char_code),
        _ => None
        
    };
}

