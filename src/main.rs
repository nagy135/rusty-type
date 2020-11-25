extern crate ncurses;
extern crate job_scheduler;
use job_scheduler::{JobScheduler, Job};
// use std::time::Duration;

mod data;

use ncurses::*;
use rand::Rng;

// static WINDOW_HEIGHT: i32 = 3;
// static WINDOW_WIDTH: i32 = 10;
static CORNER_PADDING: i32 = 5;

struct Target<'a> {
    x: i32,
    y: i32,
    remaining: i32,
    word: &'a str
}

fn main()
{
    setup();
    let dictionary = data::dictionary();

    let mut rng = rand::thread_rng();

    let mut targets: Vec<Target> = Vec::new();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut sched = JobScheduler::new();
    sched.add(Job::new("* * * * * *".parse().unwrap(), || {
        let choice = dictionary[rng.gen_range(0, dictionary.len())];
        let mut target = Target {
            x: rng.gen_range(CORNER_PADDING, max_x - CORNER_PADDING),
            y: rng.gen_range(CORNER_PADDING, max_y - CORNER_PADDING),
            word: choice,
            remaining: choice.len() as i32
        };
        spawn_word(target.x, target.y, target.word);
        targets.push(target);
    }));

    /* Start in the center. */
    // let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
    // let mut start_x = (max_x - WINDOW_WIDTH) / 2;

    let mut ch = getch();
    while ch != KEY_F(1)
    {
        sched.tick();
        ch = getch();
    }
    endwin();
}

fn spawn_word(x: i32, y: i32, word: &str){
    mvprintw(x, y, word);
}

fn setup(){
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info. */
    addstr("Use the arrow keys to move");
    mvprintw(LINES() - 1, 0, "Type words you see on screen, F1 to quit");
    refresh();

}
