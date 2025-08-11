use std::error::Error;
use semver::Version;

#[derive(Debug, Clone)]
pub struct ChannelVersion {
    pub channel: String,
    pub version: Version,
}

#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("Invalid version string: {0}")]
    InvalidVersion(String),
    #[error("Invalid channel name: {0}")]
    InvalidChannel(String),
}

impl ChannelVersion {
    pub fn new(channel: &str, version: &str) -> Result<Self, ChannelError> {
        if channel.is_empty() {
            return Err(ChannelError::InvalidChannel("Channel name cannot be empty".to_string()));
        }

        let version = Version::parse(version)
            .map_err(|_| ChannelError::InvalidVersion(version.to_string()))?;

        Ok(Self {
            channel: channel.to_string(),
            version,
        })
    }
}

pub fn get_version() -> String {
    // For backwards compatibility
    "0.1.0".to_string()
}

pub fn get_channel_version(channel: &str) -> Result<ChannelVersion, ChannelError> {
    match channel {
        "stable" => ChannelVersion::new("stable", "0.1.0"),
        "beta" => ChannelVersion::new("beta", "0.2.0-beta"),
        "nightly" => ChannelVersion::new("nightly", "0.2.0-nightly"),
        _ => Err(ChannelError::InvalidChannel(format!("Unknown channel: {}", channel)))
    }
}
