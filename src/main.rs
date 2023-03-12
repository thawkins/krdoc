use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author="Tim Hawkins", version="0.1.0", about="program to document kubernetes clusters", long_about = None)]
struct Args {
    /// kubectl context to document
    #[arg(short, long, default_value ="")]
    context: String,
    /// kubenetes namespace to document
    #[arg(short, long, default_value ="default")]
    namespace: String,
    /// List of documentation types to produce
    #[arg(short, long, default_value ="all")]
    output: String,
    /// Wether to include all namespaces
    #[arg(short, long, default_value_t =false)]
    all_namespaces: bool,
}

fn main() {
    let args = Args::parse();
    println!("context {}", args.context);
    println!("namespacet {}", args.output);
    println!("output {}", args.output);
    println!("all_namespaces {}", args.all_namespaces);
}