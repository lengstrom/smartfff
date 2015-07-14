extern crate ncurses;


use ncurses::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() {
    // Setup ncurses
    initscr();
    raw();

    // Allow for keyboard
    keypad(stdscr, true);

    let input_handle = thread::spawn( || {
        while (true) {
            let ch = getch();
        }
    });

    printw("Hello, world!");
    
    refresh();
    getch();
    endwin();
}

