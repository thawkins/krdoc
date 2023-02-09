extern crate clap;
use clap::Parser;
#[derive(Parser, Default, Debug)]
#[clap(author="Tim Hawkins. <tim.thawkins@gmail.com>", version="0.0.1", about)]
/// A tool for documenting kubernetes clusters
struct Arguments {
    /// Select which documentation to create, generate all if not present
    generate: Option<String>,
    /// Select kubectl context to document, use "default" if not present
    context: Option<String>,
    /// filter namespace (regex), no filter if not present
    filter: Option<String>
}

fn main() {      
    let args = Arguments::parse();
    
    println!("{:?}", args);
}
