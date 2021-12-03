// 关联类型Iterator trait
// Iterator<Item=f64> 解释为，其关联类型 Item 设置为 f64 的 Iterator
// ... 指的是 Iterator 提供的方法


fn main() {
    // 查看值
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // 循环获取方法1
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // 从切片中获取
    let slice = &arr;
    for i in slice {
        println!( "{}", i);
    }

    let sum: i32 = (0..5).sum();
    println!( "sum is {}", sum );

    let sum: i64 = [10,20,30].iter().sum();
    println!("sum was {}", sum);

    // windows of slice
    let ints = [1,2,3,4,5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }
}
