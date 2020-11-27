extern crate termion;

mod data;

use termion::raw::IntoRawMode;
use termion::terminal_size;
use termion::async_stdin;
use termion::cursor;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;

use rand::Rng;
use rand::rngs::ThreadRng;

use std::time::{Instant};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let dictionary = data::dictionary();
    let mut rng = rand::thread_rng();

    write!(stdout, "{}", cursor::Hide);

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
            .unwrap();

    let mut start = Instant::now();
    loop {
        let elapsed = start.elapsed();
        if elapsed.as_secs() >= 1 {
            start = Instant::now();
            let mut termina_width: u16;
            let mut termina_height: u16;
            match terminal_size() {
                Ok(sizes) => {
                    termina_width = sizes.0 as u16;
                    termina_height = sizes.1 as u16;
                },
                _ => {
                    termina_width = 10;
                    termina_height = 10;
                }
            }
            let choice = rng.gen_range(0, dictionary.len());
            let x = rng.gen_range(0, termina_width);
            let y = rng.gen_range(0, termina_height);
            write!(stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
            stdout.write_all(dictionary[choice].as_bytes()).unwrap();
            // write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
        }

        let b = stdin.next();
        // write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();
        if let Some(Ok(b'q')) = b {
            write!(stdout, "{}", cursor::Show);
            break;
        }

        thread::sleep(Duration::from_millis(50));

        stdout.flush().unwrap();
    }
}
