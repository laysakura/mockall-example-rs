///! Infrastructure layer, having main function.
mod ui;

use ui::cli::Cli;

fn main() {
    Cli::process_cmd();
}
