use clap::App;

fn main() {
    let _ = App::new("echor")
        .version("0.1.0")
        .about("Echo with Rust")
        .get_matches();
    println!("{:?}", std::env::args());
}
