#![allow(dead_code)]
#![allow(unused_variables)]

#[allow(unused_imports)]
use match_derive::Matcher;

#[allow(unused_imports)]
use match_macro::match_widget;

enum Event {
    Click,
    Key,
}

fn main() {
    match_widget! { crate::Event,
        Event::Click(u32) => (),
        Event::Key => (),
    };
    println!("Hello, world!");
}
