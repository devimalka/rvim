use core::panic;
use crossterm::cursor;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{self, ClearType, WindowSize};
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

mod filepicker;
use filepicker::list_files::get_dir_files;
use filepicker::list_files::Details;
fn main() {
    let mut stdout = stdout();
    let _ = stdout.execute(terminal::EnterAlternateScreen);
    let _ = terminal::enable_raw_mode();
    let (mut x, mut y) = terminal::size().unwrap();
    terminal::SetSize(x, y);
    let wsize = terminal::window_size();
    let _ws = match wsize {
        Ok(val) => val,
        Err(err) => panic!("Error in getting window size {:?}", err),
    };

    let mut quit = false;

    stdout.queue(terminal::Clear(ClearType::All)).unwrap();
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
                        }
                    }
                    KeyCode::Esc => println!("Esc pressed\r"),
                    KeyCode::Enter => println!("Enter pressed\r"),
                    _ => todo!(),
                },
                Event::Mouse(MouseEvent) => todo!(),
                Event::Paste(string) => todo!(),
            }
        }

        stdout.queue(cursor::MoveTo(0, 0));
        stdout.flush();
        thread::sleep(Duration::from_millis(16.66666 as u64));
    }

    terminal::disable_raw_mode();
    stdout.execute(terminal::LeaveAlternateScreen);
}
