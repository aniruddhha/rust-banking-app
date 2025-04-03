use std::io::{self, Write};

pub fn read_number<T: std::str::FromStr>(prompt: &str, error: &str) -> Result<T, T::Err> {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(error);
    input.trim().parse::<T>()
}
