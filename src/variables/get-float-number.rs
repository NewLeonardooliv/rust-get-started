fn get_float_number() -> f32 {
    let float_number: f32 = 3.5;

    return float_number; 
}

fn main() {
    let float_number_getted: f32 = get_float_number();

    println!("Float number = {}", float_number_getted);
}