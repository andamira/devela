#[cfg(target_os = "android")]
fn main() {
    use devela::AndroidLog;
    use sentry_log::exercise_diag;

    let mut out = AndroidLog::new(c"devela-sentry");
    exercise_diag(&mut out).unwrap();
}

#[cfg(not(target_os = "android"))]
fn main() {}
