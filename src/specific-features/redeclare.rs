fn redeclare_variable() {
    let variable_to_redeclare:u8 = 1;

    println!("Start values size = {}", std::mem::size_of_val(&variable_to_redeclare));

    let variable_to_redeclare:&str = "Leonardo";

    println!("End values size = {}", std::mem::size_of_val(&variable_to_redeclare));
}

fn main() {
    redeclare_variable()
}