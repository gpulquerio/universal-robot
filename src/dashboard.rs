//! Port Communication for the Basic Dashboard Commands of the Universal Robot

#[cfg(test)]
mod test;

pub mod commands;
pub mod helpers;
pub mod query;
pub mod safety;
pub mod types;

use std::net::IpAddr;

use crate::physical::UrPort;
use crate::prelude::*;

/// Dashboard Server
///
/// The dashboard server can be used to:
/// • Load and play programs
/// • power on and brake release
/// • query robot status
/// • set operational mode
/// <https://s3-eu-west-1.amazonaws.com/ur-support-site/42728/DashboardServer_e-Series_2022.pdf>
pub struct Dashboard {
    port: UrPort,
    latest_message: String,
}

impl Dashboard {
    const DASHBOARD_PORT: u16 = 29999;
    /// Initialize connection to the dashboard server port
    pub fn new(host: IpAddr, timeout: Option<Duration>) -> Result<Self> {
        let mut port = UrPort::new(host, timeout, Self::DASHBOARD_PORT)?;
        println!("{}({})", port.read()?, host);
        let mut dashboard = Dashboard {
            port,
            latest_message: String::new(),
        };
        dashboard.log("connected to Rust")?;
        Ok(dashboard)
    }
    /// Private boilerplate function to send a command or query to the dashboard server with an expected response pattern
    ///
    /// The response pattern is case insensitive
    fn send(&mut self, payload: &str, response_contains: &str) -> Result<String> {
        let response = self.port.write(payload)?.to_lowercase();
        self.latest_message = response.clone();
        if response.contains(&response_contains.to_lowercase()) {
            Ok(response)
        } else {
            Err(Error::UnexpectedResponse(response))
        }
    }
    /// Get the latest message that was received by the Dashboard server.
    ///
    /// This is cached and overwritten every time a new message is read.
    pub fn latest_message(&self) -> String {
        self.latest_message.to_owned()
    }
    /// End connection to the dashboard server port
    pub fn close(mut self) -> Result<()> {
        self.send("quit", "disconnected")?;
        self.port.close()
    }
}
