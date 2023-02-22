use clap::{App, Arg};


fn main() {

  let matches = App::new("echo")
    .version("0.1.0")
    .author("JamesFielding")
    .about("rust echo rewrite")
    
    .arg(
      Arg::with_name("text")
      .value_name("TEXT")
      .help("Input Text")
      .required(false)
      .min_values(1),
    )
    .arg(
      Arg::with_name("omit_newline")
      .short("n")
      .help("Does not print the newline")
      .takes_value(false),
    )
    .get_matches();
    
    let text = matches.values_of_lossy("text").unwrap();
    let mut ending = "\n";
    let omit_newline = matches.is_present("omit_newline");
    if omit_newline {
      ending = "";
    }
    println!("{}{}", text.join(" "), ending);
    
}