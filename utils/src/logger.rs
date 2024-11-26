use tracing::{
    Level,
    collect::set_global_default,
};
use tracing_subscriber::{
    fmt::{
        self,
        writer::MakeWriterExt,
    }, subscribe::CollectExt, EnvFilter
};

use crate::{
    EnvConfig,
    config,
};

pub fn init() {
    let stdout = config().env == EnvConfig::Development;

    let log_config = &config().log;

    let database_file = tracing_appender::rolling::daily(
        log_config.database.dir.as_ref().unwrap_or(&log_config.dir),
        &log_config.database.prefix,
    )
    .with_filter(|meta| {
        Some(meta.target()) == log_config.database.target.as_deref()
    });
    let other_file = tracing_appender::rolling::daily(
        log_config.other.dir.as_ref().unwrap_or(&log_config.dir),
        &log_config.other.prefix,
    );
    let error_file = tracing_appender::rolling::daily(
        log_config.error.dir.as_ref().unwrap_or(&log_config.dir),
        &log_config.error.prefix,
    )
    .with_max_level(Level::ERROR);

    let all_file = other_file.and(database_file).and(error_file);

    let file_subscriber = fmt::subscriber()
        .json()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(all_file);

    let io_subscriber = if stdout {
        Some(
            fmt::subscriber()
                .compact()
                .pretty()
                .with_ansi(true)
                .with_target(true)
                .with_line_number(false)
                .with_file(false)
                .with_writer(std::io::stdout),
        )
    } else {
        None
    };

    let collector = tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::TRACE.into()),
        )
        .with(io_subscriber)
        .with(file_subscriber);

    set_global_default(collector).unwrap_or_else(|e| {
        panic!("💥 Failed to setting tracing subscriber: {e:?}");
    });
}
