use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let functions = count::count::collect(&args.input);
    let count = |x: &str| functions.iter().filter(|s| s.starts_with(x)).count();
    println!(
        "{}\t{}\t{}\t{}",
        count("pthread_cond"),
        count("pthread_mutex"),
        count("pthread_rwlock"),
        count("pthread_spin")
    );
}
