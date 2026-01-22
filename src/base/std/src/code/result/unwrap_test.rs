// devela_base_std::code::result::unwrap_test

use crate::{OptRes, Panic, serr, sok, unwrap};

// const OPTION_SOME: Option<bool> = Some(true);
const OPTION_NONE: Option<bool> = None;

const RESULT_OK: Result<bool, bool> = Ok(true);
const RESULT_ERR: Result<bool, bool> = Err(true);

const OPTRES_OK: OptRes<bool, bool> = sok(true);
const OPTRES_ERR: OptRes<bool, bool> = serr(true);
const OPTRES_NONE: OptRes<bool, bool> = None;

#[test] #[rustfmt::skip]
fn test_unwrap_option_panic() {
    assert![Panic::catch(|| { assert![unwrap![some OPTION_NONE]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![some_expect OPTION_NONE, "ERR"]] }).is_err()];
}

#[test] #[rustfmt::skip]
fn test_unwrap_result_panic() {
    assert![Panic::catch(|| { assert![unwrap![ok RESULT_ERR]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![ok_expect RESULT_ERR, "ERR"]] }).is_err()];

    assert![Panic::catch(|| { assert![unwrap![err RESULT_OK]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![err_expect RESULT_OK, "ERR"]] }).is_err()];
}

#[test] #[rustfmt::skip]
fn test_unwrap_optres_panic() {
    assert![Panic::catch(|| { assert![unwrap![sok OPTRES_ERR]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![sok OPTRES_NONE]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![sok_expect OPTRES_ERR, "ERR"]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![sok_expect OPTRES_NONE, "ERR"]] }).is_err()];

    assert![Panic::catch(|| { assert![unwrap![serr OPTRES_OK]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![serr OPTRES_NONE]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![serr_expect OPTRES_OK, "ERR"]] }).is_err()];
    assert![Panic::catch(|| { assert![unwrap![serr_expect OPTRES_NONE, "ERR"]] }).is_err()];
}
