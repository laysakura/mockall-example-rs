///! Infrastructure layer, having main function.
pub(crate) mod id_generator;
pub(crate) mod persistence;
pub(crate) mod repository_impls;
mod ui;

use ui::cli::Cli;

fn main() {
    let cli = Cli::default();
    cli.process_cmd();
}
