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
pub fn unsafe_name<A, N>(value: A) -> Named<A, N> {
    Named {
        value,
        name: PhantomData,
    }
}

/// Acts as a function over named values
pub trait NameFn {
    type In;
    type Out;

    fn call<N>(self, value0: Named<Self::In, N>) -> Self::Out;
}

/// Assigns a type-level name to a value
pub fn name<F>(value: <F as NameFn>::In, f: F) -> F::Out
where
    F: NameFn,
{
    f.call(Named {
        value,
        name: PhantomData::<()>,
    })
}

/// Acts as a function over named values
pub trait NameFn2 {
    type In1;
    type In2;
    type Out;

    fn call<N1, N2>(self, value1: Named<Self::In1, N1>, value2: Named<Self::In2, N2>) -> Self::Out;
}

/// Assigns a type-level name to a value
pub fn name2<F>(value1: <F as NameFn2>::In1, value2: <F as NameFn2>::In2, f: F) -> F::Out
where
    F: NameFn2,
{
    f.call(
        Named {
            value: value1,
            name: PhantomData::<()>,
        },
        Named {
            value: value2,
            name: PhantomData::<()>,
        },
    )
}
