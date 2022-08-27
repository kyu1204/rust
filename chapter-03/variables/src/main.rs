// 상수 선언식
// const MAX_POINT: u32 = 100_000;

fn main() {
    // 변수는 기본적으로 불변
    // let x = 5;
    // x = 3 // (X)
    // 변수 값을 변경가능하게 하려면 'mut' 을 붙여야 함
    let mut x = 5;
    println!("value is {}", x);
    x = 6; // (O)
    println!("value is {}", x);

    // shadows 가능 (같은 변수 이름으로 재할당)
    let x = 1;
    let x = x + 2;
    let x = x + 3;
    println!("value is {}", x);

    let space = "     ";
    let space = space.len(); // (O)
    // let space = "     ";
    // space = space.len();  // (X)
    println!("space is {}", space);

    let a: u32 = 1;
    let b = 3.6;
    let c = 'a';
    let d: (u32, f64, char) = (a, b, c);
    let d1 = d.0;
    let e = [1, 2, 3];
    let e1 = e[0];

    // Rust 표현식과 구문으로 이루어져 있다.
    let a2 = another_func(a);
    println!("func value is {}", a2);

    let d = 5;
    if d < 10 {
        println!("if is True");
    }
    else if d == 0 {
        println!("else if is True");
    }
    else {
        println!("if is False");
    }

    // Rust 의 조건문은 무조건 bool type 이 와야한다.
    // if d {
    //     println!("is Error!");
    // }

    let condition = true;
    let f = if condition {
        5
    }
    else {
        6
    };

    // 변수 조건문 값 설정 시 조건문 결과 타입이 같아야함
    // let f = if condition {
    //     5
    // }
    // else {
    //     'c'
    // };
    println!("let condition value is {}", f);

    // loop {
    //     println!("is infinite");
    // }

    let mut g = 3;
    while g != 0 {
        println!("while value is {}", g);
        g = g - 1;
    }

    let h = [1, 2, 3];
    for h1 in h {
        println!("for value is {}", h1);
    }

    for life in (1..4).rev(){
        println!("life is {}", life);
    }
    println!("life is done");
}

fn another_func(x: u32) -> u32 {
    println!("another value is {}", x);

    // 세미콜론이 없으니 표현식이고 결과가 리턴된다
    x + 1
}