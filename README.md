# u16_tic_tac_toe

A Rust library that provides a 2 byte representation of a tic-tac-toe grid.

## Background

I read an article where a tic-tac-toe grid was represented by two bitsets of length 9, one for the Xs one for the Os, and it got me thinking.
That representation is with 18 bits just over 2 bytes and it allows illegal grid states, where a field is both filled by X and O.

## Statespace

Given that the grid has 9 fields and each field has 3 possible states (Blank, occupied by X, occupied by O), there are 3<sup>9</sup>=19683 possible grids.
To enumerate all of these grids, 15 bits are needed (ld(19683) < 15).

## Grid representation

As we have 9 fields with 3 possible states each, we can encode a field as a trinary number with 9 digits.

Each field state is given a value:

| Field state | Value |
|-------------|-------|
| Blank       |     0 |
| X           |     1 |
| O           |     2 |

Each field is given a digit x_n:

```
x_0 | x_1 | x_2
----+-----+----
x_3 | x_4 | x_5
----+-----+----
x_6 | x_7 | x_8
```

Then the value for the grid is calculated as follows:

grid = x_0 + x_1 * 3 + x_2 * 3<sup>2</sup> + x_3 * 3<sup>3</sup> + x_4 * 3<sup>4</sup> + x_5 * 3<sup>5</sup> + x_6 * 3<sup>6</sup> + x_7 * 3<sup>7</sup> + x_8 * 3<sup>8</sup>

## Is this optimal?

No.

This library still allows the representation of states that can never appear during normal play.
For example a grid where every field is occupied by Xs.

This library also does not take advantage of the symmetries of the rules of tic-tac-toe.

