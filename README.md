# aoc-rust

The main crate `aoc` aims to provide a way for a user to develop lightweight
single-binary [advent of code][aoc] runner.

[aoc]: https://adventofcode.com/

## Usage

Firstly, you will want to create a parser/solver combo. The signature for
parsers is simple. You take bytes by reference and you return your input type
`T`. That is
```rust
type ParserFn<'a, T> = fn(input: &'a [u8]) -> T;
```

Consequently, your solver function has signature
```rust
type SolverFn<'a, T> = fn(input: &'a T) -> Solution;
```

The main way to define a solver is by using the `impl_solver!` or
`impl_solver_named!` macros. The format is `impl_solver!(identifier, day,
[parse], [solve_a], [solve_b])` and `impl_solver_named!(identifier, day, "string
identifier", [parse], [solve_a], [solve_b])`, where `parse`, `solve_a`, and
`solve_b` are optional. This creates a
```rust
pub const identifier: AdventSolver = AdventSolver::new(/* ... */);
```
inside your current file.

Next, you will want to do something similar to create a year solver.
```rust
pub const SOLVER: AdventYearSolver = AdventYearSolver::new(
    2025,
    &[day01::identifier, day02::different_identifier, /* ... */],
);
```

Next, we can call `aoc_main()` from within `main()` with the desired year
solver. This will parse the command line arguments and run the desired day. You
can change the error to anything that implements `From<AdventRuntimeError>`.
```rust
fn main() -> Result<(), AdventRuntimeError> {
    aoc_main(&solvers::SOLVER)
}
```
Support for a collection of year solvers is in the works.

## Running

To run the runner for a specific day of the year, just specify the day as the
first argument. If no day is specified, it defaults to 1.
```console
$ cargo run --release -- 1
    Finished `release` profile [optimized] target(s) in 0.08s
     Running `target/release/aoc-2016 1`
day 1 parse iter: parse: 0us
day 1 parse iter: a (4us): 243
day 1 parse iter: b (62us): 142
day 1 parse collect: parse: 7us
day 1 parse collect: a (0us): 243
day 1 parse collect: b (50us): 142
ran 2 solvers
```

## Benchmarks

To benchmark your code, use `--bench` and optionally specify the number of
`--samples` to take. This uses [brunch][brunch] under the hood. It is okay at
benching, but ideally we should get a better benchmarking tool that does not
benchmark `Drop` times or supports arbitrary functions that return
`std::time::Duration`. Unfortunately [criterion][criterion] does not fit the
bill here as there is no way to actually run it without criterion defining its
own main function, which is dumb.
```console
$ cargo run --release -- 1 --bench --samples 10000
    Finished `release` profile [optimized] target(s) in 0.08s
     Running `target/release/aoc-2016 1 --bench --samples=10000`
Method                               Mean         Samples
---------------------------------------------------------
day 1 parse iter: parse          26.13 ns    9,985/10,000
day 1 parse iter: solve_a         3.04 μs    9,973/10,000
day 1 parse iter: solve_b        30.83 μs    9,917/10,000
---------------------------------------------------------
day 1 parse collect: parse        3.09 μs    9,963/10,000
day 1 parse collect: solve_a    358.46 ns    9,985/10,000
day 1 parse collect: solve_b     29.67 μs    9,914/10,000
```

[brunch]: https://crates.io/crates/brunch
[criterion]: https://crates.io/crates/criterion

## Under the hood

So how does this wrapper even work, anyway? Long story short, we convert the
functions you pass to `AdventSolver::new()` to two types of function pointers.
```rust
type ParserInternal = fn(input: &[u8]) -> ();
type SolverInternal = fn(input: &()) -> Solution;
```

These will be cast back to
```rust
type ParserActual<T> = fn(input: &[u8]) -> T;
type SolverActual<T> = fn(input: &T) -> Solution;
```

and we are essentially praying that `rustc` will determine the same ABI for
functions and function pointers so that RVO/non-RVO returns are resolved
correctly. The input type `T` will be passed around as a type-erased
`core::ptr::NonNull<()>` until it hits the runner, where it will be converted
back to a `&T` and passed to your solver function. There should be no undefined
behavior, but there is a test which can be run under `miri` via `cargo miri
test` and it also reports no UB.
