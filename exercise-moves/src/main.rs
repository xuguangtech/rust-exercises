// 
fn dumps( s: &str ) {
    println!("{}", s);
}

// assignment of a non-Copy value moves the value from one location to another
// Rust would be forced to implicitly do a copy and break its promise to 
// make allocations explicit
fn main() {
    let s1  =   "hello fei".to_string();
    // 显示的复制
    let s2  =   s1.clone();

    // error[E0382]: borrow of moved value: `s1`
    println!( "s1 {}", s1 );
    
    dumps( &s2 );
    dumps( "hello fei" );
    println!( "{}", s2);

    let v1  =   120;
    let v2  =   v1;
    println!("{} {}", v1, v2);
}
