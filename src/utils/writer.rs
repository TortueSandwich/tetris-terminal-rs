use std::io;
use crossterm::{style::{SetBackgroundColor, Print, Color}, cursor::MoveTo, queue};

use super::{tetromino::{Tetromino, Forme}, container::Container};

pub fn queue_draw(x: u16, y: u16, color:Color) -> io::Result<()> {
    let mut w = io::stdout();
    queue!(
        w,
        MoveTo(x*2, y),
        SetBackgroundColor(color),
        Print("  "),
        SetBackgroundColor(Color::Reset)
    )
}

pub fn quit_quit(){
    use crossterm::terminal::ClearType;
    use crossterm::cursor;
    use crossterm::terminal;
    use crossterm::style::ResetColor;
    use crossterm::execute;
    use crossterm::terminal::LeaveAlternateScreen;
    let mut w = io::stdout();
    let _ = queue!(w, ResetColor, terminal::Clear(ClearType::All), cursor::Show);
    let _ = terminal::disable_raw_mode();
    let _ = execute!(w, LeaveAlternateScreen);
}

pub fn draw_tetromino(t:&Tetromino, c: &Container) -> io::Result<()> {
    let mut w = io::stdout();
    for co in t.to_coord() {
        let mut x = (co.1 as u16 +  c.co_x.0 + 1)*2 + 1;
        let y = co.0 as u16 + 1 + c.co_y.0 + 1;
        if t.forme() == Forme::I || t.forme() == Forme::O { x -= 1} 
        queue!(
            w,
            MoveTo(x, y),
            SetBackgroundColor(t.color()),
            Print("  "),
            SetBackgroundColor(Color::Reset)
        )?;
    }
    Ok(())
}