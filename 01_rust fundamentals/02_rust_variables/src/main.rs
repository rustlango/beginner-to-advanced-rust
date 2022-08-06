mod integers_mutable_immutable;
use integers_mutable_immutable::{add, unsigned_integers, uint_min_max, signed_integers_min_max};

fn main() {
    add::adding();
    unsigned_integers::unsigned_integers();
    uint_min_max::unsigned_integers();
    signed_integers_min_max::signed_integers_min_max();
}
