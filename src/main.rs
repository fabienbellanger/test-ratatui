pub mod app;
pub mod error;
pub mod layout;
pub mod widget;

use app::App;
use error::AppError;

fn main() -> Result<(), AppError> {
    let mut terminal = ratatui::try_init().unwrap();
    App::default().run(&mut terminal)?;
    ratatui::try_restore().unwrap();

    Ok(())
}
