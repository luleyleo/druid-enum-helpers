# Druid enum helpers

In this repo I work on implementing the ideas mentioned in [druid#789](https://github.com/xi-editor/druid/issues/789)

The two sub-crates implement the macros while the main crate is the testing ground.

## match-macro

```rust
fn event_widget() -> druid::Matcher<Event> {
    match_widget! { Event, // could this type hint be ommited?
        Event::Click(x, y) => Label::dynamic(|data, _| {
            format!("x: {}, y: {}", data.x, data.y) // named touple?
        }),
        Event::Key(_) => Label::dynamic(|data, _| format!("key: {}", data))),
    }
}

fn event_widget() -> impl Widget<Event> {
    match_widget! { Event,
        Event::Click(x, y) => Label::dynamic(|data, _| {
            format!("x: {}, y: {}", x, y) // not valid or changed to data.x?
        }),
        _ => Label::dynamic(|data: &Event, _| format!("key: unhandled"))),
    }
}
```

## match-derive

```rust
#[derive(Clone, Data, Match)]
enum Event { .. }

fn event_widget() -> impl Widget<Event> {
    Event::matcher()
        .click(Label::dynamic(|data, _| {
            format!("x: {}, y: {}", data.0, data.1)
        ))
        .key(Label::dynamic(|data, _| {
            format!("key: {}", data))
        })
}

fn event_widget() -> impl Widget<Event> {
    Event::matcher()
        .key(Label::dynamic(|data, _| {
            format!("key: {}", data))
        })
        .default(Label::new("Unhandled Event"))
    }
}

fn event_widget() -> impl Widget<Event> {
    // Will emit warning for missing variant
    // Event::Click at runtime
    Event::matcher()
        .key(Label::dynamic(|data, _| {
            format!("key: {}", data))
        })
    }
}
```

