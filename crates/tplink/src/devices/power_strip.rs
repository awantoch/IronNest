use {
    crate::{client::TpLinkClient, error::Result},
    serde_json::{json, Value},
};

/// Power strip operations
impl TpLinkClient {
    /// Turns off a specific socket on a smart power strip
    pub async fn turn_power_strip_socket_off(&self, ip: &str, socket_id: &str) -> Result<()> {
        self.send(
            ip,
            json!({
                "context": {
                    "child_ids": [socket_id]
                },
                "system":{"set_relay_state":{"state": 0}}
            }),
        )
        .await?;
        Ok(())
    }

    /// Turns on a specific socket on a smart power strip
    pub async fn turn_power_strip_socket_on(&self, ip: &str, socket_id: &str) -> Result<()> {
        self.send(
            ip,
            json!({
                "context": {
                    "child_ids": [socket_id]
                },
                "system":{"set_relay_state":{"state": 1}}
            }),
        )
        .await?;
        Ok(())
    }

    /// Gets energy usage data for a specific socket on a smart power strip
    pub async fn get_power_strip_energy_usage(&self, ip: &str, socket_id: &str) -> Result<Value> {
        let response = self
            .send(
                ip,
                json!({
                    "context": {
                        "child_ids": [socket_id]
                    },
                    "system":{"get_energy_usage":{}}
                }),
            )
            .await?;
        Ok(response)
    }

    /// Gets emeter data for a specific socket on a smart power strip
    pub async fn get_power_strip_emeter_data(&self, ip: &str, socket_id: &str) -> Result<Value> {
        let response = self
            .send(
                ip,
                json!({
                    "context": {
                        "child_ids": [socket_id]
                    },
                    "system":{"get_emeter_data":{}}
                }),
            )
            .await?;
        Ok(response)
    }
}

// Standalone functions for backwards compatibility
pub async fn turn_power_strip_socket_off(ip: &str, socket_id: &str) -> Result<()> {
    TpLinkClient::new()
        .turn_power_strip_socket_off(ip, socket_id)
        .await
}

pub async fn turn_power_strip_socket_on(ip: &str, socket_id: &str) -> Result<()> {
    TpLinkClient::new()
        .turn_power_strip_socket_on(ip, socket_id)
        .await
}

pub async fn get_power_strip_energy_usage(ip: &str, socket_id: &str) -> Result<Value> {
    TpLinkClient::new()
        .get_power_strip_energy_usage(ip, socket_id)
        .await
}

pub async fn get_power_strip_emeter_data(ip: &str, socket_id: &str) -> Result<Value> {
    TpLinkClient::new()
        .get_power_strip_emeter_data(ip, socket_id)
        .await
}
