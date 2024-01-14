fn ownsership() {
    let name = String::from("Leonardo");

    let other_name = steal(name);

    println!("{}", other_name);
}

fn steal(string: String) -> String {
    println!("{}", string);

    string
}

fn main() {
    ownsership()
}
