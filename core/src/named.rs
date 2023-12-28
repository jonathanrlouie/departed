use std::marker::PhantomData;

/// A value with a type-level name
pub struct Named<A, N> {
    value: A,
    name: PhantomData<N>,
}

impl<A, N> Named<A, N> {
    /// Returns the underlying value
    pub fn unname(self) -> A {
        self.value
    }

    /// Returns a reference to the underlying value
    pub fn unname_ref(&self) -> &A {
        &self.value
    }
}

/// Gives a value a given name
///
/// This should only be used by library authors to export domain specific axioms.
/// Library authors should export only the types and not the constructors for
/// any defined names.
pub fn defn<A, N>(value: A, _name: N) -> Named<A, N> {
    Named {
        value,
        name: PhantomData,
    }
}

/// Acts as a function over named values
pub trait NameFn {
    type In;
    type Out;

    fn call<N>(value: Named<Self::In, N>) -> Self::Out;
}

/// Assigns a type-level name to a value
pub fn name<F>(value: <F as NameFn>::In, f: F) -> F::Out
where
    F: NameFn,
{
    F::call(Named {
        value,
        name: PhantomData::<()>,
    })
}

