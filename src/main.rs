#[macro_use]
extern crate stdweb;

use stdweb::web::{
    document,
    INode,
    IEventTarget,
    IElement,
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
    ($parent:expr, $target:expr, $element:ident) => {
        let target = $parent.query_selector($target).unwrap();
        target.append_child($element.get_element());
        $element.render();
    }
}

macro_rules! html {
    ($html:expr) => {
        unsafe {
            js!(
                var parser = new DOMParser();
                var el = parser.parseFromString($html, "text/xml");
                return el.documentElement; 
            ).into_reference_unchecked()
        }.unwrap()
    }
}

component!(AppComponent => {
    init: {
        let e: Element = html!("
        <div>
            <span>Hello</span>
            <span>World</span>
            <br/>
            <div id=\"greenButton\"></div>
        </div>
        ");

        let mut button = GreenButton::new();
        mount_component!(e, "#greenButton", button);

        e
    },
    render: |this: &Element| {
    }
});

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

    let mut app = AppComponent::new();
    mount_component!(document(), "#root", app);

    stdweb::event_loop();
}
