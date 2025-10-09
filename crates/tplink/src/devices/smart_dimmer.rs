use {
    crate::{
        client::TpLinkClient,
        error::{Result, TpLinkError},
    },
    serde_json::json,
};

/// Smart dimmer operations
impl TpLinkClient {
    /// Sets the brightness of a smart dimmer (0-100)
    pub async fn set_dimmer_brightness(&self, ip: &str, brightness: u8) -> Result<()> {
        if brightness > 100 {
            return Err(TpLinkError::DeviceCommunication {
                message: "Brightness must be between 0 and 100".to_string(),
            });
        }

        self.send(
            ip,
            json!({"smartlife.iot.dimmer":{"set_dimmer_transition":{"brightness": brightness, "duration": 1}}}),
        )
        .await?;
        Ok(())
    }

    /// Sets the inactivity timeout for a smart dimmer (in minutes)
    pub async fn set_dimmer_inactivity_timeout(&self, ip: &str, timeout_minutes: u8) -> Result<()> {
        self.send(
            ip,
            json!({"smartlife.iot.dimmer":{"set_cold_time": {"cold_time": timeout_minutes}}}),
        )
        .await?;
        Ok(())
    }
}

// Standalone functions for backwards compatibility
pub async fn set_dimmer_brightness(ip: &str, brightness: u8) -> Result<()> {
    TpLinkClient::new()
        .set_dimmer_brightness(ip, brightness)
        .await
}

pub async fn set_dimmer_inactivity_timeout(ip: &str, timeout_minutes: u8) -> Result<()> {
    TpLinkClient::new()
        .set_dimmer_inactivity_timeout(ip, timeout_minutes)
        .await
}
