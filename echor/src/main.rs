use clap::App;
use clap::Arg;

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .about("Echo with Rust")
        .arg(
            Arg::with_name("omit_newline")
                .short('n')
                .long("omit-newline")
                .help("Do not print newline"),
        )
        .get_matches();
    println!("{:#?}", matches);
}
