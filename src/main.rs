#![allow(dead_code)]
#![allow(unused_variables)]

#[allow(unused_imports)]
use match_derive::Matcher;

#[allow(unused_imports)]
use match_macro::match_widget;

use druid::widget::{Button, Flex, Label, SizedBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Copy, Data)]
enum Event {
    Click(u32, u32),
    Key(char),
    Unknown,
}

#[derive(Clone, Data, Lens)]
struct AppState {
    event: Event,
}

fn main() {
    let window = WindowDesc::new(build_ui);

    let state = AppState {
        event: Event::Key('Z'),
    };

    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch the application");
}

fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Button::new("Next State").on_click(|_, data: &mut AppState, _| {
                data.event = match data.event {
                    Event::Click(_, _) => Event::Key('Z'),
                    Event::Key(_) => Event::Unknown,
                    Event::Unknown => Event::Click(4, 2),
                }
            }),
        )
        .with_spacer(20.0)
        .with_child(
            match_widget! { Event,
                Event::Click(u32, u32) => Label::new("click"),
                Event::Key(char) => Button::new("key")
                    .on_click(|_, _, _| println!("Key was clicked")),
                Event::Unknown => SizedBox::empty(),
            }
            .lens(AppState::event),
        )
        .padding(10.0)
}
