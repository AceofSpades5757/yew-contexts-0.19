use yew::context::ContextHandle;
use yew::prelude::*;

use super::context::SettingsContext;

pub enum Message {
    Update(SettingsContext),
}

pub struct Subscriber {
    settings: SettingsContext,
    // Needed for updates
    _context_listener: ContextHandle<SettingsContext>,
}

impl Component for Subscriber {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (settings, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Message::Update))
            .expect("No Message Context Provided");

        Self {
            settings,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Update(settings) => {
                self.settings = settings;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let setting_1 = self.settings.setting_1.to_string();

        html! {
            <>
                <h2>{ "Subscriber" }</h2>
                <p>
                    <strong>{ "Setting 1: " }</strong>
                    { setting_1 }
                </p>
            </>
        }
    }
}
