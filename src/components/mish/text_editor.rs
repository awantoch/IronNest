use {
    leptos::prelude::*,
    thaw::{Button, Textarea, TextareaResize},
};

#[component]
pub fn TextEditor(
    state: String,
    set_config_server_action: impl Fn(String) + 'static,
) -> impl IntoView {
    let value = RwSignal::new(state);
    view! {
        // <p>"Text editor"</p>
        <div>
            <Textarea value resize=TextareaResize::Both />
            <Button on:click=move |_| {
                let s = value.get();
                web_sys::console::log_1(&format!("state: {s:?}").into());
                set_config_server_action(s);
            }>"Save"</Button>
        </div>
    }
}
