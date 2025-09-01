use {
    crate::components::ring_cameras::RingCameraPanelWithData,
    leptos::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MishDashboardValue {
    pub panels: Vec<MishDashboardPanel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MishDashboardPanel {
    Ring { camera_id: String },
}

#[component]
pub fn MishDashboard(value: serde_json::Value) -> impl IntoView {
    let value = serde_json::from_value::<MishDashboardValue>(value);
    view! {
        {match value {
            Ok(value) => {
                view! {
                    {value
                        .panels
                        .iter()
                        .map(|panel| {
                            match panel {
                                MishDashboardPanel::Ring { camera_id } => {
                                    view! {
                                        <RingCameraPanelWithData camera_id=camera_id.clone() />
                                    }
                                        .into_any()
                                }
                            }
                        })
                        .collect::<Vec<_>>()}
                }
                    .into_any()
            }
            Err(e) => view! { <p>{format!("Error: {e}")}</p> }.into_any(),
        }}
    }
}
