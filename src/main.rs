const PI: f64 = 3.141592;

fn main() {
    let t : (i32, bool, f64) = (1, false, 3.14);

    println!("{:?}", t);

    let hellos = ["헬로"; 10];

    println!("{:?}", hellos);

    print_number(3);

    add_numbers(3, 2);

    println!("{:?}", circle_area(2.0));

    let num = 5;

    if num % 3 == 0 {
        println!("num이 3으로 나누어떨어집니다.")
    }
    else if num % 3 == 1 {
        println!("num이 3으로 나누면 나머지는 1입니다.")
    }
    else {
        println!("num이 3으로 나누면 나머지는 2입니다.")
    }

    let condition = true;

    let num2 = if condition { 3 } else { 5 };

    println!("{num2}");

    let mut i = 0;

    let count_num = loop {
        println!("반복!");
        i += 1;

        if i == 3 {
            break i;
        }
    };

    println!("{count_num}");

    let xs = [1,2,3,4,5];
    let mut idx = 0;

    while idx < xs.len() {
        println!("xs[{}] = {}", idx, xs[idx]);
        idx += 1;
    }
    println!("완료!");

    for x in xs {
        println!("{x}");
    }
}

fn add_numbers(a: i32, b: i32) {
    let sum = a + b;
    println!("{a} + {b} = {sum}");
}

fn print_number(x: i32) {
    println!("x = {x}");
}

fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}