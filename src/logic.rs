use crate::proof::Proof;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct True(());

#[derive(Copy, Clone)]
pub struct False(());

#[derive(Copy, Clone)]
pub struct And<P, Q>(PhantomData<(P, Q)>);

#[derive(Copy, Clone)]
pub struct Or<P, Q>(PhantomData<(P, Q)>);

#[derive(Copy, Clone)]
pub struct Implies<P, Q>(PhantomData<(P, Q)>);

#[derive(Copy, Clone)]
pub struct Not<P>(PhantomData<P>);

#[derive(Copy, Clone)]
pub struct Equals<P, Q>(PhantomData<(P, Q)>);

pub fn ptrue() -> Proof<True> {
    Proof(PhantomData)
}

pub fn noncontra<P: Copy + Clone>() -> Proof<Not<And<P, Not<P>>>> {
    Proof(PhantomData)
}

pub fn and_intro<P, Q>(_p: Proof<P>, _q: Proof<Q>) -> Proof<And<P, Q>> {
    Proof(PhantomData)
}

pub fn and_elim_l<P, Q>(_and: Proof<And<P, Q>>) -> Proof<P> {
    Proof(PhantomData)
}

pub fn and_elim_r<P, Q>(_and: Proof<And<P, Q>>) -> Proof<Q> {
    Proof(PhantomData)
}

pub fn or_intro_l<P, Q>(_p: Proof<P>) -> Proof<Or<P, Q>> {
    Proof(PhantomData)
}

pub fn or_intro_r<P, Q>(_q: Proof<Q>) -> Proof<Or<P, Q>> {
    Proof(PhantomData)
}

pub fn elim_or<P, Q, R>(
    _p_to_r: impl Fn(Proof<P>) -> Proof<R>,
    _q_to_r: impl Fn(Proof<Q>) -> Proof<R>,
    _or: &Proof<Or<P, Q>>,
) -> Proof<R> {
    Proof(PhantomData)
}

pub fn implies_intro<P, Q>(_f: impl Fn(Proof<P>) -> Proof<Q>) -> Proof<Implies<P, Q>> {
    Proof(PhantomData)
}

pub fn implies_elim<P, Q>(_implies: Proof<Implies<P, Q>>, _p: Proof<P>) -> Proof<Q> {
    Proof(PhantomData)
}

pub fn not_intro<P>(_f: impl Fn(Proof<P>) -> Proof<False>) -> Proof<Not<P>> {
    Proof(PhantomData)
}

pub fn contradicts<P>(_p: Proof<P>, _not_p: Proof<Not<P>>) -> Proof<False> {
    Proof(PhantomData)
}

pub fn absurd<P>(_false: Proof<False>) -> Proof<P> {
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

/*
Example proof
*/
fn foo<P, Q>() -> Proof<Implies<Implies<P, Q>, Implies<Not<Q>, Not<P>>>>
where
    P: Copy + Clone,
    Q: Copy + Clone,
{
    implies_intro(|p2q| {
        implies_intro(|notq| not_intro(|p| contradicts(implies_elim(p2q, p), notq)))
    })
}
