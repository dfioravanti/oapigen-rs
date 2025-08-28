mod models;
mod parsing;

use serde::Deserialize;
use serde::Serialize;

// [dependencies]
// prettyplease = "0.2"
// syn = { version = "2", default-features = false, features = ["full", "parsing"] }

const INPUT: &str = stringify! {
    use serde::{Deserialize};
    use serde::{Serialize};
    impl<T, U> Into<U> for T where U: From<T> {
        fn into(self) -> U { U::from(self) }
    }
};

fn main() {
    let syntax_tree = syn::parse_file(INPUT).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    print!("{}", formatted);
}
