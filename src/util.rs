use cfg_if::cfg_if;
use worker::{console_log, Date, Request};

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub(crate) fn log_request(req: &Request) {
    if let Some(cf) = req.cf() {
        console_log!(
            "{} - [{}], located at: {:?}, within: {}",
            Date::now().to_string(),
            req.path(),
            cf.coordinates().unwrap_or_default(),
            cf.region().unwrap_or_else(|| "unknown region".into())
        );
    } else {
        console_log!("{} - [{}],", Date::now().to_string(), req.path(),);
    }
}
