use crate::{
    named::Named,
    proof::Proof
};
use std::marker::PhantomData;

pub enum True{}

pub enum False{}

pub struct And<P, Q>(PhantomData<(P, Q)>);

pub struct Or<P, Q>(PhantomData<(P, Q)>);

pub struct Implies<P, Q>(PhantomData<(P, Q)>);

pub struct Not<P>(PhantomData<P>);

pub struct Equals<P, Q>(PhantomData<(P, Q)>);

pub fn and_intro<P, Q>(_p: &Proof<P>, _q: &Proof<Q>) -> Proof<And<P, Q>> {
    Proof(PhantomData)
}

pub fn and_elim_l<P, Q>(_and: &Proof<And<P, Q>>) -> Proof<P> {
    Proof(PhantomData)
}

pub fn and_elim_r<P, Q>(_and: &Proof<And<P, Q>>) -> Proof<Q> {
    Proof(PhantomData)
}

pub fn or_intro_l<P, Q>(_p: &Proof<P>) -> Proof<Or<P, Q>> {
    Proof(PhantomData)
}

pub fn or_intro_r<P, Q>(_q: &Proof<Q>) -> Proof<Or<P, Q>> {
    Proof(PhantomData)
}

pub fn elim_or<P, Q, R>(
    _p_to_r: fn(Proof<P>) -> Proof<R>,
    _q_to_r: fn(Proof<Q>) -> Proof<R>,
    _or: &Proof<Or<P, Q>>,
) -> Proof<R> {
    Proof(PhantomData)
}

pub fn implies_intro<P, Q>(_f: fn(Proof<P>) -> Proof<Q>) -> Proof<Implies<P, Q>> {
    Proof(PhantomData)
}

pub fn implies_elim<P, Q>(_implies: &Proof<Implies<P, Q>>, _p: &Proof<P>) -> Proof<Q> {
    Proof(PhantomData)
}

pub fn not_intro<P>(_f: fn(Proof<P>) -> Proof<False>) -> Proof<Not<P>> {
    Proof(PhantomData)
}

pub fn contradicts<P>(_p: &Proof<P>, _not_p: &Proof<Not<P>>) -> Proof<False> {
    Proof(PhantomData)
}

pub fn absurd<P>(_false: &Proof<False>) -> Proof<P> {
    Proof(PhantomData)
}

pub fn refl<X>() -> Proof<Equals<X, X>> {
    Proof(PhantomData)
}

/// Creates a proof of a given type.
///
/// Library users should NOT use this.
pub fn axiom<P>() -> Proof<P> {
    Proof(PhantomData)
}


pub fn equals<T: Eq, A, B>(a: Named<T, A>, b: Named<T, B>) -> Option<Proof<Equals<A, B>>> {
    if a.unname() == b.unname() {
        Some(axiom())
    } else {
        None
    }
}