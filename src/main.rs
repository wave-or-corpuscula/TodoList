use std::{error::Error};

use todolist::todotui::TodoTUI;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = TodoTUI::new()?;
    tui.run()?;

    Ok(())
}
