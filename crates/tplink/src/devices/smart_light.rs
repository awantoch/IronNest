use {
    crate::{
        client::TpLinkClient,
        error::{Result, TpLinkError},
    },
    serde_json::json,
};

// https://github.com/python-kasa/python-kasa/blob/123ea107b1e7536bc5dfc8b93111cc5c7e8d066b/tests/fakeprotocol_iot.py#L445
const LIGHT_SERVICE: &str = "smartlife.iot.smartbulb.lightingservice";

/// Smart light operations
impl TpLinkClient {
    /// Turns a smart light on or off
    pub async fn turn_light_on_off(&self, ip: &str, on: bool) -> Result<()> {
        let state = if on { 1 } else { 0 };
        self.send(
            ip,
            json!({LIGHT_SERVICE:{"transition_light_state":{"on_off":state,"transition_period":0}}}),
        )
        .await?;
        Ok(())
    }

    /// Sets the brightness of a smart light (0-100)
    pub async fn set_light_brightness(&self, ip: &str, brightness: u8) -> Result<()> {
        if brightness > 100 {
            return Err(TpLinkError::DeviceCommunication {
                message: "Brightness must be between 0 and 100".to_string(),
            });
        }

        self.send(
            ip,
            json!({LIGHT_SERVICE:{"transition_light_state":{"brightness":brightness,"transition_period":0}}}),
        )
        .await?;
        Ok(())
    }

    /// Sets the color of a smart light using CSS color format (e.g., "#FF0000", "red", "hsl(120, 50%, 50%)")
    pub async fn set_light_color(&self, ip: &str, color: &str) -> Result<()> {
        match csscolorparser::parse(color) {
            Ok(color) => {
                let [h, s, v, _a] = color.to_hsva();
                let hue = h as u8;
                let saturation = (s * 100.) as u8;
                let value = (v * 100.) as u8;
                let color_temp = 0u8;

                // https://github.com/python-kasa/python-kasa/blob/123ea107b1e7536bc5dfc8b93111cc5c7e8d066b/kasa/iot/iotbulb.py#L407
                self.send(
                    ip,
                    json!({
                        LIGHT_SERVICE:{
                            "transition_light_state":{
                                "hue": hue,
                                "saturation": saturation,
                                "brightness": value,
                                "color_temp": color_temp,
                                "transition_period": 0
                            }
                        }
                    }),
                )
                .await?;
                Ok(())
            }
            Err(e) => Err(TpLinkError::InvalidColor(format!(
                "Failed to parse color '{color}': {e}"
            ))),
        }
    }
}

// Standalone functions for backwards compatibility
pub async fn turn_light_on_off(ip: &str, on: bool) -> Result<()> {
    TpLinkClient::new().turn_light_on_off(ip, on).await
}

pub async fn set_light_brightness(ip: &str, brightness: u8) -> Result<()> {
    TpLinkClient::new()
        .set_light_brightness(ip, brightness)
        .await
}

pub async fn set_light_color(ip: &str, color: &str) -> Result<()> {
    TpLinkClient::new().set_light_color(ip, color).await
}
