extern crate termion;

mod data;

use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::cursor;
use termion::raw::IntoRawMode;
use termion::terminal_size;

use rand::Rng;

use std::time::Instant;

static INIT_SPAWNING_SPEED: u128 = 1000;
static SPAWNING_SPEED_STEP: u128 = 50;
static SPAWNING_SPEED_MINIMUM: u128 = 300;
static REFRESH_RATE: u64 = 50;
static WORD_SPACING: i16 = 2;

#[derive(Debug)]
struct Target<'a> {
    x: u16,
    y: u16,
    length: usize,
    correct: usize,
    word: &'a str,
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut targets: Vec<Target> = Vec::new();

    let dictionary = data::dictionary();
    let mut rng = rand::thread_rng();

    write!(stdout, "{}", cursor::Hide).unwrap();

    write!(stdout, "{}", termion::clear::All).unwrap();

    let mut start = Instant::now();
    let mut termina_width: u16;
    let mut termina_height: u16;
    let mut speed: u128 = INIT_SPAWNING_SPEED;
    loop {
        let elapsed = start.elapsed();
        if elapsed.as_millis() >= speed {
            start = Instant::now();
            speed -= SPAWNING_SPEED_STEP;
            if speed <= SPAWNING_SPEED_MINIMUM {
                speed = SPAWNING_SPEED_MINIMUM;
            }
            match terminal_size() {
                Ok(sizes) => {
                    termina_width = sizes.0 as u16;
                    termina_height = sizes.1 as u16;
                }
                _ => {
                    termina_width = 10;
                    termina_height = 10;
                }
            }
            let choice = rng.gen_range(0, dictionary.len());

            let mut x: u16 = rng.gen_range(0, termina_width - dictionary[choice].len() as u16);
            let mut y: u16 = rng.gen_range(0, termina_height);

            let mut wrong_place: bool = true;
            while wrong_place {
                wrong_place = false;
                for target in targets.iter() {
                    if y == target.y {
                        if x <= target.x + target.length as u16
                            && (x as i16 + dictionary[choice].len() as i16)
                                > (target.x as i16 - WORD_SPACING)
                        {
                            wrong_place = true;
                            break;
                        }
                    }
                }
                if wrong_place {
                    x = rng.gen_range(0, termina_width - dictionary[choice].len() as u16);
                    y = rng.gen_range(0, termina_height);
                }
            }

            let new_target = Target {
                x,
                y,
                word: dictionary[choice],
                length: dictionary[choice].len(),
                correct: 0,
            };
            targets.push(new_target);
            write!(stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
            stdout.write_all(dictionary[choice].as_bytes()).unwrap();
        }

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            write!(stdout, "{}", cursor::Show).unwrap();
            println!("{:?}", targets);
            break;
        } else if let Some(Ok(pressed_key)) = b {
            let mut to_remove: Vec<usize> = Vec::new();
            for (i, target) in targets
                .iter_mut()
                .filter(|e| e.length > e.correct)
                .enumerate()
            {
                match target.word.chars().nth(target.correct) {
                    Some(first_character) => {
                        if first_character == pressed_key as char {
                            target.correct += 1;
                            if target.correct >= target.length {
                                to_remove.push(i);
                                write!(stdout, "{}", cursor::Goto(target.x, target.y)).unwrap();
                                write!(
                                    stdout,
                                    "{}",
                                    format!("{:width$}", " ", width = target.length)
                                )
                                .unwrap();
                            }
                        }
                    }
                    None => (),
                }
            }
            for i in to_remove.iter() {
                targets.remove(*i);
            }
        }

        thread::sleep(Duration::from_millis(REFRESH_RATE));

        stdout.flush().unwrap();
    }
}
