struct MyTuple(bool, u8, i8);
type MyTupleAlias = (bool, u8, i8);

struct MyStruct {
    should_do_groceries: bool,
    birth_year: u32,
    height_in_cm: u32,
}

enum Direction {
    Down,
}

enum Shape {
    Square { side: f64 },
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

fn main() {
    println!("Hello, World!");

    let arr: [i32; 3] = [1, 2, 3];

    println!("{:?}", arr);
    println!("{}", arr[0]);

    let arr_ref: &[u8] = &[1, 2, 3];

    println!("{:?}", &arr_ref);

    let stack_str: &str = "hello";

    println!("{}", stack_str);

    let tuple: (bool, u32, i32) = (true, 123, -123);

    println!("the first tuple time is {}", tuple.0);

    let my_tuple = MyTuple(false, 127, -128);

    println!("{}", my_tuple.0);
    println!("{}", my_tuple.1);
    println!("{}", my_tuple.2);

    let my_typle_alias: MyTupleAlias = (false, 127, -128);

    println!("{:?}", my_typle_alias);

    let my_struct = MyStruct {
        should_do_groceries: true,
        birth_year: 2004,
        height_in_cm: 188,
    };

    println!("{:?}", my_struct.should_do_groceries);
    println!("{:?}", my_struct.birth_year);
    println!("{:?}", my_struct.height_in_cm);

    let _direction: Direction = Direction::Down;

    let s = Shape::Rectangle {
        width: 10.5,
        height: 5.8,
    };

    match s {
        Shape::Circle { radius } => {
            println!(
                "A circle of radius {} and diameter {}!",
                radius,
                radius * 2.0
            );
        }
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        }
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        }
    }
}
