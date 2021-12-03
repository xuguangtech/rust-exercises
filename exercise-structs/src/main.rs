
// use std::fmt::Debug;

// #[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

use std::fmt;
impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.first_name)
    }
}

// The values of a struct will be placed next 
// to each other in memory, although you should not 
// assume any particular memory layout, since the 
// compiler will organize the memory for efficiency, 
// not size, and there may be padding.
// 结构体的值将在内存中彼此相邻放置，
// 尽管您不应该假设任何特定的内存布局，
// 因为编译器将组织内存以提高效率，而不是大小，并且可能会有填充。

impl Person {
    fn new(first: &str, name: &str) -> Self {
        Self {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    // 这里是方法，通过 self 实现对对象的引用
    fn full_name( &self ) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // The keyword Self refers to the struct type
    // fn copy( &self ) -> Self {
    //     Self::new(&self.first_name, &self.last_name)
    // }

    // 方法可能允许使用可变的 self 参数修改数据
    fn set_first_name( &mut self, name: &str) {
        self.first_name = name.to_string()
    }

    // &self -> error[E0507]: cannot move out of `self.first_name` 
    // which is behind a shared reference
    // fn to_tuple(&self) -> (String, String) {
    fn to_tuple(self) -> (String, String) {
        // &self -> move occurs because `self.first_name` has type `String`, 
        // which does not implement the `Copy` trait
        (self.first_name, self.last_name)
    }
}

fn how(i: u32) -> &'static str {
    match i {
        0 => "none",
        1 => "one",
        2 => "two",
        _ => "many"
    }
}
// 所有值都有生命周期
// 引用的生命周期不能长于该值的生命周期
// Rust不允许引用突然失效的情况发生
// static 
// sometimes ugliness is the necessary price of being precise
#[derive(Debug)]
struct T <'a>{
    // error[E0106]: missing lifetime specifier
    s: &'a str
}

// error[E0106]: missing lifetime specifier
// fn make_t() -> T<'static> {
//     let string = "hello yun".to_string();
//     // error[E0515]: cannot return value referencing local variable `string`
//     T {s: &string}
// }
fn lifetime() {
    let s = "hello zf".to_string();
    let t = T {s: &s};
    println!( "{:?}", t );
}


fn main() {
    let p = Person {
        first_name: "Fei".to_string(),
        last_name: "Zhao".to_string()
    };

    println!( "{:?}", p );

    println!( "person {} {}", p.first_name, p.last_name );

    let mut p2 = Person::new("Fei", "Zhao");
    println!( "person {} {}", p2.first_name, p2.last_name );

    p2.set_first_name("tieyan");
    println!( "person {}", p2.full_name() );

    // Note that after p2.to_tuple() is called, 
    // then p2 has moved and is no longer available
    // 请注意，在调用 p2.to_tuple() 之后，v 已移动且不再可用。
    println!( "person {:?}", p2.to_tuple() );

    // error[E0382]: borrow of moved value: `p2`
    // println!( "person {}", p2.full_name() );

    // 生命周期测试方法
    let h = how( 1 );
    println!( "{}", h);
    // make_t();

    lifetime();
}
