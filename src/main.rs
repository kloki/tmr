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
    let result = App::new().run(terminal).await;
    ratatui::restore();
    if result.is_err() {
        println!("\nSomething went wrong!")
    }
}
