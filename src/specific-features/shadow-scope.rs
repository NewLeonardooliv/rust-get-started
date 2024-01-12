fn shadow_scope() {
    let one_to_three = 123;

    {
        let four_to_six = 456;
        println!("in shadow scope = {}", four_to_six);
    }

    println!("first = {}", one_to_three);
}

fn main() {
    shadow_scope();
}