mod root;

fn main() {
    root::main();
}

// trait ProcessInto<P, R> {
//     fn process(self, requirements: R) -> Result<P, WErr>;
// }
//
// enum MaybeProcessed<U, P, R> where U: ProcessInto<P, R> {
//     Unprocessed(U, PhantomData<R>),
//     Processed(P)
// }
//
// impl<U, P, R> MaybeProcessed<U, P, R> where U: ProcessInto<P, R> {
//     pub fn get_processed(self, requirements: R) -> Result<P, WErr> {
//         match self {
//             MaybeProcessed::Unprocessed(u, _) => {
//                 u.process(requirements)
//             }
//             MaybeProcessed::Processed(p) => p
//         }
//     }
//
//     pub fn get_processed_inplace(&mut self, requirements: R) -> Result<&mut P, WErr> {
//         match self {
//             MaybeProcessed::Unprocessed(u, _) => {
//                 *self = MaybeProcessed::Processed(
//                     u.
//                 );
//
//                 ()
//             }
//             MaybeProcessed::Processed(_) => {
//                 ()
//             }
//         }
//     }
// }
