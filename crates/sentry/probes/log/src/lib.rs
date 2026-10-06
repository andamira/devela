#![no_std]

use devela::DiagOut;

pub fn exercise_diag<O: DiagOut + ?Sized>(out: &mut O) -> Result<(), O::Error> {
    out.trace("trace")?;
    out.debug("debug")?;
    out.info("info")?;
    out.warn("warn")?;
    out.error("error")?;
    out.critical("critical")?;
    Ok(())
}
