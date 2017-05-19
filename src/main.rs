#[macro_use]
extern crate stdweb;

use stdweb::web::{
    document,
    INode,
    IEventTarget,
    Element
};
use stdweb::web::event::ClickEvent;

macro_rules! component {
    ( $name:ident => $expression:block ) => (
        fn $name() -> Element {
            $expression
        }
    )
}

component!(green_button => {
    let button = document().create_element("button");
    button.set_text_content("Dare to click me?");
    button.add_event_listener(move |_: ClickEvent| {
        js! {
            alert("Yo! This is the Green Button!");
        }
    });
    button
});

fn main() {
    stdweb::initialize();

    let node = green_button();
    let root = document().query_selector("#root").unwrap();
    root.set_text_content("Hello!");
    root.append_child(&node);

    stdweb::event_loop();
}
