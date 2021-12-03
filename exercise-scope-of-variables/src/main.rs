
// 对变量的引用或者借用就是一个很重要的法则
// 但一定要注意变量的生命周期
// Rust是一个块作用域语言，变量只在所在的块内生效
// keep references to the original data
// borrow it

// a reference must not outlive the owner
// When a variable 'goes out of scope' then it is dropped. 
// Any memory used is reclaimed, and any other resources 
// owned by that variable are given back to the system - 
// for instance, dropping a File closes it.

// snake case
fn variable_scope() {
    {
        let a = 10;
        let b = "hello";
        {
            // warning: unused variable: `c`
            let c = "hello".to_string();
            // a,b,c are visible
            println!("a {} b {} c {}", a, b, c)
        }
        // error[E0425]: cannot find value `c` in this scope
        // println!("{}", c);

        // 字符串变量 c 被回收
        // 此处变量 a,b 可见

        // i 仅在循环内内可见
        for i in 0..a {
            println!( "i is {}", i);
            if i < b.len() {
                // 使用相同的名称创建新变量可能造成混淆
                let b = &b[i..];
                println!( "b is {} ", b);
            }
            // original b is no longer visible - it is shadowed
            // 源变量 b 不再可见，因为他当前是一个阴影变量
        }
        // 分片 b 被回收不可见
        // i 变量不可见

        // b 初始化可见
        println!( "b is {} ", b);
    }
}

// Rust is here saving you from the dreaded 'dangling pointer' 
// problem of C - a reference that points to stale data.
// Rust 在这里将您从 C 的可怕的“悬空指针”问题中拯救出来 - 一个指向陈旧数据的引用。
fn ref1() {
    let s1 = "hello fei".to_string();
    
    // warning: variable does not need to be mutable
    let mut rs1 = &s1;
    //{
        // let tmp = "hello yun".to_string();
        // borrowed value does not live long enough
        // rs1 = &tmp;
    //}
    // error[E0597]: `tmp` does not live long enough
    println!( "ref {}", rs1 );
}

fn main() {
    // variable scope testing
    variable_scope();
    ref1();
}
