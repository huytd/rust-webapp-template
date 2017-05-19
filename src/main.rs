#[macro_use]
extern crate stdweb;

use stdweb::web::{
    document,
    INode,
    IEventTarget
};
use stdweb::web::event::ClickEvent;

fn main() {
    stdweb::initialize();

    let node = document().create_element("button");
    node.set_text_content("Click me");
    node.add_event_listener(move |_: ClickEvent| {
        js! {
            alert("Yo!");
        }
    });
    let root = document().query_selector("#root").unwrap();
    root.set_text_content("Hello!");
    root.append_child(&node);

    stdweb::event_loop();
}
