use std::io::stdout;

use crossterm::{
    event::read,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen);
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    read()?;
    stdout.execute(terminal::LeaveAlternateScreen);
    terminal::disable_raw_mode()?;

    Ok(())
}
