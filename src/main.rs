use std::io::{Write, stdout};
use crossterm::{ExecutableCommand};
use crossterm::terminal::{self, ClearType};
use std::time::Duration;
use std::thread;

fn main(){
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    thread::sleep(Duration::from_secs(5));
    let _ = stdout.flush();
}
