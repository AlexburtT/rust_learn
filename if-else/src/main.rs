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

    loop_if();

    while_if();

    for_if();
}

// не конвертирует нелогический тип в логический, нужно указывать явно условие. После первого совпадения, дальше не проверяет. - https://doc.rust-lang.ru/book/ch03-05-control-flow.html

// Повторное выполнение кода с помощью циклов
// Повторение выполнения кода с помощью loop
fn loop_if() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 100 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// Циклы с условием while
fn while_if() {
    let mut number = 10;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}


// Цикл по элементам коллекции с помощью for

fn for_if() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}