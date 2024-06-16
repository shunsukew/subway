use async_trait::async_trait;
use serde::Deserialize;
use garde::Validate;

use super::{Extension, ExtensionRegistry};

pub struct Logging {
    pub config: LoggingConfig,
}

#[derive(Deserialize, Validate, Debug, Clone)]
#[garde(allow_unvalidated)]
pub struct LoggingConfig {
    pub format: LogFormat,
    #[garde(custom(validate_output_config))]
    pub output: Output,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    Json,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Output {
    #[serde(rename = "type")]
    output_type: OutputType,
    path: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutputType {
    File,
    Syslog,
}

fn validate_output_config(output: &Output, _context: &()) -> garde::Result {
    tracing::info!("Validating output: {:?}", output);
    if output.output_type == OutputType::File && output.path.is_none() {
        Err(garde::Error::new("path is required for file output"))?;
    }
    tracing::info!("Output is valid");

    return Ok(())
}

#[async_trait]
impl Extension for Logging {
    type Config = LoggingConfig;

    async fn from_config(config: &Self::Config, _registry: &ExtensionRegistry) -> Result<Self, anyhow::Error> {
        config.validate(&())?;
        Ok(Self::new(config.clone()))
    }
}

impl Logging {
    pub fn new(config: LoggingConfig) -> Self {
        tracing::info!("Logging config: {:?}", config);

        Self { config }
    }
}
