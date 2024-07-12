#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &mut Rectangle) -> bool {
        other.width = other.width * 2;
        other.height = other.height * 2;
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let mut rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let mut rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&mut rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&mut rect3));

    println!("rect2 is {:?}", rect2);
}
