#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};

    let rect2 = Rectangle::square(20);

    println!("rect2 is {:#?}", rect2);

    println!("The area of the rectangle rect1 is {} square pixels.", rect1.area());
}
