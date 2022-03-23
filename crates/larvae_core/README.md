# larvae-core

![larvae](https://gitlab.com/uploads/-/system/project/avatar/32538279/larvae.jpg?width=64)

Dimensional analysis calculator library used for the [larvae cli](https://gitlab.com/j-mcavoy/larvae)

## Features

- [x] Dimensional analysis equation parsing and evaluation
  - [x] efficient Earley parsing grammar of mathematical expressions with units
- [x] Basic Arithmetic (dimensionless numbers)

  - [x] +, -
  - [x] \*, /
  - [x] ^
  - [x] Other mathematical functions
    - [x] %
    - [x] !
    - [x] sqrt
    - [x] pi
    - [x] exp
    - [x] ln
    - [x] log

- [x] Dimensional analysis

  - [x] Basic dimensions
    - [x] Length
      - [x] metric (m, km, cm, etc.)
      - [ ] imperial (in. ft, mi)
    - [x] Mass
      - [x] metric (kg, g, etc.)
      - [ ] imperial (lb, oz., ton)
    - [x] Time
      - metric (s, ms, min, hr, etc.)
  - [x] Compound dimensions
    - [x] Force
      - [x] metric (N, kN, etc.)
      - [ ] imperial (lbf)
  - [x] ability to easily create custom dimensions/unit systems using macros
  - [x] Dimensional analysis arithmetic
    - [x] Quantity arithmetic
      - [x] +,-
        - [x] error checking if dimensions are compatible (i.e. can't do 1m + 6s)
      - [x] \*,/
      - [x] partial dimensions (i.e sqrt(1m) = 1 m ^ 0.5)
    - [x] Converting quantities
    - [x] Converting between unit systems

- [ ] Future goals
  - [ ] Support algebraic solving of variables w/ units
