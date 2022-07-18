use std::env;
use loxide::loxide::Loxide;

fn main() {
    let loxide = Loxide::new();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 2 {
        loxide.run_file(&args[1]).unwrap();
        return;
    }

    loxide.run_prompt().unwrap();
}
