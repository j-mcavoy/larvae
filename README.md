# larvae

![larvae](https://gitlab.com/uploads/-/system/project/avatar/32538279/larvae.jpg?width=64)

a super fast scientific calculator with dimensional analysis support written in Rust 🦀🐛

heavily inspired from [insect](https://github.com/sharkdp/insect)

## Usage:

Command mode:

```bash
$ larvae '1 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s'
11123100 m^2  s^-1
```

Interactive mode:

```bash
$ larvae
>>> 1 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s
11123100 m^2  s^-1
>>>
```

## Why re-write insect in Rust?

1. I thought it'd be fun
2. Maybe I'd learn something cool like Earley parsing
3. Maybe with a little more work, it'll even be as useful as insect someday
4. It's super fast. It's like 2 orders of magnitude faster than insect in its current state (albeit it's still lacking some features):

```bash
$ hyperfine \
    'insect "e / e * log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s"' \
    'larvae "e / e * log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s"'
Benchmark 1: insect "e / e * log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s"
  Time (mean ± σ):     307.8 ms ±   4.4 ms    [User: 384.5 ms, System: 29.0 ms]
  Range (min … max):   301.2 ms … 314.0 ms    10 runs

Benchmark 2: larvae "e / e * log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s"
  Time (mean ± σ):       1.9 ms ±   0.7 ms    [User: 1.7 ms, System: 1.3 ms]
  Range (min … max):     0.6 ms …   4.3 ms    489 runs

  Warning: Command took less than 5 ms to complete. Results might be inaccurate.

Summary
  'larvae "e / e * log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s"' ran
  160.53 ± 57.94 times faster than 'insect "e / e * log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s"'
```

## Features

- [x] Basic arithmetic (dimensionless numbers)

  - [x] +, -,
  - [x] \*, /
  - [x] ^
  - [x] Other mathematical functions
    - [x] %
    - [x] !
    - [x] sqrt
    - [x] pi
    - [x] exp
    - [x] log

- [x] Dimensional analysis
  - [x] Dimensional analysis arithmetic
  - [x] Basic units
    - [x] Length
    - [x] Mass
    - [x] Time
  - [x] Compound units
    - [x] Force
- [ ] Cli
  - [x] Basic functionality (currently only works when tokens are separated by white space)
  - [x] Equation parsing w/ recognition of dimensioned quantities
  - [ ] "Smart" Tokenisation of equation strings containing functions, units, values, etc.
  - [ ] Colors
  - [x] Fancy characters (i.e. s⁻³·m⁻², Ω)
- [ ] Future goals
  - [ ] Support algebraic solving of variables w/ units
