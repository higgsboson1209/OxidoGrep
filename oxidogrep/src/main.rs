//Using function in Standard Library to read command line arguments 
use std::env;
use std::process;

use oxidogrep::Config;
fn main() {
    //reading the command line arguments 
    let args: Vec<String> = env::args().collect();


    let config  = Config::new(&args).unwrap_or_else(|err| {

        println!("Problem parsing arguments: {}",err);
        process::exit(1);
   });

    println!("Searching for {}", config.query);
    println!("In file {} \n ",config.filename);

    if let Err(e) = oxidogrep::run(config) {

        println!("Application error: {}", e);
        process::exit(1);
    }


}
