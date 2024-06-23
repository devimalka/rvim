use core::panic;
use crossterm::cursor;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, ClearType};
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

mod filepicker;

fn main() {
    let mut stdout = stdout();
    let _ = stdout.execute(terminal::EnterAlternateScreen);
    let _ = terminal::enable_raw_mode();
    let (x, y) = terminal::size().unwrap();
    terminal::SetSize(x, y);
    let wsize = terminal::window_size();
    let _ws = match wsize {
        Ok(val) => val,
        Err(err) => panic!("Error in getting window size {:?}", err),
    };

    let mut quit = false;
    let mut column: u16 = 0;
    let mut row: u16 = 0;
    let mut esc = false;

    let cur = stdout.execute(cursor::EnableBlinking);
    stdout.queue(terminal::Clear(ClearType::All)).unwrap();

    stdout.execute(cursor::SetCursorStyle::BlinkingBlock);
    while !quit {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(x, y) => {
                    terminal::SetSize(x, y);
                }
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => match event.code {
                    KeyCode::Char(x) => {
                        if x == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                            quit = true
                        } else if x == 'j' && esc {
                            row += 1;
                        } else if x == 'k' && esc {
                            if row != 0 {
                                row -= 1;
                            }
                        } else if x == 'l' && esc {
                            column += 1;
                        } else if x == 'h' && esc {
                            if column != 0 {
                                column -= 1;
                            }
                        }
                    }
                    KeyCode::Esc => {
                        if esc {
                            esc = false;
                        } else {
                            esc = true;
                        }
                    }
                    KeyCode::Enter => println!("Enter pressed\r"),
                    _ => todo!(),
                },
                Event::Mouse(MouseEvent) => todo!(),
                Event::Paste(string) => todo!(),
            }
        }
        stdout.execute(cursor::MoveTo(column, row));
        stdout.flush();
        thread::sleep(Duration::from_millis(16.66666 as u64));
        stdout.execute(terminal::Clear(ClearType::All));
    }

    terminal::disable_raw_mode();
    stdout.execute(terminal::LeaveAlternateScreen);
}
