- [strings](#strings)
- [cheat](#cheat)
- [methods](#methods)
  - [new](#new)
  - [len](#len)
  - [replace](#replace)
  - [split charactors](#split-charactors)
  - [vec operation](#vec-operation)

# strings

# cheat

```rust
let st="kdfjls";
let st=String::new("thisl is string");

let word:&str in st1.split_whitespace() {
    println!("{}",word);
}
let s2= s1.replace("ntihin","john");
```

# methods

## new

```rust
let st1="its me nithin";
let st2=String::new("this is string");
```

## len

```rust
st.len()
```

## replace

```rust
let st1="its me nithin";
let st2=st1.replace("nithin","john");
```

## split charactors

```rust
let v1: Vec<char> = st.chars().collect();
```

## vec operation

```rust
   let st = String::from("c d i d k a b c");
        let mut v1: Vec<char> = st.chars().collect();
        println!("{:?}", v1);
        v1.sort();
        println!("after sort {:?}", v1);
        v1.dedup();
        println!("after dedup {:?}", v1);
```
