mod cli;
use cli::Cli;

fn main() {
    env_logger::init();
    Cli::run();
}
