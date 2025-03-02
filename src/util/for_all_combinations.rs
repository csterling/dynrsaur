/// Calls a macro with every combination of the given identifiers.
/// 
/// E.g.
/// ```
/// use std::fmt::{Display, Formatter};
/// use dynrsaur::for_all_combinations;
///
/// struct DisplayAllCombos;
///
/// impl Display for DisplayAllCombos {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         macro_rules! write_ids {
///             ($($ids:ident),*) => {
///                 $(f.write_str(stringify!($ids))?;)*
///                 f.write_str("\n")?;
///             };
///         }
///
///         for_all_combinations!(write_ids => A, B, C);
/// 
///         Ok(())
///     }
/// }
///
/// assert_eq!(
///     DisplayAllCombos.to_string(),
///     "\nC\nB\nBC\nA\nAC\nAB\nABC\n"
/// )
/// ```
#[macro_export]
macro_rules! for_all_combinations {
    ($for_one_combination:ident => $elem1:ident $(,)? $($elems:ident),*) => {
        for_all_combinations!($for_one_combination => $($elems),*);
        for_all_combinations!(@recurse $for_one_combination => $elem1 : $($elems),*);
    };
    (@recurse $for_one_combination:ident => $($required_elems:ident),* $(,)? : $optional_elem1:ident $(,)? $($optional_elems:ident),*) => {
        for_all_combinations!(@recurse $for_one_combination => $($required_elems),* : $($optional_elems),*);
        for_all_combinations!(@recurse $for_one_combination => $($required_elems),*, $optional_elem1 : $($optional_elems),*);
    };
    (@recurse $for_one_combination:ident => $($required_elems:ident),* $(,)? :) => {
        $for_one_combination!($($required_elems),*);
    };
    ($for_one_combination:ident =>) => {
        $for_one_combination!();
    };
}
