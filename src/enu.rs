#[derive(Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

pub enum Shape{
    Circle(f32),
    Square(u32),
    Rectangle(i32,i32)
}

pub fn shape(shape:Shape){
    match shape{
        Shape::Circle(r) => println!("The area of the citcle is: {}",3.14*r*r),
        Shape::Square(a) => println!("The area of the square is: {}",a*a),
        Shape::Rectangle(w,h) => println!("The area of the rectangle is: {}",w*h),
    }
}

pub fn move_player(dir: Direction){
    match dir{
        Direction::Down => println!("Moving down"),
        Direction::Up => println!("Moving up"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}

    // Q. WHat are enums?
    // Ans. Enums are a way let you enumerate over various types of values.