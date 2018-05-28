extern crate predicates;

fn main() {
    use predicates::prelude::*;
    let predicate_fn = predicate::ne(5);
    println!("{}", predicate_fn.eval_to_string(&7));
}
