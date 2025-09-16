use rand::{Rng, rng};
use std::io;

fn generate_password(length: usize) -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=";
    let mut rng = rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

fn main() {
    println!("Введите длину пароля:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    let length: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Некорректное число");
            return;
        }
    };
    let password = generate_password(length);
    println!("Сгенерированный пароль: {}", password);
}
