// devela/src/code/result/opt_res/_test_unwrap.rs

use crate::{OptRes, serr, sok, unwrap};

const OPTION_SOME: Option<bool> = Some(true);
const OPTION_NONE: Option<bool> = None;

const RESULT_OK: Result<bool, bool> = Ok(true);
const RESULT_ERR: Result<bool, bool> = Err(true);

const OPTRES_OK: OptRes<bool, bool> = sok(true);
const OPTRES_ERR: OptRes<bool, bool> = serr(true);
const OPTRES_NONE: OptRes<bool, bool> = None;

#[test]
fn test_unwrap_option() {
    assert![unwrap![some OPTION_SOME]];
    assert![unwrap![some_expect OPTION_SOME, "ERR"]];
    assert_eq![unwrap![some_or OPTION_SOME, false], true];
    assert_eq![unwrap![some_or OPTION_NONE, false], false];
}
#[test]
fn test_unwrap_result() {
    assert![unwrap![ok RESULT_OK]];
    assert![unwrap![ok_expect RESULT_OK, "ERR"]];
    assert_eq![unwrap![ok_or RESULT_OK, false], true];
    assert_eq![unwrap![ok_or RESULT_ERR, false], false];
    assert![unwrap![err RESULT_ERR]];
    assert![unwrap![err_expect RESULT_ERR, "ERR"]];
    assert_eq![unwrap![err_or RESULT_ERR, false], true];
    assert_eq![unwrap![err_or RESULT_OK, false], false];
}
#[test]
fn test_unwrap_optres() {
    assert![unwrap![sok OPTRES_OK]];
    assert![unwrap![sok_expect OPTRES_OK, "ERR"]];
    assert_eq![unwrap![sok_or OPTRES_OK, false], true];
    assert_eq![unwrap![sok_or OPTRES_ERR, false], false];
    assert_eq![unwrap![sok_or OPTRES_NONE, false], false];
    assert![unwrap![serr OPTRES_ERR]];
    assert![unwrap![serr_expect OPTRES_ERR, "ERR"]];
    assert_eq![unwrap![serr_or OPTRES_ERR, false], true];
    assert_eq![unwrap![serr_or OPTRES_OK, false], false];
    assert_eq![unwrap![serr_or OPTRES_NONE, false], false];
}

#[rustfmt::skip]
#[cfg(feature = "std")]
mod std {
    use super::*;
    use crate::Panic;

    #[test]
    fn test_unwrap_option_panic() {
        assert![Panic::catch(|| { assert![unwrap![some OPTION_NONE]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![some_expect OPTION_NONE, "ERR"]] }).is_err()];
    }
    #[test]
    fn test_unwrap_result_panic() {
        assert![Panic::catch(|| { assert![unwrap![ok RESULT_ERR]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![ok_expect RESULT_ERR, "ERR"]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![err RESULT_OK]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![err_expect RESULT_OK, "ERR"]] }).is_err()];
    }
    #[test]
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
}
