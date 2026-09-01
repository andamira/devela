// devela/src/lang/prog/script/machine/state.rs
//
//! Defines [`ScriptMachine`].
//

use crate::{ConstInit, Infallible, ValueKind, is, unwrap, whilst};
use crate::{ScriptCall, ScriptError, ScriptHost, ScriptOp, ScriptOutcome, ScriptValue};
use ScriptError as Error;

#[doc = crate::_tags!(lang runtime state)]
/// A fixed-stack resumable scripting machine.
#[doc = crate::_doc_meta!{
    location("lang/prog/script/machine"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(ScriptMachine<u32, 4> = 56|448; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(ScriptMachine<u32, 4> = 80|640; niche Option),
}]
/// The machine stores suspended execution state independently of the
/// program being executed. A host may therefore retain a machine between
/// calls and resume it against the same loaded program.
///
/// `R` is the opaque reference payload stored by [`ScriptValue::Ref`].
/// The machine only requires it to be [`Copy`]; its meaning and validity
/// belong to the caller.
///
/// `STACK` is the maximum number of live stack values.
///
/// # Execution
///
/// [`step`][Self::step] and [`run`][Self::run] execute machine operations
/// directly. Host calls return to the caller as [`ScriptOutcome::HostCall`]
/// and may be completed with [`complete_call`][Self::complete_call].
///
/// [`step_with`][Self::step_with] and [`run_with`][Self::run_with] are
/// convenience forms that resolve those calls through a [`ScriptHost`].
///
/// Reaching the end of `program` completes execution as if it had returned,
/// using the top stack value when present.
///
/// # Execution errors
///
/// A failing operation is atomic with respect to machine state: its
/// instruction position and logical stack remain unchanged.
///
/// Host-side effects performed before a [`ScriptHost`] returns an error
/// are outside this guarantee and are not rolled back.
///
/// # Example
/// ```
/// use devela::{ScriptMachine, ScriptOp, ScriptOutcome, ScriptValue};
///
/// type V = ScriptValue<()>;
/// let program = [
///     ScriptOp::Push(V::Int(20)),
///     ScriptOp::Push(V::Int(22)),
///     ScriptOp::Add,
/// ];
/// let mut machine = ScriptMachine::<(), 2>::new();
///
/// assert_eq!(
///     machine.run(&program, 3),
///     Ok(ScriptOutcome::Returned(Some(V::Int(42)))),
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptMachine<R: Copy, const STACK: usize> {
    ip: usize,
    len: usize,
    stack: [ScriptValue<R>; STACK],
}

impl<R: Copy, const STACK: usize> Default for ScriptMachine<R, STACK> {
    fn default() -> Self {
        Self::new()
    }
}
impl<R: Copy, const STACK: usize> ConstInit for ScriptMachine<R, STACK> {
    const INIT: Self = Self::new();
}

/* state and stack */
impl<R: Copy, const STACK: usize> ScriptMachine<R, STACK> {
    /* construction */

    /// Creates an empty machine positioned at the first operation.
    pub const fn new() -> Self {
        Self { ip: 0, len: 0, stack: [ScriptValue::Nil; STACK] }
    }

    /* queries */

    /// Returns the current instruction position.
    pub const fn ip(&self) -> usize {
        self.ip
    }
    /// Returns the number of live stack values.
    pub const fn stack_len(&self) -> usize {
        self.len
    }
    /// Returns the maximum number of live stack values.
    pub const fn stack_capacity(&self) -> usize {
        STACK
    }
    /// Returns whether the stack contains no live values.
    pub const fn stack_is_empty(&self) -> bool {
        self.len == 0
    }
    /// Returns whether the stack is full.
    pub const fn stack_is_full(&self) -> bool {
        self.len == STACK
    }

    /* state */

    /// Resets execution and clears the logical stack.
    pub const fn reset(&mut self) {
        self.ip = 0;
        self.len = 0;
    }

    /* stack */

    /// Attempts to push a value onto the stack.
    ///
    /// Returns `false` when the stack is full.
    pub const fn push(&mut self, value: ScriptValue<R>) -> bool {
        if self.len == STACK {
            false
        } else {
            self.stack[self.len] = value;
            self.len += 1;
            true
        }
    }
    /// Removes and returns the top stack value.
    pub const fn pop(&mut self) -> Option<ScriptValue<R>> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.stack[self.len])
        }
    }
    /// Returns the top stack value.
    pub const fn peek(&self) -> Option<&ScriptValue<R>> {
        if self.len == 0 { None } else { Some(&self.stack[self.len - 1]) }
    }
    /// Returns the live portion of the stack.
    pub fn stack(&self) -> &[ScriptValue<R>] {
        &self.stack[..self.len]
    }
}

/* direct execution */
impl<R: Copy, const STACK: usize> ScriptMachine<R, STACK> {
    /* helpers */

    const fn return_value(&self) -> Option<ScriptValue<R>> {
        is! { self.len == 0, None, Some(self.stack[self.len - 1]) }
    }
    const fn after_advance(&self, program_len: usize) -> Option<ScriptOutcome<R>> {
        is! { self.ip == program_len, Some(ScriptOutcome::Returned(self.return_value())), None }
    }
    const fn jump_target(ip: usize, offset: isize, program_len: usize) -> Option<usize> {
        let target = is! { offset >= 0,
            unwrap![some? ip.checked_add(offset as usize)],
            unwrap![some? ip.checked_sub(offset.unsigned_abs())]
        };
        is! { target <= program_len, Some(target), None }
    }

    /* execution */

    /// Executes one script operation.
    ///
    /// Host operations are returned as [`ScriptOutcome::HostCall`] and leave
    /// the machine unchanged until completed with [`complete_call`](Self::complete_call).
    ///
    /// # Execution errors
    ///
    /// A failing operation is atomic with respect to machine state:
    /// its instruction position and logical stack remain unchanged.
    pub const fn step(
        &mut self,
        program: &[ScriptOp<R>],
    ) -> Result<Option<ScriptOutcome<R>>, Error> {
        self.step_inner::<Infallible>(program)
    }

    const fn step_inner<E>(
        &mut self,
        program: &[ScriptOp<R>],
    ) -> Result<Option<ScriptOutcome<R>>, Error<E>> {
        let program_len = program.len();
        is![self.ip > program_len, return Err(Error::invalid_ip(self.ip, program_len))];
        is![self.ip == program_len, return Ok(Some(ScriptOutcome::Returned(self.return_value())))];
        let op = program[self.ip];
        match op {
            ScriptOp::Push(value) => {
                is! { self.len == STACK, return Err(Error::stack_overflow(STACK)); }
                self.stack[self.len] = value;
                self.len += 1;
                self.ip += 1;
                Ok(self.after_advance(program_len))
            }
            ScriptOp::Drop => {
                is! { self.len == 0, return Err(Error::stack_underflow(1, 0)); }
                self.len -= 1;
                self.ip += 1;
                Ok(self.after_advance(program_len))
            }
            ScriptOp::Dup => {
                is! { self.len == 0, return Err(Error::stack_underflow(1, 0)) }
                is! { self.len == STACK, return Err(Error::stack_overflow(STACK)) }
                self.stack[self.len] = self.stack[self.len - 1];
                self.len += 1;
                self.ip += 1;
                Ok(self.after_advance(program_len))
            }
            ScriptOp::Add => {
                is! {self.len < 2, return Err(Error::stack_underflow(2, self.len)) }
                let lhs = self.stack[self.len - 2];
                let rhs = self.stack[self.len - 1];
                let lhs = match lhs {
                    ScriptValue::Int(value) => value,
                    other => return Err(Error::expected_kind(ValueKind::Int, other.kind())),
                };
                let rhs = match rhs {
                    ScriptValue::Int(value) => value,
                    other => return Err(Error::expected_kind(ValueKind::Int, other.kind())),
                };
                let value = unwrap![some_ok_or? lhs.checked_add(rhs), Error::IntegerOverflow];
                self.stack[self.len - 2] = ScriptValue::Int(value);
                self.len -= 1;
                self.ip += 1;
                Ok(self.after_advance(program_len))
            }
            ScriptOp::Eq => {
                is! { self.len < 2, return Err(Error::stack_underflow(2, self.len)); }
                let lhs = self.stack[self.len - 2];
                let rhs = self.stack[self.len - 1];
                let equal = match (lhs, rhs) {
                    (ScriptValue::Nil, ScriptValue::Nil) => true,
                    (ScriptValue::Bool(a), ScriptValue::Bool(b)) => a == b,
                    (ScriptValue::Int(a), ScriptValue::Int(b)) => a == b,
                    (ScriptValue::Ref(_), _) | (_, ScriptValue::Ref(_)) => {
                        return Err(Error::unsupported_kind(ValueKind::Ref));
                    }
                    _ => false,
                };
                self.stack[self.len - 2] = ScriptValue::Bool(equal);
                self.len -= 1;
                self.ip += 1;
                Ok(self.after_advance(program_len))
            }
            ScriptOp::Jump(offset) => {
                let target = unwrap![some_ok_or? Self::jump_target(self.ip, offset, program_len),
                    Error::invalid_jump(self.ip, offset, program_len)];
                self.ip = target;
                Ok(self.after_advance(program_len))
            }
            ScriptOp::JumpIf(offset) => {
                is! { self.len == 0, return Err(Error::stack_underflow(1, 0)) }
                let condition = match self.stack[self.len - 1] {
                    ScriptValue::Bool(value) => value,
                    other => return Err(Error::expected_kind(ValueKind::Bool, other.kind())),
                };
                let target = if condition {
                    unwrap![some_ok_or? Self::jump_target(self.ip, offset, program_len),
                        Error::invalid_jump(self.ip, offset, program_len)]
                } else {
                    self.ip + 1
                };
                self.len -= 1;
                self.ip = target;
                Ok(self.after_advance(program_len))
            }
            ScriptOp::CallHost { id, arity } => {
                let arity = arity as usize;
                is! { self.len < arity, return Err(Error::stack_underflow(arity, self.len)) }
                let base = self.len - arity;
                // The call always produces one result.
                is! { base >= STACK, return Err(Error::stack_overflow(STACK)) }
                let call = ScriptCall::new(self.ip, self.len, id, arity as u8);
                Ok(Some(ScriptOutcome::HostCall(call)))
            }
            ScriptOp::Yield => {
                self.ip += 1;
                Ok(Some(ScriptOutcome::Yielded))
            }
            ScriptOp::Return => {
                self.ip = program_len;
                Ok(Some(ScriptOutcome::Returned(self.return_value())))
            }
        }
    }

    /// Runs until execution yields, returns, requests a host call,
    /// exhausts `budget`, or fails.
    pub const fn run(
        &mut self,
        program: &[ScriptOp<R>],
        budget: usize,
    ) -> Result<ScriptOutcome<R>, Error> {
        is! { self.ip > program.len(), return Err(Error::invalid_ip(self.ip, program.len())) }
        is! { self.ip == program.len(), return Ok(ScriptOutcome::Returned(self.return_value())) }
        whilst! { executed in 0..budget; {
            is! { let Some(outcome) = unwrap![ok? self.step(program)], return Ok(outcome) }
        }}
        Ok(ScriptOutcome::BudgetExhausted)
    }
}

/* hosted execution */
impl<R: Copy, const STACK: usize> ScriptMachine<R, STACK> {
    /// Executes one operation, resolving host calls through `host`.
    ///
    /// A failing operation is atomic with respect to machine state.
    /// Host-side effects are not rolled back.
    pub fn step_with<H: ScriptHost<R>>(
        &mut self,
        program: &[ScriptOp<R>],
        host: &mut H,
    ) -> Result<Option<ScriptOutcome<R>>, ScriptError<H::Error>> {
        let outcome = self.step_inner::<H::Error>(program)?;
        let call = unwrap![some_or? outcome, ScriptOutcome::HostCall(call) => call, Ok(outcome)];
        // `step_inner` just produced this token, so the machine still matches it.
        let arity = call.arity() as usize;
        let base = self.len - arity;
        let result =
            host.call(call.id(), &self.stack[base..self.len]).map_err(ScriptError::Host)?;
        self.complete_call_inner::<H::Error>(call, result)?;
        Ok(self.after_advance(program.len()))
    }
    /// Runs using `host` until execution yields, returns,
    /// exhausts `budget`, or fails.
    pub fn run_with<H: ScriptHost<R>>(
        &mut self,
        program: &[ScriptOp<R>],
        host: &mut H,
        budget: usize,
    ) -> Result<ScriptOutcome<R>, ScriptError<H::Error>> {
        is![self.ip > program.len(), return Err(ScriptError::invalid_ip(self.ip, program.len()))];
        is![self.ip == program.len(), return Ok(ScriptOutcome::Returned(self.return_value()))];
        whilst! { executed in 0..budget; {
            is![let Some(outcome) = unwrap![ok? self.step_with(program, host)], return Ok(outcome)];
        }}
        Ok(ScriptOutcome::BudgetExhausted)
    }
}

/* host calls */
impl<R: Copy, const STACK: usize> ScriptMachine<R, STACK> {
    /// Returns one argument of a pending host call.
    ///
    /// Arguments are indexed in call order, starting at zero.
    #[must_use]
    pub const fn call_arg(&self, call: ScriptCall, index: u8) -> Option<ScriptValue<R>> {
        is! { !self.call_matches(call) || index >= call.arity(), return None }
        let base = self.len - call.arity() as usize;
        Some(self.stack[base + index as usize])
    }
    /// Returns the arguments of a pending host call.
    pub fn call_args(&self, call: ScriptCall) -> Result<&[ScriptValue<R>], ScriptError> {
        is! { !self.call_matches(call), return Err(ScriptError::InvalidCall) }
        let arity = call.arity() as usize;
        is! { arity > self.len, return Err(ScriptError::InvalidCall) }
        let base = self.len - arity;
        Ok(&self.stack[base..self.len])
    }
    /// Completes a pending host call with its resulting value.
    ///
    /// The call arguments are replaced by `result`,
    /// and execution advances to the following operation.
    ///
    /// # Errors
    ///
    /// Returns [`ScriptError::InvalidCall`] if the machine
    /// no longer matches the state in which `call` was produced.
    pub const fn complete_call(
        &mut self,
        call: ScriptCall,
        result: ScriptValue<R>,
    ) -> Result<(), ScriptError> {
        self.complete_call_inner::<Infallible>(call, result)
    }

    /* helpers */

    const fn call_matches(&self, call: ScriptCall) -> bool {
        self.ip == call.ip && self.len == call.stack_len
    }
    const fn complete_call_inner<E>(
        &mut self,
        call: ScriptCall,
        result: ScriptValue<R>,
    ) -> Result<(), ScriptError<E>> {
        is! { !self.call_matches(call), return Err(ScriptError::InvalidCall) }
        let arity = call.arity() as usize;
        is! { arity > self.len, return Err(ScriptError::InvalidCall) }
        let base = self.len - arity;
        is! { base >= STACK, return Err(ScriptError::stack_overflow(STACK)) }
        self.stack[base] = result;
        self.len = base + 1;
        self.ip += 1;
        Ok(())
    }
}
