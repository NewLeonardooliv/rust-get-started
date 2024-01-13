fn multiplication_table(number: u8) {
    let mut counter: u8 = 0;
    while counter < 10 {
        counter += 1;
        println!("{} X {} = {}", number, counter, number * counter);
    }
}

fn main() {
    for i in 1..5 {
        multiplication_table(i);
    }
}
