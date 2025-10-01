use std::path::Path;

fn main() {
    let path = Path::new("/Users/macbookpro/Desktop/utf/.june.txt.swx");
    // println!("ends with {}", path.e)
    if path.to_string_lossy().ends_with("swx") {
        println!("path ends correctly")
    } else {
        println!("doesnt end correctly")
    }
}