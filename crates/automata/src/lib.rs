//! This library provides an implementation of Nondeterministic Finite Automata
//! (NFA) and Deterministic Finite Automata (DFA) for Unicode scalar values
//! (the [`char`] type). It is used by the [`ere`] crate to represent compiled
//! regular expressions.
//!
//! [`ere`]: <https://github.com/timothee-haudebourg/ere-rs>
pub use btree_range_map::{AnyRange, RangeSet};

pub mod nfa;
pub use nfa::NFA;

pub mod dfa;
pub use dfa::DFA;

pub fn any_char() -> RangeSet<char> {
	let mut set = RangeSet::new();
	set.insert('\u{0}'..='\u{d7ff}');
	set.insert('\u{e000}'..='\u{10ffff}');
	set
}

/// Computes the intersection of two character sets.
pub fn charset_intersection(a: &RangeSet<char>, b: &RangeSet<char>) -> RangeSet<char> {
	let mut result = a.clone();

	for r in b.gaps() {
		result.remove(r.cloned());
	}

	result
}

/// Deterministic or non-deterministic automaton.
pub trait Automaton<T> {
	type State<'a>
	where
		Self: 'a;

	fn initial_state(&self) -> Option<Self::State<'_>>;

	fn next_state<'a>(
		&'a self,
		current_state: Self::State<'a>,
		token: T,
	) -> Option<Self::State<'_>>;

	fn is_final_state<'a>(&'a self, state: &Self::State<'a>) -> bool;
}
