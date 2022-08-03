mod context;
mod producer;
mod subscriber;

use context::SettingsProvider;
use producer::Producer;
use subscriber::Subscriber;
use yew::prelude::*;

fn main() {
    yew::start_app::<Main>();
}

#[function_component(Main)]
fn main() -> Html {
    html! {
        <SettingsProvider>
            <Producer />
            <Subscriber />
        </SettingsProvider>
    }
}
