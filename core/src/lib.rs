pub mod logic;
pub mod named;
pub mod proof;

#[macro_export]
macro_rules! gdp {
    ($v:ty, named $n:ty where $p:ty) => { SuchThat<Named<$v, $n>, $p> };
}
