#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
#[derive(PartialEq,PartialOrd)]
enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 50
}

#[derive(Debug)]
enum Difficulty {
    Easy = 1,
    Medium,
    Hard
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right"
        }
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

fn eat_and_dump(v: Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is {}", s),
        Bool(b) => println!("bool is {}", b)
    }
}

fn dump() {

}

fn main() {
    println!("Hello, world!");

    let start = Direction::Left;
    println!( "{:?}", start.as_str() );

    // error[E0369]: binary operation `==` 
    // cannot be applied to type `Direction`
    assert_eq!(start, Direction::Left);

    let mut d = start;
    for _ in 0..8 {
        println!( "d {:?}", d);
        d = d.next();
    }

    let s = Speed::Slow;
    let speed = s as u32;
    println!( "speed {}", speed );

    let diff = Difficulty::Hard;
    println!( "diff {:?}", diff );

    use Value::*;
    let n = Number(2.3);
    let s = Str("hello fei".to_string());
    let b = Bool(true);
    
    println!( "n {:?} s {:?} b {:?}", n, s, b);

    eat_and_dump( b );
    eat_and_dump( s );
    eat_and_dump( n );
}
