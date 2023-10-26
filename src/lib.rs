pub use departed_macros::*;
pub use departed_core::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        #[names]
        fn blah() {}
        blah();
    }
}
