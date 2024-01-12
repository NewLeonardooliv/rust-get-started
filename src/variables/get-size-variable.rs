fn get_size_variable() {
    let variable = 300;

    println!(
        "variable = {}, size = {} bytes",
        variable,
        std::mem::size_of_val(&variable)
    );
}

fn main() {
    get_size_variable();
}
