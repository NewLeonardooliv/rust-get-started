fn get_size_char_variable() -> usize {
    let char_variable:char = 'C';

    return std::mem::size_of_val(&char_variable);
}

fn main() {
    let memory_size_char_variable = get_size_char_variable();

    println!("char size = {}", memory_size_char_variable);
}