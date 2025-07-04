#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}


fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The width of the rectangle is {} pixels", rect1.width);
    } else {
        println!("Width of the rectangle is 0 pixels");
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}