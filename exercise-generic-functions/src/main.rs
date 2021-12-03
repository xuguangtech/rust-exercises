fn dump<T> (value: &T) 
// T 实现了Debug
where T: std::fmt::Debug {
    println!( "value is {:?}", value );
}


fn sqr<T> (x: T) -> T::Output 
where T: std::ops::Mul + Copy {
    x * x
}

fn main() {
    // println!("Hello, world!");
    let n = 42;
    dump( &n );

    println!( "{}", sqr(1.2) );
}
