struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    let p = Point { x: 0, y: 7 };

    //Destructuring with let.
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    println!("b is {}", b);

    //Ignoring unused variables with an underscore.
    let _x = 0;
    let _y = 7;

    //Ignoring parts of a value with ..
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    //Extra conditionals with match guards.
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    //
    let x = 4;
    let y = true;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ operator to bind to a variable.
    let message = Message::Hello { id: 9 };
    match message {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
