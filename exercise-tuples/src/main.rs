
// It's sometimes very useful to return multiple values from a function. 
// Tuples are a convenient solution:

// tuples的值可以包含多种数据类型，这是跟数据不同的地方
// 
fn add_mul( x: f64, y: f64 ) -> ( f64, f64 ) {
    (x + y, x * y )
}

fn asset_tuple() {
    let tuple = ("hello", 10, 'z');

    assert_eq!(tuple.0, "hello");
    // error[E0277]: can't compare `{integer}` with `&str`
    // assert_eq!(tuple.1, "9");
    assert_eq!(tuple.1, 10);
    assert_eq!(tuple.2, 'z');
}

fn arr2tuple() {
    for t in ["zero", "one", "two"].iter().enumerate() {
        println!( " {} {};", t.0, t.1 );
    }
    // 0 zero; ...
}

fn combine_tuples() {
    let names   =   ["ten", "hundred", "thousand"];
    let nums    =   [10, 100, 1000];
    for p in names.iter().zip( nums.iter() ) {
        println!( " {} {};", p.0, p.1 );
    }
}
fn main() {
    let t = add_mul(2.0, 10.0);

    // can debug print
    println!( "t {:?}", t);

    // can 'index' the values
    println!( "add {} mul {}", t.0, t.1 );

    // can extract value
    let (add, mul) = t;
    println!( "add {} mul {}", add, mul );

    asset_tuple();
    arr2tuple();
    combine_tuples();

}
