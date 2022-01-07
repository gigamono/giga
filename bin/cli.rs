use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {

}

fn main() {
    let args = Args::parse();
}
