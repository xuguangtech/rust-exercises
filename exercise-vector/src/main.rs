struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range(start: f64, end: f64, step: f64 ) -> FRange {
    FRange{ val:start, end: end, incr:step}
}

impl Iterator for FRange {
    type Item = f64; // associated type and a placeholder type
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}

fn traverse() {
    for x in range(0.0, 1.0, 0.1) {
        println!("{:.1}", x);
    }
}

fn vector_test() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump( &v );

    let first = v[0];
    let maybe_first = v.get(0);

    let maybe_value =   if maybe_first.is_some() {
        *maybe_first.unwrap()
    } else {
        -1
    };
    // or 
    // maybe_first.unwrap_or(&-1);
    println!("maybe_value is {}", maybe_value);

    println!("v is {:?}", v);
    println!("first {}", first);
    println!("mayby_first {:?}", maybe_first);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}

fn dump( arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn main() {
    vector_test();
    traverse();

    let v: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.sin()).collect();
    println!("{:?}", v);
}
