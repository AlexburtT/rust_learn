use std::io;

fn main() {
    println!("Угадай число!");
    println!("Введите своё число:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Вы угадали: {}", guess);
}