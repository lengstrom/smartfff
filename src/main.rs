extern crate ncurses;


use ncurses::*;
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

    let input_handle = thread::spawn( || {
        while (true) {
            let char_code : u32 = (getch() as u32);
            tx.send(char_code);
        }
    });

    printw("Hello, world!");
    
    refresh();
    getch();
    endwin();
}

fn get_character() -> Option<i32> {
    match (rx.recv()
    let ch = char::from_u32(char_code).expect("Invalid char!");
}

