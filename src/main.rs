use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let mut functions = count::count::collect(&args.input)
        .drain(..)
        .filter(|s| {
            s.starts_with("pthread_cond")
                || s.starts_with("pthread_mutex")
                || s.starts_with("pthread_rwlock")
                || s.starts_with("pthread_spin")
        })
        .collect::<Vec<_>>();
    let len = functions.len();
    functions.sort();
    functions.dedup();
    for f in functions {
        println!("{}", f);
    }
    println!("{}", len);
}
