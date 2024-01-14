fn pattern_matching() {
    for x in 1..=20 {
        println!(
            "{}, {}",
            x,
            match x {
                1 => "Pouco",
                2 | 3 => "Mais ou menos",
                4..=9 => "Um pouco mais",
                _ if x % 2 == 0 => "Bem mais",
                _ => "Muito",
            }
        );
    }
}

fn main() {
    pattern_matching();
}