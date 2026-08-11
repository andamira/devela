// devela/src/lang/prog/script/machine/_test.rs

use super::*;
use crate::{ValueKind, is, unwrap};

type V = ScriptValue<u8>;
type M<const STACK: usize> = ScriptMachine<u8, STACK>;

/* direct execution */

#[test]
const fn direct_execution_is_const() {
    let program = [ScriptOp::Push(V::Int(20)), ScriptOp::Push(V::Int(22)), ScriptOp::Add];
    let mut machine = M::<2>::new();
    let outcome = unwrap![ok_expect machine.run(&program, 3), "unexpected script error"];
    match outcome {
        ScriptOutcome::Returned(Some(V::Int(42))) => {}
        _ => panic!("unexpected outcome"),
    }
    assert!(machine.ip() == 3);
    assert!(machine.stack_len() == 1);
    unwrap![some_expect machine.peek(), V::Int(42) => {}, "unexpected stack"];
}
#[test]
fn yield_is_resumable() {
    let program = [
        ScriptOp::Push(V::Int(7)),
        ScriptOp::Yield,
        ScriptOp::Push(V::Int(5)),
        ScriptOp::Add,
        ScriptOp::Return,
    ];
    let mut machine = M::<2>::new();
    assert_eq!(machine.run(&program, 16), Ok(ScriptOutcome::Yielded));
    assert_eq!(machine.ip(), 2);
    assert_eq!(machine.stack(), &[V::Int(7)]);
    assert_eq!(machine.run(&program, 16), Ok(ScriptOutcome::Returned(Some(V::Int(12)))));
}
#[test]
fn budget_exhaustion_is_resumable() {
    let program = [ScriptOp::<u8>::Jump(0)];
    let mut machine = M::<0>::new();
    assert_eq!(machine.run(&program, 8), Ok(ScriptOutcome::BudgetExhausted));
    assert_eq!(machine.ip(), 0);
    assert_eq!(machine.run(&program, 8), Ok(ScriptOutcome::BudgetExhausted));
    assert_eq!(machine.ip(), 0);
}
#[test]
fn jumps_can_move_backward() {
    let program = [ScriptOp::<u8>::Jump(1), ScriptOp::Jump(-1)];
    let mut machine = M::<0>::new();
    assert_eq!(machine.run(&program, 2), Ok(ScriptOutcome::BudgetExhausted));
    assert_eq!(machine.ip(), 0);
}
#[test]
fn failed_operation_is_atomic() {
    let program = [ScriptOp::Push(V::Bool(true)), ScriptOp::Push(V::Int(1)), ScriptOp::Add];
    let mut machine = M::<2>::new();
    assert_eq!(machine.step(&program), Ok(None));
    assert_eq!(machine.step(&program), Ok(None));
    let before = machine.clone();
    assert_eq!(
        machine.step(&program),
        Err(ScriptError::ExpectedKind { expected: ValueKind::Int, found: ValueKind::Bool })
    );
    assert_eq!(machine, before);
}
#[test]
fn integer_overflow_is_atomic() {
    let program = [ScriptOp::Push(V::Int(i64::MAX)), ScriptOp::Push(V::Int(1)), ScriptOp::Add];
    let mut machine = M::<2>::new();
    assert_eq!(machine.run(&program, 2), Ok(ScriptOutcome::BudgetExhausted));
    let before = machine.clone();
    assert_eq!(machine.step(&program), Err(ScriptError::IntegerOverflow));
    assert_eq!(machine, before);
}
#[test]
const fn host_call_can_be_resolved_directly_in_const() {
    const ADD: ScriptCallId = unwrap![some_expect ScriptCallId::new(7), "invalid call id"];
    let program = [
        ScriptOp::Push(V::Int(20)),
        ScriptOp::Push(V::Int(22)),
        ScriptOp::CallHost { id: ADD, arity: 2 },
        ScriptOp::Return,
    ];
    let mut machine = M::<2>::new();
    let call = unwrap![ok_expect machine.run(&program, 3),
        ScriptOutcome::HostCall(call) => call, "expected host call"];
    assert!(call.id().raw() == 7);
    assert!(call.arity() == 2);
    assert!(call.ip() == 2);
    unwrap![some_expect machine.call_arg(call, 0), V::Int(20) => {}, "unexpected first argument"];
    unwrap![some_expect machine.call_arg(call, 1), V::Int(22) => {}, "unexpected second argument"];
    unwrap![ok_expect machine.complete_call(call, V::Int(42)), "could not complete call"];
    let outcome = unwrap![ok_expect machine.run(&program, 1), "unexpected script error"];
    match outcome {
        ScriptOutcome::Returned(Some(V::Int(42))) => {}
        _ => panic!("unexpected result"),
    }
}
#[test]
fn hosted_execution_resolves_calls() {
    const ADD: ScriptCallId = unwrap![some_expect ScriptCallId::new(7), "invalid call id"];
    let program = [
        ScriptOp::Push(V::Int(20)),
        ScriptOp::Push(V::Int(22)),
        ScriptOp::CallHost { id: ADD, arity: 2 },
        ScriptOp::Return,
    ];
    let mut calls = 0;
    let mut host = |id: ScriptCallId, args: &[V]| -> Result<V, &'static str> {
        calls += 1;
        is! { id != ADD, return Err("unknown call") }
        is! { args != [V::Int(20), V::Int(22)], return Err("invalid arguments") }
        Ok(V::Int(42))
    };
    let mut machine = M::<2>::new();
    assert_eq!(
        machine.run_with(&program, &mut host, 4),
        Ok(ScriptOutcome::Returned(Some(V::Int(42))))
    );
    assert_eq!(calls, 1);
}
#[test]
fn host_error_preserves_machine_state() {
    const FAIL: ScriptCallId = unwrap![some_expect ScriptCallId::new(3), "invalid call id"];
    let program = [ScriptOp::Push(V::Int(5)), ScriptOp::CallHost { id: FAIL, arity: 1 }];
    let mut machine = M::<1>::new();
    assert_eq!(machine.run(&program, 1), Ok(ScriptOutcome::BudgetExhausted));
    let before = machine.clone();
    let mut host = |_id: ScriptCallId, _args: &[V]| -> Result<V, &'static str> { Err("rejected") };
    assert_eq!(machine.run_with(&program, &mut host, 1), Err(ScriptError::Host("rejected")));
    assert_eq!(machine, before);
}
#[test]
fn execution_reports_stack_and_jump_errors() {
    let mut machine = M::<1>::new();
    assert_eq!(
        machine.step(&[ScriptOp::<u8>::Drop]),
        Err(ScriptError::StackUnderflow { needed: 1, available: 0 })
    );
    assert!(machine.push(V::Int(1)));
    assert_eq!(
        machine.step(&[ScriptOp::Push(V::Int(2))]),
        Err(ScriptError::StackOverflow { capacity: 1 })
    );
    machine.reset();
    let before = machine.clone();
    assert_eq!(
        machine.step(&[ScriptOp::<u8>::Jump(-1)]),
        Err(ScriptError::InvalidJump { ip: 0, offset: -1, len: 1 })
    );
    assert_eq!(machine, before);
}
