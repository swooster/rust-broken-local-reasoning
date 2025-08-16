use std::fmt::Display;

fn main() {
    print_stuff(&[2, 3, 5, 7, 11, 13, 17]);
}

pub fn print_stuff<'a, T>(vals: &'a T)
where
    &'a T: IntoIterator<Item: Display>,
{
    println!("Values:");
    for val in vals {
        println!("  {val}");
    }
}

#[allow(dead_code)]
fn random_unrelated_code() {
    #[cfg(feature="broken")]
    rust_broken_local_reasoning::no_op();
}
