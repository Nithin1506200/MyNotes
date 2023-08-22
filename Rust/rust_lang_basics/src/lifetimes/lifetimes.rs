#[allow(dead_code)]
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

#[allow(dead_code)]
fn failed_borrow<'a>() {
    let _x: i32 = 12;
    // let y: &'a i32 = &_x;
    // error: `x` does not live long enough
    // label: borrowed value does not live long enough
}
#[derive(Debug)]
#[allow(dead_code)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}
//-------------------GENERIC BOUNDING--------------
// Just like generic types can be bounded, lifetimes (themselves generic) use bounds as well. The : character has a slightly different meaning here, but + is the same. Note how the following read:
// T: 'a: All references in T must outlive lifetime 'a.
// T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T)
where
    T: std::fmt::Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}
//--------------------COERCION----------------------
// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

#[cfg(test)]
mod test {
    #[test]
    fn print_refs_test() {
        let x = 32;
        let y = 43;
        super::print_refs(&x, &y)
    }
}
