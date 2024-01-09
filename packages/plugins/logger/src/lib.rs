use std::sync::Arc;

use env_logger::Env;
use log::{debug, error, info, warn};
use polywrap_plugin::*;
use std::fmt::Debug;
use wrap::{
    module::{ArgsLog, Module},
    types::LogLevel,
    wrap_info::get_manifest,
};

pub mod wrap;
pub use env_logger;
use std::result::Result;

// 1. Define a new trait
pub trait LogFuncTrait: Fn(LogLevel, &str) + Debug + Send + Sync {}

// 2. Implement this new trait for all types that implement the desired behaviors.
impl<T> LogFuncTrait for T where T: Fn(LogLevel, &str) + Debug + Send + Sync {}

#[derive(Debug)]
pub struct LoggerPlugin {
    log_func: Option<Box<dyn LogFuncTrait>>,
}

impl LoggerPlugin {
    pub fn new(custom_logger: Option<Box<dyn LogFuncTrait>>) -> Self {
        Self {
            log_func: custom_logger,
        }
    }
}

#[plugin_impl]
impl Module for LoggerPlugin {
    fn log(&mut self, args: &ArgsLog, _: Arc<dyn Invoker>) -> Result<bool, PluginError> {
        env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
        match self.log_func {
            Some(ref func) => {
                func(args.level, &args.message);
            }
            None => match args.level {
                LogLevel::DEBUG => debug!("{}", args.message),
                LogLevel::WARN => warn!("{}", args.message),
                LogLevel::ERROR => error!("{}", args.message),
                LogLevel::INFO => info!("{}", args.message),
                _ => {
                    return Err(PluginError::InvocationError {
                        exception: format!("Unknown log level"),
                    });
                }
            },
        }
        return Ok(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polywrap_client::{
        builder::{ClientConfig, ClientConfigBuilder},
        client::Client,
    };
    use polywrap_core::{client::CoreClientConfigBuilder, macros::uri, uri::Uri};
    use polywrap_msgpack_serde::to_vec;
    use polywrap_plugin::package::PluginPackage;

    #[test]
    fn test_default_logging() {
        let log_args = ArgsLog {
            level: LogLevel::INFO,
            message: String::from("Info message"),
        };

        let mut builder = ClientConfig::new();
        let logger_plugin = LoggerPlugin::new(None);
        let logger_package: PluginPackage<LoggerPlugin> = PluginPackage::from(logger_plugin);
        builder.add_package(uri!("plugin/logger"), Arc::new(logger_package));
        let client = Client::new(builder.build());
        let result = client.invoke::<bool>(
            &uri!("plugin/logger"),
            "log",
            Some(&to_vec(&log_args).unwrap()),
            None,
            None,
        );
        assert!(result.is_ok());
    }
}
