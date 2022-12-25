#[cfg(test)]
mod vec_test {

    #[test]
    fn vec_impl() {
        let v: Vec<i32> = Vec::new();
        println!("{:?}", v);
        let v2: Vec<i32> = vec![1, 2, 3];
        println!("{:?}", v2);
    }
    #[test]
    fn iter() {
        let mut v: Vec<i32> = vec![1, 2, 3, 4];
        //normal iter
        for i in v.iter() {
            println!("{:?}", i);
        }
        // mutable iter
        for i in v.iter_mut() {
            *i = 7;
            println!("{:?}", i);
        }
        //enumerate
        for (i, x) in v.iter().enumerate() {
            println!("i:{} x:{} ", i, x)
        }
    }
}
