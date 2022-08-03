use yew::prelude::*;

use super::context::SettingsContext;

pub struct Producer;

impl Component for Producer {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (settings_ctx, _handler) = ctx
            .link()
            .context::<SettingsContext>(Callback::noop())
            .unwrap();
        let onclick = Callback::from(move |_| {
            settings_ctx.dispatch("Message Received: Producer.".to_string())
        });

        html! {
            <>
                <h2>{ "Producer" }</h2>
                <button {onclick}>{ "Producer" }</button>
            </>
        }
    }
}
