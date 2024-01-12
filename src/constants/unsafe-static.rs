static mut GLOBAL:u8 = 1;

fn unsafe_quote() {
    unsafe {
        println!("this is a unsafe constant = {}", GLOBAL)
    }
}

fn main() {
    unsafe_quote();
}