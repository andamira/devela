use super::*;
use crate::{Value64, ValueKind, ValueKind4, is, unwrap};

type V = Value64;
type M<const STACK: usize> = ScriptMachine<STACK>;

const fn int(value: i64) -> V {
    match V::try_from_int(value) {
        Some(value) => value,
        None => panic!("integer does not fit the compact script word"),
    }
}

/* direct execution */

#[test]
const fn direct_execution_is_const() {
    let program = [ScriptOp::Push(int(20)), ScriptOp::Push(int(22)), ScriptOp::Add];
    let mut machine = M::<2>::new();
    let outcome = unwrap![ok_expect machine.run(&program, 3), "unexpected script error"];
    match outcome {
        ScriptOutcome::Returned(Some(value)) => match value.as_int() {
            Some(42) => {}
            _ => panic!("unexpected returned value"),
        },
        _ => panic!("unexpected outcome"),
    }
    assert!(machine.ip() == 3);
    assert!(machine.stack_len() == 1);
    match machine.peek() {
        Some(value) => match value.as_int() {
            Some(42) => {}
            _ => panic!("unexpected stack value"),
        },
        None => panic!("missing result"),
    }
}
#[test]
fn yield_is_resumable() {
    let program = [
        ScriptOp::Push(int(7)),
        ScriptOp::Yield,
        ScriptOp::Push(int(5)),
        ScriptOp::Add,
        ScriptOp::Return,
    ];
    let mut machine = M::<2>::new();
    assert_eq!(machine.run(&program, 16), Ok(ScriptOutcome::Yielded));
    assert_eq!(machine.ip(), 2);
    assert_eq!(machine.stack(), &[int(7)]);
    assert_eq!(machine.run(&program, 16), Ok(ScriptOutcome::Returned(Some(int(12)))));
}
#[test]
fn budget_exhaustion_is_resumable() {
    let program = [ScriptOp::Jump(0)];
    let mut machine = M::<0>::new();
    assert_eq!(machine.run(&program, 8), Ok(ScriptOutcome::BudgetExhausted));
    assert_eq!(machine.ip(), 0);
    assert_eq!(machine.run(&program, 8), Ok(ScriptOutcome::BudgetExhausted));
    assert_eq!(machine.ip(), 0);
}
#[test]
fn jumps_can_move_backward() {
    let program = [ScriptOp::Jump(1), ScriptOp::Jump(-1)];
    let mut machine = M::<0>::new();
    assert_eq!(machine.run(&program, 2), Ok(ScriptOutcome::BudgetExhausted));
    assert_eq!(machine.ip(), 0);
}
#[test]
fn failed_operation_is_atomic() {
    let program = [ScriptOp::Push(V::TRUE), ScriptOp::Push(int(1)), ScriptOp::Add];
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
    let program = [ScriptOp::Push(int(V::INT_MAX)), ScriptOp::Push(int(1)), ScriptOp::Add];
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
        ScriptOp::Push(int(20)),
        ScriptOp::Push(int(22)),
        ScriptOp::CallHost { id: ADD, arity: 2 },
        ScriptOp::Return,
    ];
    let mut machine = M::<2>::new();
    let call = unwrap![ok_expect machine.run(&program, 3),
        ScriptOutcome::HostCall(call) => call, "expected host call"];
    assert!(call.id().raw() == 7);
    assert!(call.arity() == 2);
    assert!(call.ip() == 2);
    match machine.call_arg(call, 0) {
        Some(value) => match value.as_int() {
            Some(20) => {}
            _ => panic!("unexpected first argument"),
        },
        None => panic!("missing first argument"),
    }
    match machine.call_arg(call, 1) {
        Some(value) => match value.as_int() {
            Some(22) => {}
            _ => panic!("unexpected second argument"),
        },
        None => panic!("missing second argument"),
    }
    unwrap![ok_expect machine.complete_call(call, int(42)), "could not complete call"];
    let outcome = unwrap![ok_expect machine.run(&program, 1), "unexpected script error"];
    match outcome {
        ScriptOutcome::Returned(Some(value)) => match value.as_int() {
            Some(42) => {}
            _ => panic!("unexpected result"),
        },
        _ => panic!("unexpected outcome"),
    }
}
#[test]
fn hosted_execution_resolves_calls() {
    const ADD: ScriptCallId = unwrap![some_expect ScriptCallId::new(7), "invalid call id"];
    let program = [
        ScriptOp::Push(int(20)),
        ScriptOp::Push(int(22)),
        ScriptOp::CallHost { id: ADD, arity: 2 },
        ScriptOp::Return,
    ];
    let mut calls = 0;
    let mut host = |id: ScriptCallId, args: &[V]| -> Result<V, &'static str> {
        calls += 1;
        is! { id != ADD, return Err("unknown call") }
        is! { args != [int(20), int(22)], return Err("invalid arguments") }
        Ok(int(42))
    };
    let mut machine = M::<2>::new();
    assert_eq!(
        machine.run_with(&program, &mut host, 4),
        Ok(ScriptOutcome::Returned(Some(int(42))))
    );
    assert_eq!(calls, 1);
}
#[test]
fn host_error_preserves_machine_state() {
    const FAIL: ScriptCallId = unwrap![some_expect ScriptCallId::new(3), "invalid call id"];
    let program = [ScriptOp::Push(int(5)), ScriptOp::CallHost { id: FAIL, arity: 1 }];
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
        machine.step(&[ScriptOp::Drop]),
        Err(ScriptError::StackUnderflow { needed: 1, available: 0 })
    );
    assert!(machine.push(int(1)));
    assert_eq!(
        machine.step(&[ScriptOp::Push(int(2))]),
        Err(ScriptError::StackOverflow { capacity: 1 })
    );
    machine.reset();
    let before = machine.clone();
    assert_eq!(
        machine.step(&[ScriptOp::Jump(-1)]),
        Err(ScriptError::InvalidJump { ip: 0, offset: -1, len: 1 })
    );
    assert_eq!(machine, before);
}

#[test]
const fn ref_words_can_be_compared_without_resolving() {
    let reference = unwrap![some_expect V::try_from_parts(ValueKind4::Ref, 0x1234), "invalid ref"];
    let program = [ScriptOp::Push(reference), ScriptOp::Dup, ScriptOp::Eq];
    let mut machine = M::<2>::new();
    let result = unwrap![ok_expect machine.run(&program, 3), "script failed"];
    match result {
        ScriptOutcome::Returned(Some(value)) => match value.as_bool() {
            Some(true) => {}
            _ => panic!("identical ref tokens should compare equal"),
        },
        _ => panic!("expected a boolean result"),
    }
}
#[test]
fn different_kinds_are_not_equal() {
    let reference = V::try_from_parts(ValueKind4::Ref, 9).unwrap();
    let program = [ScriptOp::Push(reference), ScriptOp::Push(int(9)), ScriptOp::Eq];
    let mut machine = M::<2>::new();
    assert_eq!(machine.run(&program, 3), Ok(ScriptOutcome::Returned(Some(V::FALSE))));
}
