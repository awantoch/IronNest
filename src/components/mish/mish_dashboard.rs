use {
    crate::components::ring_cameras::RingCameraPanelWithData,
    leptos::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MishDashboardValue {
    pub layout: MishDashboardLayout,
    pub panels: Vec<MishDashboardPanel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MishDashboardPanel {
    Ring { camera_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MishDashboardLayout {
    Grid { columns: i32 },
}

#[component]
pub fn MishDashboard(value: serde_json::Value) -> impl IntoView {
    let value = serde_json::from_value::<MishDashboardValue>(value);
    view! {
        {match value {
            Ok(value) => {
                view! {
                    <div style=match value.layout {
                        MishDashboardLayout::Grid { columns } => {
                            format!(
                                "display: grid; grid-template-columns: repeat({}, 1fr); gap: 10px;",
                                columns,
                            )
                        }
                    }>
                        {value
                            .panels
                            .iter()
                            .map(|panel| {
                                view! {
                                    <div>
                                        {match panel {
                                            MishDashboardPanel::Ring { camera_id } => {
                                                view! {
                                                    <RingCameraPanelWithData camera_id=camera_id.clone() />
                                                }
                                                    .into_any()
                                            }
                                        }}
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                }
                    .into_any()
            }
            Err(e) => view! { <p>{format!("Error: {e}")}</p> }.into_any(),
        }}
    }
}
