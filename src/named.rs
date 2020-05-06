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

/// Acts as a function over named values
trait NameFn {
    type Out;

    fn call<T0, N>(self, value0: Named<T0, N>) -> Self::Out;
}

/// Assigns a type-level name to a value
pub fn name<T0, F>(value0: T0, f: F) -> F::Out
    where F: NameFn,
{
    f.call(Named {
        value: value0,
        name: PhantomData::<()>,
    })
}
