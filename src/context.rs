use std::rc::Rc;

use yew::prelude::*;

/// A group of settings to be shared among [child] components.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub setting_1: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { setting_1: String::from("DEFAULT SETTING 1") }
    }
}

impl Reducible for Settings {
    // We set this to String to avoid additional boilerplate.
    //
    // If you need to set different data types, then you'll need to expand
    // on this.
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self { setting_1: action }.into()
    }
}

pub type SettingsContext = UseReducerHandle<Settings>;

#[derive(Properties, Debug, PartialEq)]
pub struct SettingsProviderProps {
    #[prop_or_default]
    pub children: Children,
}


#[function_component(SettingsProvider)]
pub fn settings_provider(props: &SettingsProviderProps) -> Html {

    let settings = use_reducer_eq(|| Settings::default());
    html! {
        <>
            <ContextProvider<SettingsContext> context={settings}>
                {props.children.clone()}
            </ContextProvider<SettingsContext>>
        </>
    }
}
