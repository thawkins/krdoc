use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value ="default")]
    context: String,

    /// Name of the person to greet
    #[arg(short, long, default_value ="all")]
    output: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = true)]
    all_namespaces: bool,
}

fn main() {
    let args = Args::parse();
    println!("context {}!", args.context);
    println!("output {}!", args.output);
    println!("all_namespaces {}!", args.all_namespaces);


}