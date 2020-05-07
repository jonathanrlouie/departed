use departed::{
    logic::{equals, refl, Equals},
    named::{name2, unsafe_name, NameFn2, Named},
    proof::{such_that, SuchThat},
};
use std::marker::PhantomData;

struct Length<Xs>(PhantomData<Xs>);

// should be a library function, since it creates and returns a new named value and needs to call unsafe_name
fn length<T, Xs>(xs: &Named<Vec<T>, Xs>) -> Named<usize, Length<Xs>> {
    unsafe_name(xs.unname_ref().len())
}

// This zip function will cause compilation errors if xs and ys are not proven to have the same length
fn zip<Xs, Ys, N>(
    xs: SuchThat<Named<Vec<u32>, Xs>, Equals<Length<Xs>, N>>,
    ys: SuchThat<Named<Vec<u32>, Ys>, Equals<Length<Ys>, N>>,
) -> Vec<(u32, u32)> {
    xs.extract()
        .into_iter()
        .zip(ys.extract().into_iter())
        .collect()
}

struct ZipFn;

impl NameFn2 for ZipFn {
    type In1 = Vec<u32>;
    type In2 = Vec<u32>;
    type Out = ();

    fn call<Xs, Ys>(self, xs: Named<Vec<u32>, Xs>, ys: Named<Vec<u32>, Ys>) -> Self::Out {
        let xs_len = length(&xs);
        let ys_len = length(&ys);

        match equals(xs_len, ys_len) {
            Some(eq_prf) => {
                let xs_with_prf = such_that(xs, &eq_prf);

                // use refl() to prove that Equals<Length<Ys>, Length<Ys>>
                let ys_with_prf = such_that(ys, &refl());
                println!("{:?}", zip(xs_with_prf, ys_with_prf));
            }
            None => eprintln!("lengths are not the same!"),
        }
    }
}

fn main() {
    name2(vec![1, 2, 3], vec![4, 5, 6], ZipFn);
}
