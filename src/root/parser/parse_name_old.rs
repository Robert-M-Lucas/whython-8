// #[derive(Debug)]
// pub enum NameConnectors {
//     NonStatic,
//     Static,
// }
//
// #[derive(Debug, Dissolve, Getters)]
// pub struct UnresolvedNameToken {
//     location: Location,
//     containing_class: Option<String>,
//     indirection: Indirection,
//     base: String,
//     names: Vec<(NameConnectors, String)>,
//     function_call: Option<Vec<EvaluableToken>>,
// }
//
// impl UnresolvedNameToken {
//     pub fn new_unresolved(
//         s: &Span,
//         containing_class: Option<String>,
//     ) -> UnresolvedNameToken {
//         let location = Location::from_span(s);
//         let file_location = location.path().clone();
//         UnresolvedNameToken {
//             location,
//             containing_class,
//             indirection: Indirection(0),
//             base: s.to_string(),
//             names: Vec::new(),
//             function_call: None
//         }
//     }
//
//     pub fn new_unresolved_top(
//         s: String,
//         location: Location
//     ) -> UnresolvedNameToken {
//         UnresolvedNameToken {
//             location,
//             containing_class: None,
//             indirection: Indirection(0),
//             base: s,
//             names: Vec::new(),
//             function_call: None
//         }
//     }
// }

// pub fn parse_full_name_old(s: Span, containing_class: Option<String>) -> ParseResult<Span, UnresolvedNameToken> {
//     // TOD: Handle indirection
//
//     let (s, _) = discard_ignored(s)?;
//
//     let location = Location::from_span(&s);
//
//     let (mut s, base_name) = parse_name::parse_simple_name(s)?;
//
//     let mut names = Vec::new();
//     let mut function_call = None;
//
//     if let Ok((ns, contents)) = default_section(s, '(') {
//         function_call = Some(parse_arguments(contents, containing_class.as_ref().and_then(|s| Some(s.as_str())))?.1);
//         s = ns;
//     }
//     else {
//         loop {
//             let ns;
//             let connector = if let Some(next) = s.chars().next() {
//                 if next == '.' {
//                     ns = s.take_split(1).0;
//                     NameConnectors::NonStatic
//                 } else if next == ':' && s.chars().nth(1).is_some_and(|c| c == ':') {
//                     ns = s.take_split(2).0;
//                     NameConnectors::Static
//                 }
//                 else {
//                     break;
//                 }
//             } else {
//                 break;
//             };
//
//             let (ns, _) = discard_ignored(ns)?;
//
//             let (ns, name) = parse_name::parse_simple_name(ns)?;
//             // ? Shouldn't be necessary due to parse_simple_name call but still included in case of implementation change
//             let (ns, _) = discard_ignored(ns)?;
//
//             names.push((connector, name.to_string()));
//
//             if let Ok((ns, contents)) = default_section(ns, '(') {
//                 function_call = Some(parse_arguments(contents, containing_class.as_ref().and_then(|s| Some(s.as_str())))?.1);
//                 s = ns;
//                 break;
//             }
//
//             s = ns;
//         }
//     }
//
//     Ok((
//         s,
//         UnresolvedNameToken {
//             location,
//             containing_class,
//             indirection: Indirection(0), // TODO
//             base: base_name.to_string(),
//             names,
//             function_call,
//         },
//     ))
// }
