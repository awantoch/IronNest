use {
    crate::{
        error::{Result, TpLinkError},
        protocol::encrypt_with_header,
    },
    serde_json::Value,
    std::net::IpAddr,
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
    },
};

/// Core TP-Link client for device communication
pub struct TpLinkClient;

impl TpLinkClient {
    /// Creates a new TP-Link client
    pub fn new() -> Self {
        Self
    }

    /// Sends a JSON command to a TP-Link device and returns the response
    pub async fn send(&self, ip: &str, json: Value) -> Result<Value> {
        let ip_addr: IpAddr = ip
            .parse()
            .map_err(|_| TpLinkError::InvalidIpAddress(ip.to_string()))?;

        let mut stream = TcpStream::connect((ip_addr, 9999)).await?;

        let msg_bytes = serde_json::to_vec(&json)?;
        let discover_msg = encrypt_with_header(&msg_bytes);

        stream.write_all(&discover_msg).await?;

        let mut buf = [0u8; 1024];
        let bytes_read = stream.read(&mut buf).await?;

        if bytes_read == 0 {
            return Err(TpLinkError::DeviceCommunication {
                message: "No response from device".to_string(),
            });
        }

        let decrypted_msg = crate::protocol::decrypt_with_header(&buf);
        let response = serde_json::from_slice::<Value>(&decrypted_msg)?;

        log::trace!("Device response: {:#}", response);
        Ok(response)
    }
}

impl Default for TpLinkClient {
    fn default() -> Self {
        Self::new()
    }
}
