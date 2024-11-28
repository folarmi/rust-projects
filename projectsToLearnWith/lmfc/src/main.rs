use clap::Parser;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version,about,long_about = None)]

struct  Args {
    // Name of the person to greet
    #[arg(short,long)]
    name: String,
}
fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.name)
}


// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }
