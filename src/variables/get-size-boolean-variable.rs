fn get_boolean_variable() -> usize {
    let boolean_variable:bool = true;

    return std::mem::size_of_val(&boolean_variable);
}

fn main() {
    let memory_size_boolean_variable = get_boolean_variable();

    println!("boolean size = {}", memory_size_boolean_variable);
}
