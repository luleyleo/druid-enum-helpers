#![allow(dead_code)]
#![allow(unused_variables)]

#[allow(unused_imports)]
use match_derive::Matcher;

#[allow(unused_imports)]
use match_macro::match_widget;

use druid::widget::{Label, Button};
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
            Event::Click(u32, u32) => {
                let widget = Some(Label::new("test"));
                widget.map(|w| {
                    let boxed: Box<dyn Widget<Event>> = Box::new(w);
                    boxed
                })
            },
            Event::Key(char) => {
                let widget = Button::new("test");
                let boxed: Box<dyn Widget<Event>> = Box::new(widget);
                Some(boxed)
            },
            Event::Unknown => None,
        }
    );
}
