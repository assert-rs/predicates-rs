extern crate predicates;

fn main() {
    use predicates::prelude::*;
    let predicate_fn = predicate::ne(5).and(predicate::ge(5));

    println!("{}", predicate_fn.tree_eval(&7));
}
