pub use departed_core::*;
pub use departed_macros::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        name!(1, for<A> |a: Named<u32, A>| -> () { 
            name!(2, for<B> |b: Named<u32, B>| -> () {
                
            })
        })
    }
}
