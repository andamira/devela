use devela::Io;
use sentry_log::exercise_diag;

fn main() {
    let mut out = Io::stderr();
    exercise_diag(&mut out).unwrap();
}
