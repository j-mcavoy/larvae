mod cli;
mod core;
mod equation;

use cli::Cli;

fn main() {
    env_logger::init();
    Cli::run();
}
