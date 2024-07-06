use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets the type of tags to generate (alphanumeric, latin, english, french)
    #[arg(default_value_t = String::from("latin"))]
    dict: String,

    /// Sets the number of tags to generate
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    let tag_type = args.dict;

    println!("{}", args.count);
    println!("{}", tag_type);
}
