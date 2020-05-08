use departed::{
    logic::{equals, refl, Equals},
    named::{name, NameFn, Named},
    proof::{such_that, SuchThat},
};
use std::fmt;
use std::marker::PhantomData;
use text_io::read;

// Type-level numbers; could use typenum crate for this instead
struct Z;
struct S<N>(PhantomData<N>);

struct Length<Xs>(PhantomData<Xs>);

// A vector parameterized by its length
// This is similar to what you can do with const generics, but you can create a LenVec from a vector with a length only known at runtime
struct LenVec<T, N> {
    items: Vec<T>,
    _len: PhantomData<N>
}

impl<T: fmt::Debug, N> fmt::Debug for LenVec<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LenVec")
            .field("items", &self.items)
            .finish()
    }
}

impl<T, N> LenVec<T, N> {
    fn new() -> LenVec<T, Z> {
        LenVec {
            items: vec![],
            _len: PhantomData
        }
    }

    fn from_vec<Xs>(xs: Named<Vec<T>, Xs>) -> LenVec<T, Length<Xs>> {
        LenVec {
            items: xs.unname(),
            _len: PhantomData
        }
    }

    fn push(mut self, item: T) -> LenVec<T, S<N>> {
        self.items.push(item);
        LenVec {
            items: self.items,
            _len: PhantomData
        }
    }
}

impl<T, N> LenVec<T, S<N>> {
    fn pop(mut self) -> (LenVec<T, N>, T) {
        let item = self.items.pop().unwrap();
        (LenVec {
            items: self.items,
            _len: PhantomData
        }, item)
    }
}

fn swap_first_two_items<N>(xs: LenVec<u32, S<S<N>>>) -> LenVec<u32, S<S<N>>> {
    let (xs, item1) = xs.pop();
    let (xs, item2) = xs.pop();
    xs.push(item1).push(item2)
}

fn build_vec_from_input() -> Vec<u32> {
    let mut xs = vec![];
    println!("Enter a positive number to add it to the vector (0 to quit):");
    let mut value = read!();
    while value > 0 {
        xs.push(value);
        println!("Enter a positive number to add it to the vector (0 to quit):");
        value = read!();
    }
    xs
}

struct ProcessVecFn;

impl NameFn for ProcessVecFn {
    type In = Vec<u32>;
    type Out = ();

    fn call<Xs>(self, xs: Named<Vec<u32>, Xs>) {
        let xs = LenVec::<_, Length<Xs>>::from_vec(xs);
        let xs = xs.push(2);
        println!("Pushed 2");
        let (xs, popped) = xs.pop();
        println!("Popped value: {}", popped);
        //let (xs, popped) = xs.pop();    doesn't compile, since we don't know if the input vector is empty or not

        let xs = xs.push(3);
        let xs = xs.push(5);
        println!("Before swap: {:?}", xs);
        println!("After swap: {:?}", swap_first_two_items(xs)); // this wouldn't compile if two items hadn't been pushed on to the vector
    }
}

fn main() {
    let vec = build_vec_from_input();
    name(vec, ProcessVecFn);
}
