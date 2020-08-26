use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
  let mut data = String::new();
  let f = File::open("/etc/hosts")
               .expect("Unable to open file");
  let mut br = BufReader::new(f);
  br.read_to_string(&mut data)
    .expect("Unable to read string");
  println!("{}", data);
}
