use app::arr::find;
use app::shape::*;
use app::my_struct::MyStruct;

fn main() {
    let circle = Circle {
        radius: 1.0,
    };

    let rect = Rectangle {
        width: 4.0,
        height: 3.5
    };

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rect.area());

    let my_struct = MyStruct::new(30, String::from("Hello From Rust!!"));
    my_struct.display();

    find();
}

