fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![4, 5, 6];
    let mut v2_iter = v2.iter();

    let mut v2_iter = v2_iter.skip(2);
    println!("Got: {:?}", v2_iter.next());
    println!("Got: {:?}", v2_iter.next());
    println!("Got: {:?}", v2_iter.next());
    println!("Got: {:?}", v2_iter.next());
    let v3 = vec![3, 6, 7];
    let v3_iter = v3.iter();
    let total:usize = v3_iter.sum();
    for val3 in v3.iter() {
        println!("v3 got: {}", val3);
    }
    println!("total: {}", total);

    let v4  = vec![2, 20 ,50];
    let v4_ok: Vec<usize> = v4.into_iter()
        .filter(|s| s > &30)
        .collect();
    //println!("v4 is {:?}", v4);
    println!("v4_ok is {:?}", v4_ok);
}
