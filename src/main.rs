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
    ( $name:ident => {
        init: $init_expression:block,
        render: $render_expression:expr
      }
    ) => (
        struct $name {
            element: Element
        }

        impl $name {
            pub fn new() -> $name {
                let e = $name{ element: $init_expression };
                e
            }

            pub fn get_element(&self) -> &Element {
                &self.element
            }

            pub fn render(&mut self) {
                $render_expression(&self.element);
            }
        }
    )
}

macro_rules! mount_component {
    ( $root:ident, $element:ident) => {
        $root.append_child($element.get_element());
        $element.render();
    }
}

component!(GreenButton => {
    init: {
        let button = document().create_element("button");
        button.add_event_listener(move |_: ClickEvent| {
            js! {
                alert("Yo! This is the Green Button!");
            }
        });
        button
    },
    render: |this: &Element| {
        this.set_text_content("This is a button");
    }
});

fn main() {
    stdweb::initialize();

    let mut button = GreenButton::new();
    let root = document().query_selector("#root").unwrap();

    mount_component!(root, button);

    stdweb::event_loop();
}
