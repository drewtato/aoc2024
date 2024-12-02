// pub trait BetterSumIter<A> {
//     type Output;
//     fn bsum(self) -> Self::Output;
// }

// macro_rules! impl_better_sum {
//     ($()) => {

//     };
// }
// impl<I, A> BetterSumIter<A> for I
// {
//     type Output = A;

//     fn bsum(self) -> Self::Output {
//         self.fold(A::zero(), |acc, item| acc + item)
//     }
// }
