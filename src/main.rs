mod util;
mod test;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets the type of tags to generate (alphanumeric, latin, english, french)
    #[arg(default_value_t = String::from("alphanumeric"))]
    dict: String,

    /// Sets the number of tags to generate
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    let tag_type = String::from(args.dict);

    for _i in 0..args.count {
        let tag = generate_tag(&tag_type);
        println!("{}", tag);
    }
}

fn generate_tag(tag_type: &str) -> String {
    match tag_type {
        "alphanumeric" => util::generate_alphanumeric_tag(Option::None),
        "custom" => util::generate_language_based_tag("custom"),
        _ => util::generate_alphanumeric_tag(Option::None),
    }
}
