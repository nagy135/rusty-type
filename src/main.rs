extern crate ncurses;

mod data;

use ncurses::*;
use rand::Rng;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

fn main()
{
    setup();
    let dictionary = data::dictionary();

    let mut rng = rand::thread_rng();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    for word in &dictionary {
        let x = rng.gen_range(1, max_x - 1);
        let y = rng.gen_range(1, max_y - 1);
        spawn_word(x, y, &word);
    }
    /* Start in the center. */
    let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
    let mut start_x = (max_x - WINDOW_WIDTH) / 2;

    let mut ch = getch();
    while ch != KEY_F(1)
    {

        ch = getch();
    }

    endwin();
}

fn spawn_word(x: i32, y: i32, word: &str ){
    mvprintw(x, y, &word);
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
