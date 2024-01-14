fn result_when_error_or_success() -> Result<String, u8> {
    Err(32)
}

fn main() {
    match result_when_error_or_success() {
        Ok(s) => println!("Realized with success = {}", s),
        Err(number) => println!("throw error = {}", number),
    }
}
