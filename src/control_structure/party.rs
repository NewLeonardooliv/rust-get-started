static ALLOWED_AGE: i32 = 21;

fn is_allowed(age: i32) -> bool {
    age > ALLOWED_AGE
}

fn main() {
    let user_age: i32 = 22;

    if is_allowed(user_age) {
        println!("Is it allowed to enter the party");
        return;
    }

    println!("not allowed to enter the party");
}
