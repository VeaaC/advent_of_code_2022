use std::io::Read;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let child = std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024 * 1024)
        .spawn(move || {
            let mut input = Default::default();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let result = aoclib::run(args.day, &input);
            println!("Result:\n{result}");
        })
        .unwrap();
    child.join().unwrap();

    Ok(())
}
