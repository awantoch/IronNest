use {
    leptos::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MishButtonValue {
    pub label: String,
    pub action: MishButtonAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MishButtonAction {
    Log(String),
}

#[component]
pub fn MishButton(value: serde_json::Value) -> impl IntoView {
    let value = serde_json::from_value::<MishButtonValue>(value);
    view! {
        {match value {
            Ok(value) => {
                view! {
                    <button on:click=move |_| {
                        let action = value.action.clone();
                        match action {
                            MishButtonAction::Log(s) => {
                                web_sys::console::log_1(&format!("action: {s}").into());
                            }
                        }
                    }>{value.label}</button>
                }
                    .into_any()
            }
            Err(e) => view! { <p>{format!("Error: {e}")}</p> }.into_any(),
        }}
    }
}
