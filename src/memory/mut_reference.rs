fn ownership() {
    let mut name = String::from("Leonardo");

    steal(&mut name);
    
    println!("{}", name);
}

fn steal(string: &mut String) {
    string.push_str(" Oliveira");
    println!("{}", string);
}

fn main() {
    ownership()
}
