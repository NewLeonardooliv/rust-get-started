fn ownsership() {
    let name = String::from("Leonardo");

    steal(name);

    println!("{}", name);
}

fn steal(string: String) {
    println!("{}", string);
}

fn main() {
    ownsership()
}