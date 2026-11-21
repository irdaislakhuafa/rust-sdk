use crate::appcontext::AppContext;
use crate::appcontext::Interface;
use crate::log::config::Config;
use crate::log::level::Level;
use crate::log::message::{DebugMsg, LogMessage};
use crate::log::sink::{CompositeSink, LogSink, WriterSink};
use crate::log::{Logger, StorageDriver, StorageOpt, init};
use std::sync::{Arc, Mutex};

fn make_ctx() -> AppContext {
    let mut c = AppContext::new();
    c.set_request_id("req-1")
        .set_user_agent("ua")
        .set_service_version("v1")
        .set_request_start_time(std::time::SystemTime::now());
    c
}

#[test]
fn test_level_parse() {
    struct Case {
        name: &'static str,
        input: &'static str,
        ok: bool,
        expected: Option<Level>,
    }
    let cases = [
        Case {
            name: "trace",
            input: "trace",
            ok: true,
            expected: Some(Level::Trace),
        },
        Case {
            name: "debug",
            input: "DEBUG",
            ok: true,
            expected: Some(Level::Debug),
        },
        Case {
            name: "warn alias",
            input: "warning",
            ok: true,
            expected: Some(Level::Warn),
        },
        Case {
            name: "invalid",
            input: "nope",
            ok: false,
            expected: None,
        },
    ];
    for case in cases {
        let r = Level::parse(case.input);
        if case.ok {
            assert_eq!(r.unwrap(), case.expected.unwrap(), "case: {}", case.name);
        } else {
            assert!(r.is_err(), "case: {}", case.name);
        }
    }
}

#[test]
fn test_level_ordering_filter() {
    assert!(Level::Trace < Level::Info);
    assert!(Level::Error < Level::Fatal);
}

#[test]
fn test_config_parse_default_console() {
    let mut cfg = Config::default();
    cfg.parse_default();
    assert_eq!(cfg.skip_frame_count, 3);
}

#[test]
fn test_config_parse_default_file() {
    let mut cfg = Config {
        storage: StorageOpt {
            driver: StorageDriver::File,
            file_location: String::new(),
            rotation: Default::default(),
        },
        ..Default::default()
    };
    cfg.parse_default();
    assert_eq!(cfg.storage.file_location, "logs/current.json");
}

#[test]
fn test_log_message_str_and_debug() {
    let s = "hello";
    assert_eq!(s.to_log_message(), "hello");
    let d = DebugMsg(42i32);
    assert_eq!(d.to_log_message(), "42");
}

#[test]
fn test_console_emits_json() {
    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let sink = WriterSink::new(BufWriter { inner: buf.clone() });
    let mut cfg = Config::default();
    cfg.level = Level::Trace;
    cfg.storage.driver = StorageDriver::Console;
    let mut logger = crate::log::ConsoleLogger::with_sink(cfg, Box::new(sink));
    let ctx = make_ctx();
    logger.info(&ctx, &"hello world");
    let out = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    assert!(out.contains("\"level\":\"info\""), "case output: {}", out);
    assert!(
        out.contains("\"message\":\"hello world\""),
        "case output: {}",
        out
    );
    assert!(
        out.contains("\"request_id\":\"req-1\""),
        "case output: {}",
        out
    );
}

// helper writer
struct BufWriter {
    inner: Arc<Mutex<Vec<u8>>>,
}
impl std::io::Write for BufWriter {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.inner.lock().unwrap().extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_level_filtering() {
    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let sink = WriterSink::new(BufWriter { inner: buf.clone() });
    let mut cfg = Config::default();
    cfg.level = Level::Warn;
    cfg.storage.driver = StorageDriver::Console;
    let mut logger = crate::log::ConsoleLogger::with_sink(cfg, Box::new(sink));
    let ctx = make_ctx();
    logger.info(&ctx, &"should be filtered");
    logger.warn(&ctx, &"should appear");
    let out = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    assert!(!out.contains("should be filtered"), "case: {}", out);
    assert!(out.contains("should appear"), "case: {}", out);
}

#[test]
fn test_with_ctx_fields_replace_and_merge() {
    use crate::log::CtxFieldsFn;
    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let sink = WriterSink::new(BufWriter { inner: buf.clone() });
    let mut cfg = Config::default();
    cfg.level = Level::Trace;
    let mut logger = crate::log::ConsoleLogger::with_sink(cfg, Box::new(sink));

    let provider: CtxFieldsFn = Arc::new(|_ctx: &AppContext| {
        let mut m = serde_json::Map::new();
        m.insert(
            "custom".to_string(),
            serde_json::Value::String("v".to_string()),
        );
        m
    });

    // replace mode (default): only custom field, no base? Go replace returns custom only.
    logger.with_ctx_fields(Some(provider.clone()));
    let ctx = make_ctx();
    logger.info(&ctx, &"msg1");
    let out = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    // In replace mode we skip base fields
    assert!(out.contains("\"custom\":\"v\""), "out: {}", out);

    // merge mode
    buf.lock().unwrap().clear();
    logger.merged_fields();
    logger.info(&ctx, &"msg2");
    let out = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    assert!(out.contains("\"custom\":\"v\""), "out: {}", out);
    assert!(out.contains("\"request_id\":\"req-1\""), "out: {}", out);
}

#[test]
fn test_init_dispatch() {
    let mut cfg = Config::default();
    cfg.storage.driver = StorageDriver::Console;
    let l = init(cfg.clone()).unwrap();
    let ctx = make_ctx();
    l.info(&ctx, &"ok");

    // file init with temp dir
    let dir = std::env::temp_dir().join("rustsdk_log_test");
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join("t.json");
    let _ = std::fs::remove_file(&file);
    let mut cfg2 = Config::default();
    cfg2.storage = StorageOpt {
        driver: StorageDriver::File,
        file_location: file.to_string_lossy().to_string(),
        rotation: Default::default(),
    };
    let l2 = init(cfg2).unwrap();
    let ctx = make_ctx();
    l2.info(&ctx, &"file msg");
    l2.cleanup();
    let content = std::fs::read_to_string(&file).unwrap();
    assert!(content.contains("file msg"), "content: {}", content);
    let _ = std::fs::remove_file(&file);
}

#[test]
fn test_init_file_rotation() {
    let dir = std::env::temp_dir().join("rustsdk_log_rot");
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join("rot.json");
    let _ = std::fs::remove_file(&file);
    let mut cfg = Config::default();
    cfg.level = Level::Trace;
    cfg.storage = StorageOpt {
        driver: StorageDriver::File,
        file_location: file.to_string_lossy().to_string(),
        rotation: crate::log::StorageRotation {
            enable: true,
            max_size: 1, // 1 MB? our policy treats max_size as MB -> 1MB, too big for test. Use direct sink instead.
            max_backups: 2,
            max_age: 30,
            compress: false,
        },
    };
    // Instead test rotation via FileSink with small byte threshold by constructing policy directly.
    use crate::log::file_sink::{FileSink, SizeRotatePolicy};
    use std::sync::Arc;
    let policy = Arc::new(SizeRotatePolicy {
        max_size_bytes: 20,
        max_backups: 2,
    }) as Arc<dyn crate::log::file_sink::RotatePolicy>;
    let sink = FileSink::new(&file, Some(policy)).unwrap();
    let mut cfg2 = Config::default();
    cfg2.level = Level::Trace;
    let mut logger = crate::log::FileLogger::with_sink(cfg2, Box::new(sink));
    let ctx = make_ctx();
    // write enough to exceed 20 bytes
    for _ in 0..5 {
        logger.info(&ctx, &"this is a longer message to rotate");
    }
    logger.cleanup();
    // backup file should exist
    let backup = std::path::PathBuf::from(format!("{}.1", file.display()));
    assert!(backup.exists(), "backup should exist");
    let _ = std::fs::remove_file(&file);
    let _ = std::fs::remove_file(&backup);
}

#[test]
fn test_fatal_log_only_no_exit() {
    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let sink = WriterSink::new(BufWriter { inner: buf.clone() });
    let mut cfg = Config::default();
    cfg.level = Level::Trace;
    cfg.fatal_behavior = crate::log::FatalBehavior::LogOnly;
    let mut logger = crate::log::ConsoleLogger::with_sink(cfg, Box::new(sink));
    let ctx = make_ctx();
    logger.fatal(&ctx, &"boom");
    let out = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    assert!(out.contains("boom"));
}

#[test]
fn test_level_int_string_roundtrip() {
    for lvl in [
        Level::Trace,
        Level::Debug,
        Level::Info,
        Level::Warn,
        Level::Error,
        Level::Fatal,
        Level::Panic,
    ] {
        let s = lvl.as_str();
        assert_eq!(Level::parse(s).unwrap(), lvl);
        let i = lvl.as_i8();
        assert_eq!(Level::from_i8(i).unwrap(), lvl);
        assert_eq!(Level::try_from(i).unwrap(), lvl);
        assert_eq!(i8::from(lvl), i);
    }
}

#[test]
fn test_level_invalid_int() {
    assert!(Level::from_i8(99).is_err());
    assert!(Level::try_from(99i32).is_err());
}

#[test]
fn test_level_ordering() {
    assert!(Level::Trace < Level::Debug);
    assert!(Level::Debug < Level::Info);
    assert!(Level::Info < Level::Warn);
    assert!(Level::Warn < Level::Error);
    assert!(Level::Error < Level::Fatal);
    assert!(Level::Fatal < Level::Panic);
}

#[test]
fn test_init_multi_console_file() {
    let dir = std::env::temp_dir().join("rustsdk_multi");
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join("multi.json");
    let _ = std::fs::remove_file(&file);

    let logger = crate::log::init_multi(crate::log::MultiConfig {
        level: Level::Info,
        skip_frame_count: 3,
        fatal_behavior: crate::log::FatalBehavior::LogOnly,
        outputs: vec![
            crate::log::OutputConfig {
                driver: StorageDriver::Console,
                level: Some(Level::Info),
                ..Default::default()
            },
            crate::log::OutputConfig {
                driver: StorageDriver::File,
                level: Some(Level::Debug),
                file_location: file.to_string_lossy().to_string(),
                rotation: Default::default(),
            },
        ],
    })
    .unwrap();

    let ctx = make_ctx();
    logger.debug(&ctx, &"file only");
    logger.info(&ctx, &"both");
    logger.cleanup();

    let content = std::fs::read_to_string(&file).unwrap();
    assert!(content.contains("file only"), "content: {}", content);
    assert!(content.contains("both"), "content: {}", content);
    let _ = std::fs::remove_file(&file);
}

#[test]
fn test_init_multi_empty_outputs_defaults_console() {
    let mut cfg = crate::log::MultiConfig::default();
    cfg.outputs.clear();
    let logger = crate::log::init_multi(cfg).unwrap();
    let ctx = make_ctx();
    logger.info(&ctx, &"ok");
}

#[test]
fn test_fanout_fatal_coordination() {
    let buf1 = Arc::new(Mutex::new(Vec::<u8>::new()));
    let buf2 = Arc::new(Mutex::new(Vec::<u8>::new()));
    let s1 = WriterSink::new(BufWriter {
        inner: buf1.clone(),
    });
    let s2 = WriterSink::new(BufWriter {
        inner: buf2.clone(),
    });

    let mut cfg = Config::default();
    cfg.level = Level::Trace;
    cfg.fatal_behavior = crate::log::FatalBehavior::LogOnly;
    let l1 = crate::log::ConsoleLogger::with_sink(cfg.clone(), Box::new(s1));
    let l2 = crate::log::ConsoleLogger::with_sink(cfg, Box::new(s2));

    let mut logger = crate::log::fanout(
        vec![Box::new(l1), Box::new(l2)],
        crate::log::FatalBehavior::LogOnly,
    );
    let ctx = make_ctx();
    logger.fatal(&ctx, &"boom");
    let o1 = String::from_utf8(buf1.lock().unwrap().clone()).unwrap();
    let o2 = String::from_utf8(buf2.lock().unwrap().clone()).unwrap();
    assert!(o1.contains("boom"));
    assert!(o2.contains("boom"));
}

#[test]
fn test_composite_sink_best_effort() {
    // one ok sink, one failing sink (we simulate failure via a sink that errors)
    struct FailSink;
    impl LogSink for FailSink {
        fn write(&self, _entry: &[u8]) -> Result<(), crate::errors::SdkError> {
            Err(crate::errors::new_with_code(
                crate::codes::Code::LogWriteFailure,
                "fail".to_string(),
            ))
        }
    }
    impl std::fmt::Debug for FailSink {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("FailSink")
        }
    }

    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let ok = WriterSink::new(BufWriter { inner: buf.clone() });
    let composite = CompositeSink::new(vec![Box::new(ok), Box::new(FailSink)]);
    let entry = b"hello\n";
    let r = composite.write(entry);
    assert!(r.is_err());
    // ok sink still received bytes
    assert_eq!(buf.lock().unwrap().as_slice(), entry);
}

#[test]
fn test_init_with_sink() {
    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let sink = WriterSink::new(BufWriter { inner: buf.clone() });
    let logger = crate::log::init_with_sink(
        Level::Trace,
        Box::new(sink),
        crate::log::FatalBehavior::LogOnly,
    );
    let ctx = make_ctx();
    logger.info(&ctx, &"via sink");
    let out = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    assert!(out.contains("via sink"));
}

#[test]
fn test_fanout_ctx_fields() {
    use crate::log::CtxFieldsFn;
    let buf1 = Arc::new(Mutex::new(Vec::<u8>::new()));
    let buf2 = Arc::new(Mutex::new(Vec::<u8>::new()));
    let s1 = WriterSink::new(BufWriter {
        inner: buf1.clone(),
    });
    let s2 = WriterSink::new(BufWriter {
        inner: buf2.clone(),
    });

    let mut cfg = Config::default();
    cfg.level = Level::Trace;
    cfg.fatal_behavior = crate::log::FatalBehavior::LogOnly;
    let l1 = crate::log::ConsoleLogger::with_sink(cfg.clone(), Box::new(s1));
    let l2 = crate::log::ConsoleLogger::with_sink(cfg, Box::new(s2));

    let provider: CtxFieldsFn = Arc::new(|_ctx: &AppContext| {
        let mut m = serde_json::Map::new();
        m.insert(
            "custom".to_string(),
            serde_json::Value::String("v".to_string()),
        );
        m
    });

    let mut logger = crate::log::fanout(
        vec![Box::new(l1), Box::new(l2)],
        crate::log::FatalBehavior::LogOnly,
    );
    logger.with_ctx_fields(Some(provider));
    logger.merged_fields();
    let ctx = make_ctx();
    logger.info(&ctx, &"msg");

    for buf in [&buf1, &buf2] {
        let out = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
        assert!(out.contains("\"custom\":\"v\""), "out: {}", out);
        assert!(out.contains("\"request_id\":\"req-1\""), "out: {}", out);
    }
}
