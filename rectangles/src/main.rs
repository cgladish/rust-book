fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = Rectangle {
        width: dbg!(2 * width1),
        height: height1
    };

    println!(
        "The area of the rectangle is {}",
        area(&rect1)
    );

    println!("Rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {}",
        rect1.area()
    );

    println!(
        "Calling width func {}",
        rect1.width()
    );
    println!(
        "Calling width func via ref {}",
        (&rect1).width()
    );

    let square = Rectangle::square(123);
    println!(
        "Square {square:?}"
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

impl Rectangle{
    fn width(&self) -> u32 {
        self.width * 10
    }
}