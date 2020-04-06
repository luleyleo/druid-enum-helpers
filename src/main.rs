#![allow(dead_code)]
#![allow(unused_variables)]

#[allow(unused_imports)]
use match_derive::Matcher;

#[allow(unused_imports)]
use match_macro::match_widget;

enum Event {
    Click(u32),
    Key,
}

fn main() {
    let fun = match_widget! { Event,
        Event::Click(u32) => println!("Click"),
        Event::Key => println!("Key"),
    };
    fun(Event::Click(42));
}
