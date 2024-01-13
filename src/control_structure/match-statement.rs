fn main() {
    let language: &str = "PHP";

    let pourpose = match language {
        "PHP" => "Web",
        "FLUTTER" => "Mobile",
        "PYTHON" => "Data Science",
        _ => ""
    };

    println!("{}", pourpose);
}
