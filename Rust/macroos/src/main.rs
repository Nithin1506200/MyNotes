use percy_dom::prelude::*;

macro_rules! give_me_foo_or_bar {
    (foo) => {};
    (bar) => {};
    (d) => {
        println!("fasdfasf");
    };
}
#[test]
fn madin() {
    let x = html! { <div>{ "My text nodes here " }</div> };
    if true {
        panic!("ohh my gawd");
    }
}

fn main() {}
