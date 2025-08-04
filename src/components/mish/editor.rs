use {
    crate::components::mish::{
        json_editor::JsonEditor, mish_button::MishButton, number_editor::NumberEditor,
        text_editor::TextEditor,
    },
    leptos::prelude::*,
    std::sync::Arc,
};

#[component]
pub fn Editor(
    state: serde_json::Value,
    action: impl Fn(serde_json::Value) + 'static + Send + Sync,
) -> impl IntoView {
    let action = Arc::new(action);
    let raw_editor_mode = RwSignal::new(false);

    let editor = {
        let state = state.clone();
        move || {
            let state = state.clone();
            let action = action.clone();
            if raw_editor_mode.get() {
                view! { <JsonEditor state=Some(state) set_config_server_action=move |s| { action(s) } /> }.into_any()
            } else {
                match state.clone() {
                    serde_json::Value::Bool(b) => view! {
                        <input
                            type="checkbox"
                            checked=b
                            on:input:target=move |ev| {
                                let value = ev.target().checked();
                                action(serde_json::Value::Bool(value));
                            }
                        />
                    }
                    .into_any(),
                    serde_json::Value::String(s) => view! {
                        <TextEditor
                            state=s
                            set_config_server_action=move |s| {
                                action(serde_json::Value::String(s))
                            }
                        />
                    }
                    .into_any(),
                    serde_json::Value::Number(n) => {
                        view! { <NumberEditor state=n.to_string() set_config_server_action=move |s| { action(s) } /> }
                            .into_any()
                    }
                    serde_json::Value::Array(a) => {
                        let action = Arc::new(action);
                        a.iter()
                            .enumerate()
                            .map(|(i, v)| {
                                let a = a.clone();
                                let action = action.clone();
                                view! {
                                    <p>Array: {i}</p>
                                    <div style="border-left: 5px solid black">
                                        <NestedEditor
                                            state=v.clone()
                                            action=Box::new(move |s| {
                                                let mut a = a.clone();
                                                a[i] = s;
                                                action(serde_json::Value::Array(a));
                                            })
                                        />
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                            .into_any()
                    }
                    serde_json::Value::Object(o) => {
                        let action = Arc::new(action);
                        if let Some(v) = o.get("-") && let Some(v) = v.as_str() && v == "mish-button" {
                            view! { <MishButton value=state.clone() /> }.into_any()
                        } else if let Some(v) = o.get("/") && let Some(cid) = v.as_str() {
                            view! { <a href=format!("/settings/dag-inspector/ipld-blob/{cid}")>"CID: "{cid.to_string()}</a> }.into_any()
                        } else {
                            o.clone()
                                .into_iter()
                                .map(|(k, v)| {
                                    let o = o.clone();
                                    let action = action.clone();
                                    view! {
                                        <div>"Key: "{k.clone()}</div>
                                        <div>
                                            "Value: " {serde_json::to_string_pretty(&v).unwrap()}
                                        </div>
                                        <div style="border-left: 5px solid black">
                                            <NestedEditor
                                                state=v.clone()
                                                action=Box::new(move |s| {
                                                    let mut o = o.clone();
                                                    o.insert(k.clone(), s);
                                                    action(serde_json::Value::Object(o));
                                                })
                                            />
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                                .into_any()
                        }
                    }
                    _ => view! { <JsonEditor state=Some(state) set_config_server_action=move |s| { action(s) } /> }
                        .into_any(),
                }
            }
        }
    };

    view! {
        <div>
            <label for="raw-editor-mode">"RAW editor mode"</label>
            <input type="checkbox" id="raw-editor-mode" bind:checked=raw_editor_mode />
        </div>
        <div>{serde_json::to_string_pretty(&state).unwrap()}</div>
        {editor}
    }
}

#[component]
pub fn NestedEditor(
    state: serde_json::Value,
    action: Box<dyn Fn(serde_json::Value) + 'static + Send + Sync>,
) -> impl IntoView {
    view! { <Editor state=state action=action /> }
}
