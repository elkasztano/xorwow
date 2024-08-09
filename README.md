# Xorwow

Several implementations of the Xorwow generator as proposed here:

[https://www.jstatsoft.org/article/view/v008i14](https://www.jstatsoft.org/article/view/v008i14)

Implements the `SeedableRng` and `RngCore` traits from [`rand_core`](https://docs.rs/rand_core/latest/rand_core/), so the crate may be used to create various kinds of pseudorandom number sequences.

## Example

```rust
use rand_core::{SeedableRng, RngCore}; // version = "0.6.4"
use xorwow::Xorwow160;

fn main() {

    // initialize the pseudorandom number generator
    let mut rng = Xorwow160::seed_from_u64(123456789);

    // clock it a few times
    (0..100).for_each(|_| { rng.next_u32(); });

    // generate a few numbers
    (0..10).for_each(|_| { println!("{}", rng.next_u32()); });

}
```
## Notes

* The generators in this crate are __not__ suitable for any kind of cryptographical use.

* If you need more functionality than just generating `u32` or `u64` integers, I highly recommend to have a look at the [rand book](https://rust-random.github.io/book/). As mentioned above, `SeedableRng` and `RngCore` are implemented.