#![allow(dead_code)]
#![allow(unused_variables)]

#[allow(unused_imports)]
use match_derive::Matcher;

#[allow(unused_imports)]
use match_macro::match_widget;

use druid::widget::{Label, Button, SizedBox};
use druid::{Data, Widget};

mod matcher;

#[derive(Clone, Copy, Data)]
enum Event {
    Click(u32, u32),
    Key(char),
    Unknown,
}

fn main() {
    let matcher = matcher::WidgetMatcher::new(
        match_widget! { Event,
            Event::Click(u32, u32) => Label::new("test"),
            Event::Key(char) => Button::new("test"),
            Event::Unknown => SizedBox::empty(),
        }
    );
}
