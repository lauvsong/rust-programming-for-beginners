// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Green,
    Blue
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Blue => println!("blue"),
            Color::Green => println!("green")
        }
    }
}

struct Dimension {
    width: i32,
    height: i32,
    depth: i32
}

impl Dimension {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height:{:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox{
    dimension: Dimension,
    weight: i32,
    color: Color
}

impl ShippingBox {
    fn new(dimension: Dimension, weight: i32, color: Color) -> Self {
        Self {
            dimension,
            weight,
            color
        }
    }

    fn print(&self) {
        self.dimension.print();
        println!("weight: {:?}", self.weight);
        self.color.print();
    }
}

fn main() {
    let dimension = Dimension {
        width: 1,
        height: 2,
        depth: 1
    };

    let shippingBox = ShippingBox::new(dimension, 1, Color::Red);
    shippingBox.print();
}
