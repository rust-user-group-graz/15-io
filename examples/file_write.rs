use std::fs;

fn main() {
    let data = "Some data!";
    fs::write("/tmp/foo", data)
       .expect("Unable to write file");
}
