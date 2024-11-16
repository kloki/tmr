use app::App;
mod app;

use ratatui::{TerminalOptions, Viewport};

#[tokio::main]
async fn main() {
    println!();
    let terminal = ratatui::try_init_with_options(TerminalOptions {
        viewport: Viewport::Inline(10),
    })
    .unwrap();
    App::new().run(terminal).await.unwrap();
    ratatui::restore();
}
