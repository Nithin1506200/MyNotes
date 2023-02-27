#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    #[test]
    fn create_str() {
        let st2 = String::from("nkls kasl");
        let mut st1 = String::new(); //empty string
                                     // push charactor 'c' and "a" doesnt work
        st1.push('a');
        // st1.push("a") doesn't work
        println!("first string : {}", st1);
        st1.push_str(" string");
        println!("second string : {}, len :{}", st1, st1.len())
    }
    #[test]
    fn split_whitespace() {
        let str = "its me nithin";
        for word in str.split_whitespace() {
            println!("{}", word);
        }
    }
    #[test]
    fn vector_string() {
        let st = String::from("c d i d k a b c");
        let mut v1: Vec<char> = st.chars().collect();
        println!("{:?}", v1);
        v1.sort();
        println!("after sort {:?}", v1);
        v1.dedup();
        println!("after dedup {:?}", v1);
    }
    #[test]
    fn looping() {
        let s = String::from("AaBbCcDdEeFf");
        for i in s.bytes() {
            println!("byte me:{}", i);
        }
        for i in s.chars() {
            println!("char: {}", i);
        }
    }
    #[test]
    fn replace() {
        let st = "its me nithin";
        let s2 = st.replace("nithin", "john");
        println!("{},{}", st, s2);
    }
}
