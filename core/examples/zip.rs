mod safe_zip {
    use departed::{
        logic::{Equals, axiom},
        named::{defn, Named},
        proof::{Pred, Proof},
    };
    use std::marker::PhantomData;

    pub struct Length<Xs>(PhantomData<Xs>);

    pub fn length<T, Xs>(xs: &Named<Vec<T>, Xs>) -> Named<usize, Length<Xs>> {
        defn(xs.unname_ref().len(), Length(PhantomData))
    }

    // This zip function will cause compilation errors if xs and ys are not proven to have the same length
    pub fn zip<Xs, Ys, N>(
        xs: Pred<Vec<u32>, Xs, Equals<Length<Xs>, N>>,
        ys: Pred<Vec<u32>, Ys, Equals<Length<Ys>, N>>,
    ) -> Vec<(u32, u32)> {
        xs.extract()
            .into_iter()
            .zip(ys.extract().into_iter())
            .collect()
    }

    pub fn equals<T: Eq, A, B>(a: Named<T, A>, b: Named<T, B>) -> Option<Proof<Equals<A, B>>> {
        if a.unname() == b.unname() {
            Some(axiom())
        } else {
            None
        }
    }
}

use departed::{
    logic::refl,
    named::{name2, NameFn2, Named},
    proof::such_that,
};

// User defined length function should not be allowed to be defined.
// This is prevented by defn requiring a safe_zip::Length to be constructable, which
// only the library can do.
/*
use departed::named::defn;
use std::marker::PhantomData;

fn bad_length<T, Xs>(xs: &Named<Vec<T>, Xs>) -> Named<usize, safe_zip::Length<Xs>> {
    defn(xs.unname_ref().len(), safe_zip::Length(PhantomData))
}
*/

struct ZipFn;

impl NameFn2 for ZipFn {
    type In1 = Vec<u32>;
    type In2 = Vec<u32>;
    type Out = ();

    fn call<Xs, Ys>(self, xs: Named<Vec<u32>, Xs>, ys: Named<Vec<u32>, Ys>) -> Self::Out {
        let xs_len = safe_zip::length(&xs);
        let ys_len = safe_zip::length(&ys);

        match safe_zip::equals(xs_len, ys_len) {
            Some(eq_prf) => {
                let xs_with_prf = such_that(xs, &eq_prf);

                // use refl() to prove that Equals<Length<Ys>, Length<Ys>>
                let ys_with_prf = such_that(ys, &refl());
                println!("{:?}", safe_zip::zip(xs_with_prf, ys_with_prf));
            }
            None => eprintln!("lengths are not the same!"),
        }
    }
}

fn main() {
    name2(vec![1, 2, 3], vec![4, 5, 6], ZipFn);
}
