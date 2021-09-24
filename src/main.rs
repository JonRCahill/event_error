use sycamore::prelude::*;
use sycamore_router::Route;
use wasm_bindgen_futures::spawn_local;

#[component(App<G>)]
fn app() -> Template<G> {
    let template = Signal::new(Template::empty());

    create_effect(cloned!((template) => move || {
        spawn_local(cloned!((template) => async move {
            let t = template! {
                Counter()
            };
            template.set(t);
        }));
    }));

    template! {
        div {
            (template.get().as_ref().clone())
        }
    }
}

#[component(Counter<G>)]
fn counter() -> Template<G> {
    let counter = Signal::new(0);

    create_effect(cloned!((counter) => move || {
        log::info!("Counter value: {}", *counter.get());
    }));

    let increment = cloned!((counter) => move |_| counter.set(*counter.get() + 1));

    let reset = cloned!((counter) => move |_| counter.set(0));

    template! {
        div {
            "Counter demo"
            p(class="value") {
                "Value: "
                (counter.get())
            }
            button(class="increment", on:click=increment) {
                "Increment"
            }
            button(class="reset", on:click=reset) {
                "Reset"
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| template! { App() });
}
