// use nom::error::ParseError;
// use nom::{IResult, Parser};
//
// pub trait Alt<I, O, E> {
//     /// Tests each parser in the tuple and returns the result of the first one that succeeds
//     fn choice(&mut self, input: I) -> IResult<I, O, E>;
// }
//
// /// Tests a list of parsers one by one until one succeeds.
// ///
// /// It takes as argument a tuple of parsers. There is a maximum of 21
// /// parsers. If you need more, it is possible to nest them in other `alt` calls,
// /// like this: `alt(parser_a, alt(parser_b, parser_c))`
// ///
// /// ```rust
// /// # use nom::error_position;
// /// # use nom::{Err,error::ErrorKind, Needed, IResult};
// /// use nom::character::complete::{alpha1, digit1};
// /// use nom::branch::alt;
// /// # fn main() {
// /// fn parser(input: &str) -> IResult<&str, &str> {
// ///   alt((alpha1, digit1))(input)
// /// };
// ///
// /// // the first parser, alpha1, recognizes the input
// /// assert_eq!(parser("abc"), Ok(("", "abc")));
// ///
// /// // the first parser returns an error, so alt tries the second one
// /// assert_eq!(parser("123456"), Ok(("", "123456")));
// ///
// /// // both parsers failed, and with the default error type, alt will return the last error
// /// assert_eq!(parser(" "), Err(Err::Error(error_position!(" ", ErrorKind::Digit))));
// /// # }
// /// ```
// ///
// /// With a custom error type, it is possible to have alt return the error of the parser
// /// that went the farthest in the input data
// pub fn alt<I: Clone, O, E: ParseError<I>, List: nom::branch::Alt<I, O, E>>(
//     mut l: List,
// ) -> impl FnMut(I) -> IResult<I, O, E> {
//     move |i: I| l.choice(i)
// }
//
// macro_rules! alt_trait(
//   ($first:ident $second:ident $($id: ident)+) => (
//     alt_trait!(__impl $first $second; $($id)+);
//   );
//   (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
//     alt_trait_impl!($($current)*);
//
//     alt_trait!(__impl $($current)* $head; $($id)+);
//   );
//   (__impl $($current:ident)*; $head:ident) => (
//     alt_trait_impl!($($current)*);
//     alt_trait_impl!($($current)* $head);
//   );
// );
//
// macro_rules! alt_trait_impl(
//   ($($id:ident)+) => (
//     impl<
//       Input: Clone, Output, Error: ParseError<Input>,
//       $($id: Parser<Input, Output, Error>),+
//     > Alt<Input, Output, Error> for ( $($id),+ ) {
//
//       fn choice(&mut self, input: Input) -> IResult<Input, Output, Error> {
//         match self.0.parse(input.clone()) {
//           Err(Err::Error(e)) => alt_trait_inner!(1, self, input, e, $($id)+),
//           res => res,
//         }
//       }
//     }
//   );
// );
//
// macro_rules! alt_trait_inner(
//   ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident $($id:ident)+) => (
//     match $self.$it.parse($input.clone()) {
//       Err(Err::Error(e)) => {
//         let err = $err.or(e);
//         succ!($it, alt_trait_inner!($self, $input, err, $($id)+))
//       }
//       res => res,
//     }
//   );
//   ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident) => (
//     Err(Err::Error(Error::append($input, ErrorKind::Alt, $err)))
//   );
// );
//
// alt_trait!(A B C D E F G H I J K L M N O P Q R S T U);
//
// // Manually implement Alt for (A,), the 1-tuple type
// impl<Input, Output, Error: ParseError<Input>, A: Parser<Input, Output, Error>>
// nom::branch::Alt<Input, Output, Error> for (A,)
// {
//     fn choice(&mut self, input: Input) -> IResult<Input, Output, Error> {
//         self.0.parse(input)
//     }
// }
//
// macro_rules! succ (
//   (0, $submac:ident ! ($($rest:tt)*)) => ($submac!(1, $($rest)*));
//   (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
//   (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
//   (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
//   (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
//   (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
//   (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
//   (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
//   (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
//   (9, $submac:ident ! ($($rest:tt)*)) => ($submac!(10, $($rest)*));
//   (10, $submac:ident ! ($($rest:tt)*)) => ($submac!(11, $($rest)*));
//   (11, $submac:ident ! ($($rest:tt)*)) => ($submac!(12, $($rest)*));
//   (12, $submac:ident ! ($($rest:tt)*)) => ($submac!(13, $($rest)*));
//   (13, $submac:ident ! ($($rest:tt)*)) => ($submac!(14, $($rest)*));
//   (14, $submac:ident ! ($($rest:tt)*)) => ($submac!(15, $($rest)*));
//   (15, $submac:ident ! ($($rest:tt)*)) => ($submac!(16, $($rest)*));
//   (16, $submac:ident ! ($($rest:tt)*)) => ($submac!(17, $($rest)*));
//   (17, $submac:ident ! ($($rest:tt)*)) => ($submac!(18, $($rest)*));
//   (18, $submac:ident ! ($($rest:tt)*)) => ($submac!(19, $($rest)*));
//   (19, $submac:ident ! ($($rest:tt)*)) => ($submac!(20, $($rest)*));
//   (20, $submac:ident ! ($($rest:tt)*)) => ($submac!(21, $($rest)*));
// );
