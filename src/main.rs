use std::{error::Error};

use dotenv;

use todolist::todotui::TodoTUI;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let mut tui = TodoTUI::new()?;
    tui.run()?;

    Ok(())
}
