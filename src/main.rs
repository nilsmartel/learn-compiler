use structopt::StructOpt;
mod compile;
mod parse;

fn main() {
    let config = Config::from_args();

    let input = std::fs::read_to_string(config.file).unwrap();

    let ast = parse::Ast::new(&input).unwrap();

    println!("{:#?}", ast);
}

#[derive(StructOpt)]
struct Config {
    /// Input file to operate on
    #[structopt(short, long)]
    file: String,
}
