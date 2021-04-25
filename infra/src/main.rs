///! Infrastructure layer, having main function.
pub(crate) mod id_generator;
pub(crate) mod persistence;
pub(crate) mod repository_impls;
mod ui;

use repository_impls::RepositoryImpls;
use ui::cli::Cli;

fn main() {
    let repo = RepositoryImpls::default();
    let cli = Cli::new(&repo);
    cli.process_cmd();
}
