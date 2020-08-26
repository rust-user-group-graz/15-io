use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let f = File::open("/etc/hosts")
               .expect("Unable to open file");
  let f = BufReader::new(f);

  for line in f.lines() {
    let line = line.expect("Unable to read line");
    println!("Line: {}", line);
  }
}
