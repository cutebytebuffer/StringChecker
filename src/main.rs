
mod parser;
mod scanner;
mod ui;

fn main() -> color_eyre::Result<()> {
 color_eyre::install()?;

 ratatui::run(ui::app)?;

 Ok(())
}