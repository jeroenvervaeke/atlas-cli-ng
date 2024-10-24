pub mod cli;

fn main() {
    let cli = cli::new();
    let _matches = cli.get_matches();
}
