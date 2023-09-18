use clap::{App,Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Bryan Pritchett <bryan@coderbox.co")
        .about("Rust Echo")
        .get_matches();

    //println!("{:?}",matches);
}
