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
$ time insect '1 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s'
11123100 m²/s
insect   0.31s user 0.03s system 135% cpu 0.246 total

$ time larvae '1 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s'
11123100 m^2  s^-1
./target/release/larvae   0.00s user 0.00s system 80% cpu 0.009 total
```

## Features

- [x] Basic arithmetic (dimensionless numbers)

  - [x] +, -,
  - [x] \*, /
  - [x] ^
  - [ ] Other mathematical functions
    - [ ] %
    - [ ] !
    - [ ] sqrt
    - [ ] pi
    - [ ] exp
    - [ ] log

- [x] Dimensional analysis
  - [x] Dimensional analysis arithmetic
  - [ ] Basic units
    - [x] Length
    - [x] Mass
    - [x] Time
    - [ ] (more coming to a macro! near you)
  - [ ] Compound units (i.e. Newtons [kg m / s^2])
- [ ] Cli
  - [x] Basic functionality (currently only works when tokens are separated by white space)
  - [x] Equation parsing w/ recognition of dimensioned quantities
  - [ ] "Smart" Tokenisation of equation strings containing functions, units, values, etc.
  - [ ] Colors
  - [ ] Fancy characters (i.e. s⁻³·m⁻², Ω)
- [ ] Future goals
  - [ ] Support algebraic solving of variables w/ units
