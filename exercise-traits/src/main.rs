
// 建立类型之间的关系
// 实现某种特质 implement x trait
// 定义特质 并为特定类型实现它
// 熟悉Rust需要学习标准库的特质 traits 
// 例如 Debug
trait Show {
    fn show(&self) -> String;
}
impl Show for i32 {
    fn show(&self) -> String {
        format!( "four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!( "eight-byte float {}", self)
    }
}

fn trait_test() {
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();

    println!("show {}", s1);
    println!("show {}", s2);
}

struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range( x1: f64, x2: f64, skip: f64) ->FRange {
    FRange {val: x1, end: x2, incr: skip}
}

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res > self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    for x in range(0.0, 1.0, 0.1) {
        println!( "{}", x )
    }

    trait_test();
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name())
}
