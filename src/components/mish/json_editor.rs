use {
    leptos::prelude::*,
    thaw::{Textarea, TextareaResize},
};

#[component]
pub fn JsonEditor(
    state: Option<serde_json::Value>,
    set_config_server_action: impl Fn(serde_json::Value) + 'static,
) -> impl IntoView {
    let value = RwSignal::new(
        state
            .map(|s| serde_json::to_string_pretty(&s).unwrap())
            .unwrap_or_default(),
    );

    view! {
        <p>"Raw editor"</p>
        <Textarea value placeholder="Textarea" resize=TextareaResize::Both />
        <button on:click=move |_| {
            let s = value.get();
            match serde_json::from_str::<serde_json::Value>(&s) {
                Ok(s) => {
                    set_config_server_action(s);
                }
                Err(e) => {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message(&format!("Error parsing JSON: {e:?}"))
                        .unwrap();
                }
            }
        }>"Save"</button>
    }
}
