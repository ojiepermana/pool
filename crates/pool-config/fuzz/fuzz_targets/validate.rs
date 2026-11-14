#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    match pool_config::validate(data) {
        Ok(config) => assert_eq!(
            config.config_version(),
            pool_config::SUPPORTED_CONFIG_VERSION
        ),
        Err(rejection) => assert!(!rejection.findings().is_empty()),
    }
});
