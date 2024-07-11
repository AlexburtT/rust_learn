fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// не конвертирует нелогический тип в логический, нужно указывать явно условие. После первого совпадения, дальше не проверяет. - https://doc.rust-lang.ru/book/ch03-05-control-flow.html