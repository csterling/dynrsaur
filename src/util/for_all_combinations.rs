/// Calls the macro `for_one_combination` with every combination of the given identifiers.
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
