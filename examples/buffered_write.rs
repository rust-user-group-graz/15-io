use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
  let data = "Some data!";
  let f = File::create("/tmp/foo")
               .expect("Unable to create file");
  let mut f = BufWriter::new(f);
  f.write_all(data.as_bytes())
   .expect("Unable to write data");
}
