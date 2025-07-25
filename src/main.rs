use app::App;
use tui::{setup_terminal, shutdown_terminal};

mod app;
mod event;
mod input;
mod loader;
mod state;
mod tui;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut terminal = setup_terminal()?;
    let app = App::new();
    app.run(&mut terminal).await;
    shutdown_terminal(terminal)?;

    Ok(())
}
