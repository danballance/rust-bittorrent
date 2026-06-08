use crate::bencode::character_processor::CharacterProcessor;
use crate::bencode::types::{BencodeKind, BencodeState, Context};

/*
 * These tests confirm that for each input character we generate the correct
 * Kind and State, such as i => Integer Start. The code relies on context.
 * However, in these tests we do not consider the context or the resulting values.
 */

type ExpectedPhase = (
    char,
    Option<BencodeKind>,
    Option<BencodeState>,
    Vec<BencodeKind>,
);

#[test]
fn test_integer() {
    let input = "i12e";
    let expected_phases: Vec<ExpectedPhase> = vec![
        (
            'i',
            Some(BencodeKind::Integer),
            Some(BencodeState::Start),
            vec![],
        ),
        (
            '1',
            Some(BencodeKind::Integer),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            '2',
            Some(BencodeKind::Integer),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'e',
            Some(BencodeKind::Integer),
            Some(BencodeState::End),
            vec![],
        ),
    ];
    _run_and_assert_iteration(input, expected_phases);
}

#[test]
fn test_simple_string() {
    let input = "5:hello";
    let expected_phases: Vec<ExpectedPhase> = vec![
        (
            '5',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![],
        ),
        (
            ':',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![],
        ),
        (
            'h',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'e',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'l',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'l',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'o',
            Some(BencodeKind::String),
            Some(BencodeState::End),
            vec![],
        ),
    ];
    _run_and_assert_iteration(input, expected_phases);
}

#[test]
fn test_empty_string() {
    let input = "0:";
    let expected_phases: Vec<ExpectedPhase> = vec![
        (
            '0',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![],
        ),
        (
            ':',
            Some(BencodeKind::String),
            Some(BencodeState::End),
            vec![],
        ),
    ];
    _run_and_assert_iteration(input, expected_phases);
}

#[test]
fn test_simple_list() {
    let input = "li52ee";
    let expected_phases: Vec<ExpectedPhase> = vec![
        (
            'l',
            Some(BencodeKind::List),
            Some(BencodeState::Start),
            vec![BencodeKind::List],
        ),
        (
            'i',
            Some(BencodeKind::Integer),
            Some(BencodeState::Start),
            vec![BencodeKind::List],
        ),
        (
            '5',
            Some(BencodeKind::Integer),
            Some(BencodeState::Data),
            vec![BencodeKind::List],
        ),
        (
            '2',
            Some(BencodeKind::Integer),
            Some(BencodeState::Data),
            vec![BencodeKind::List],
        ),
        (
            'e',
            Some(BencodeKind::Integer),
            Some(BencodeState::End),
            vec![BencodeKind::List],
        ),
        (
            'e',
            Some(BencodeKind::List),
            Some(BencodeState::End),
            vec![],
        ),
    ];
    _run_and_assert_iteration(input, expected_phases);
}

#[test]
fn test_double_var_list() {
    let input = "l5:helloi52ee";
    let expected_phases: Vec<ExpectedPhase> = vec![
        (
            'l',
            Some(BencodeKind::List),
            Some(BencodeState::Start),
            vec![BencodeKind::List],
        ),
        (
            '5',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![BencodeKind::List],
        ),
        (
            ':',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![BencodeKind::List],
        ),
        (
            'h',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'e',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'l',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'l',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![],
        ),
        (
            'o',
            Some(BencodeKind::String),
            Some(BencodeState::End),
            vec![],
        ),
        (
            'i',
            Some(BencodeKind::Integer),
            Some(BencodeState::Start),
            vec![BencodeKind::List],
        ),
        (
            '5',
            Some(BencodeKind::Integer),
            Some(BencodeState::Data),
            vec![BencodeKind::List],
        ),
        (
            '2',
            Some(BencodeKind::Integer),
            Some(BencodeState::Data),
            vec![BencodeKind::List],
        ),
        (
            'e',
            Some(BencodeKind::Integer),
            Some(BencodeState::End),
            vec![BencodeKind::List],
        ),
        (
            'e',
            Some(BencodeKind::List),
            Some(BencodeState::End),
            vec![],
        ),
    ];
    _run_and_assert_iteration(input, expected_phases);
}

#[test]
fn test_dictionary() {
    let input = "d3:foo3:bare";
    let expected_phases: Vec<ExpectedPhase> = vec![
        (
            'd',
            Some(BencodeKind::Dictionary),
            Some(BencodeState::Start),
            vec![BencodeKind::Dictionary],
        ),
        (
            '3',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![BencodeKind::Dictionary],
        ),
        (
            ':',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![BencodeKind::Dictionary],
        ),
        (
            'f',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![BencodeKind::Dictionary],
        ),
        (
            'o',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![BencodeKind::Dictionary],
        ),
        (
            'o',
            Some(BencodeKind::String),
            Some(BencodeState::End),
            vec![BencodeKind::Dictionary],
        ),
        (
            '3',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![BencodeKind::Dictionary],
        ),
        (
            ':',
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
            vec![BencodeKind::Dictionary],
        ),
        (
            'b',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![BencodeKind::Dictionary],
        ),
        (
            'a',
            Some(BencodeKind::String),
            Some(BencodeState::Data),
            vec![BencodeKind::Dictionary],
        ),
        (
            'r',
            Some(BencodeKind::String),
            Some(BencodeState::End),
            vec![BencodeKind::Dictionary],
        ),
        (
            'e',
            Some(BencodeKind::Dictionary),
            Some(BencodeState::End),
            vec![],
        ),
    ];
    _run_and_assert_iteration(input, expected_phases);
}
fn _run_and_assert_iteration(input: &str, mut expected_phases: Vec<ExpectedPhase>) -> () {
    let processor = CharacterProcessor::new();
    let mut ctxt = Context::new();
    for c in input.chars() {
        let expected_phase = expected_phases.remove(0);
        ctxt = processor
            .next_character(c, ctxt)
            .expect("CharacterProcessor failed to produce a next phase");
        assert_eq!(
            ctxt.character,
            Some(expected_phase.0),
            "Incorrect character {:?}",
            ctxt.character
        );
        assert_eq!(
            ctxt.kind, expected_phase.1,
            "Incorrect Kind {:?}",
            ctxt.kind
        );
        assert_eq!(
            ctxt.state, expected_phase.2,
            "Incorrect state {:?}",
            ctxt.state
        );
    }
}
