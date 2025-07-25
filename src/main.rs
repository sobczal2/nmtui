use app::App;
use tui::{setup_terminal, shutdown_terminal};

mod app;
mod command;
mod event;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut terminal = setup_terminal()?;
    let app = App::new();
    app.run(&mut terminal).await;
    shutdown_terminal(terminal)?;

    Ok(())
}
