fn find(haystack: &str, needle: char) -> Option<usize> {
    for (i, c) in haystack.char_indices() {
        if c == needle {
            return Some(i)
        }
    }

    None
}

fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1 ..]),
    }
}

fn main() {
    let file_name = "foobar.rs";

    match extension_explicit(file_name) {
        None => println!("None"),
        Some(_) => println!("Some"),
    }
}
