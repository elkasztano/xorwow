# Xorwow Generators

Several implementations of the Xorwow generator as proposed here:

[https://www.jstatsoft.org/article/view/v008i14](https://www.jstatsoft.org/article/view/v008i14)

Implements the `SeedableRng` and `RngCore` traits from [`rand_core`](https://docs.rs/rand_core/latest/rand_core/), so the crate may be used to create various kinds of pseudorandom number sequences.

## Examples

Get a few pseudorandom `u32` integers:

```rust
use rand_core::{SeedableRng, RngCore}; // version = "0.6.4"
use xorwowgen::Xorwow160;

fn main() {
    // initialize the pseudorandom number generator
    let mut rng = Xorwow160::seed_from_u64(123456789);

    // clock it a few times
    for _ in 0..100 {
        rng.next_u32();
    }

    // generate a few numbers
    for _ in 0..10 {
        println!("{}", rng.next_u32());
    }
}
```

Shuffle mutable array:

```rust
use xorwowgen::xorwow64::WrapA;
use rand::SeedableRng; // version = "0.8.5"
use rand::seq::SliceRandom;

fn main() {
    // initialize with true random bytes
    let mut rng = WrapA::from_entropy();

    let mut my_data = ["foo", "bar", "baz", "qux"];

    // shuffle data and print the result
    my_data.shuffle(&mut rng);
    println!("{:?}", &my_data);

    // shuffle again ...
    my_data.shuffle(&mut rng);
    println!("{:?}", &my_data);
}
```

## Notes

* The generators in this crate are __not__ suitable for any kind of cryptographical use.

* If you need more functionality than just generating `u32` or `u64` integers, I highly recommend to have a look at the [rand book](https://rust-random.github.io/book/). As mentioned above, `SeedableRng` and `RngCore` are implemented.
