use std::fs;

fn main() {
  let data = fs::read("/etc/hosts")
             .expect("Unable to read file");
  println!("{}", data.len());
}
