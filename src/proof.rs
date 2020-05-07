use crate::named::Named;
use std::marker::PhantomData;

/// A proof that can be passed to a function to prove that a predicate has been satisfied.
pub struct Proof<P>(pub(crate) PhantomData<P>);

/// A value with a proof attached
pub struct SuchThat<A, P> {
    value: A,
    _pd: PhantomData<P>,
}

impl<A, P> SuchThat<A, P> {
    /// Extracts the value from a proof-attached value
    pub fn value(self) -> A {
        self.value
    }

    /// Returns a reference to the value of a proof-attached value
    pub fn value_ref(&self) -> &A {
        &self.value
    }
}

impl<A, N, P> SuchThat<Named<A, N>, P> {
    /// Extract the value from a proof-attached named value
    pub fn extract(self) -> A {
        self.value().unname()
    }

    /// Returns a reference to the value of a proof-attached named value
    pub fn extract_ref(&self) -> &A {
        self.value_ref().unname_ref()
    }
}

/// Attaches a proof to a value
pub fn such_that<A, P>(value: A, _proof: &Proof<P>) -> SuchThat<A, P> {
    SuchThat {
        value,
        _pd: PhantomData,
    }
}
