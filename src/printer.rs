const WIDTH: usize = 10;

/// Prints the msg + the item at a constant width. The item is turned into it's Debug String: '{:?}'
pub fn eprintln_debug(msg: &str, item: &dyn std::fmt::Debug) {
    let item_string = format!("{:?}", item);
    eprintln(msg, &item_string);
}

/// Prints the msg + the item at a constant width.
pub fn eprintln(msg: &str, item: &dyn std::fmt::Display) {
    eprintln!("{msg}: {item:>WIDTH$}");
}

