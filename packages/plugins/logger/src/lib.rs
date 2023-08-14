use std::sync::Arc;

use log::{debug, error, info, warn};
use polywrap_core::invoker::Invoker;
use polywrap_plugin::{error::PluginError, implementor::plugin_impl};
use std::fmt::Debug;
use wrap::{
    module::{ArgsLog, Module},
    types::LogLevel,
    wrap_info::get_manifest,
};

pub mod wrap;

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
        println!("hola:)");
        match self.log_func {
            Some(ref func) => {
                func(args.level, &args.message);
            }
            None => match args.level {
                LogLevel::DEBUG => debug!("{}", args.message),
                LogLevel::WARN => warn!("{}", args.message),
                LogLevel::ERROR => error!("{}", args.message),
                LogLevel::INFO => println!("{}", args.message),
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
    use log::{LevelFilter, Metadata, Record};
    use polywrap_client::{
        builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
        client::PolywrapClient,
    };
    use polywrap_core::{client::ClientConfigBuilder, macros::uri, uri::Uri};
    use polywrap_msgpack_serde::to_vec;
    use polywrap_plugin::package::PluginPackage;
    use std::sync::Mutex;

    // This struct will capture logs for us during tests.
    struct TestLogger {
        messages: Arc<Mutex<Vec<(LogLevel, String)>>>,
    }

    impl log::Log for TestLogger {
        fn enabled(&self, _: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            let level = match record.level() {
                log::Level::Debug => LogLevel::DEBUG,
                log::Level::Warn => LogLevel::WARN,
                log::Level::Error => LogLevel::ERROR,
                log::Level::Info => LogLevel::INFO,
                _ => unimplemented!(),
            };
            self.messages
                .lock()
                .unwrap()
                .push((level, format!("{}", record.args())));
        }

        fn flush(&self) {}
    }

    fn init_test_logger() -> Arc<Mutex<Vec<(LogLevel, String)>>> {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let logger = TestLogger {
            messages: messages.clone(),
        };
        log::set_boxed_logger(Box::new(logger)).unwrap();
        log::set_max_level(LevelFilter::Debug);
        messages
    }

    #[test]
    fn test_default_logging() {
        let messages = init_test_logger();

        let log_args = ArgsLog {
            level: LogLevel::INFO,
            message: String::from("Info message"),
        };

        let mut builder = PolywrapClientConfig::new();
        let logger_plugin = LoggerPlugin::new(None);
        let logger_package: PluginPackage<LoggerPlugin> = PluginPackage::from(logger_plugin);
        builder.add_package(uri!("plugin/logger"), Arc::new(logger_package));
        let client = PolywrapClient::new(builder.build());
        let result = client.invoke::<bool>(
            &uri!("plugin/logger"),
            "log",
            Some(&to_vec(&log_args).unwrap()),
            None,
            None,
        );
        assert!(result.is_ok());
        assert_eq!(
            messages.lock().unwrap().pop().unwrap(),
            (LogLevel::INFO, "Info message".to_string())
        );
    }
}
