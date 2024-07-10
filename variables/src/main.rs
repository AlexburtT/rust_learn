fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Затение(переменных)
    let b = 5;

    let b = b + 1;

    {
        let b = b * 2;
        println!("The value of x in the inner scope is: {}", b);
    }

    println!("The value of x is: {}", b);

    // Числа с плавающей точкой - https://habr.com/ru/articles/541816/
    // Cтандарт IEEE-754
    let y = 0.1;
    let z = 0.2;
    let sum = y + z;
    println!("The value of sum is: {}", sum);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "{}, {}, {}, {}, {}, {}",
        sum, difference, product, quotient, truncated, remainder
    );

    // Операторы - https://doc.rust-lang.ru/book/appendix-02-operators.html 
}

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; константы только такая нотация и могут быть вычисляемыми - https://doc.rust-lang.ru/reference/const_eval.html
