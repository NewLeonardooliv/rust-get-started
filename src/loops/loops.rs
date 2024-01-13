fn multiplication_table(number: u32) {
    let mut counter: u32 = 0;
    while counter < 10 {
        counter += 1;
        println!("{} X {} = {}", number, counter, number * counter);
    }
}

fn main() {
    // For in
    for i in 1..5 {
        multiplication_table(i);
    }

    // Infinite Loop
    let mut counter: u32 = 1;
    loop {
        counter += 1;
        multiplication_table(counter);

        if counter == 100 * 1000 {
            break;
        }
    }
}
