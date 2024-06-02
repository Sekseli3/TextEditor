use std::io::{stdout, Write};

use crossterm::{cursor, event::read, terminal, ExecutableCommand, QueueableCommand};

fn main() -> anyhow::Result<()> {
    let mut cx = 0;
    let mut cy = 0;
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen);
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    loop {
        stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.flush()?;
        match read()? {
            crossterm::event::Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Char('q') => break,
                _ => {}
            },
            _ => {}
        }
    }
    stdout.execute(terminal::LeaveAlternateScreen);
    terminal::disable_raw_mode()?;

    Ok(())
}
