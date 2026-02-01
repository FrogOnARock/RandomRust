fn main() {

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
        area: 0
    };

    let mut rect2 = Rectangle {
        width: 20,
        height: 40,
        area: 0
    };

    let mut rect3 = Rectangle {
        width: 60,
        height: 40,
        area: 0
    };

    dbg!(&rect1);

    if rect1.width() {
        println!("The rectangle is valid.");
    }

    rect1.area();
    rect2.area();
    rect3.area();

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    area: u32
}

impl Rectangle {
    fn area(&mut self) {
        self.area = self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.area > rect2.area
    }
}