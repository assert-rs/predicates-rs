extern crate predicates;

fn main() {
    use predicates::prelude::*;
    let predicate_fn = predicate::ne(5).not().and(predicate::ge(5));

    let (result, output) = predicate_fn.tree_eval(&5);
    println!("{}", output);
}
