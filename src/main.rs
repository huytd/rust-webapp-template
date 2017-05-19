extern crate stdweb;

use stdweb::web::{
    document,
    INode
};

fn main() {
    stdweb::initialize();

    let root = document().query_selector("#root").unwrap();
    root.set_text_content("Hello!");

    stdweb::event_loop();
}
