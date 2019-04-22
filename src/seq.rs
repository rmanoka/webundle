pub struct Seq<A, B>(pub A, pub B);

#[macro_export]
macro_rules! seq {
    ($first:expr, $($rest: expr),* $(,)*) => {
        $crate::seq::Seq($first, seq!($($rest),+))
    };
    ($one:expr) => {
        $one
    };
}
