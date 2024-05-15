use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// arg a
    #[arg(short, long, required = false, default_value_t = String::from("a"))]
    a: String,

    /// arg b
    #[arg(short, long, required = false, default_value_t = String::from("b"))]
    b: String,
}
fn main() {
    let args = Cli::parse();

    println!("ARGS: {} - {}", args.a, args.b);
}
