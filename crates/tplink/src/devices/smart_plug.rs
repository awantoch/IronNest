use {
    crate::{client::TpLinkClient, error::Result},
    serde_json::json,
};

/// Smart plug operations
impl TpLinkClient {
    /// Turns on a smart plug
    pub async fn turn_plug_on(&self, ip: &str) -> Result<()> {
        self.send(ip, json!({"system":{"set_relay_state":{"state": 1}}}))
            .await?;
        Ok(())
    }

    /// Turns off a smart plug
    pub async fn turn_plug_off(&self, ip: &str) -> Result<()> {
        self.send(ip, json!({"system":{"set_relay_state":{"state": 0}}}))
            .await?;
        Ok(())
    }

    /// Sets the alias (name) of a smart plug
    pub async fn set_plug_alias(&self, ip: &str, alias: &str) -> Result<()> {
        self.send(ip, json!({"system":{"set_dev_alias":{"alias": alias}}}))
            .await?;
        Ok(())
    }

    /// Reboots a smart plug
    pub async fn reboot_plug(&self, ip: &str) -> Result<()> {
        self.send(ip, json!({"system":{"reboot":{"delay": 1}}}))
            .await?;
        Ok(())
    }
}

// Standalone functions for backwards compatibility
pub async fn turn_plug_on(ip: &str) -> Result<()> {
    TpLinkClient::new().turn_plug_on(ip).await
}

pub async fn turn_plug_off(ip: &str) -> Result<()> {
    TpLinkClient::new().turn_plug_off(ip).await
}

pub async fn set_plug_alias(ip: &str, alias: &str) -> Result<()> {
    TpLinkClient::new().set_plug_alias(ip, alias).await
}

pub async fn reboot_plug(ip: &str) -> Result<()> {
    TpLinkClient::new().reboot_plug(ip).await
}
