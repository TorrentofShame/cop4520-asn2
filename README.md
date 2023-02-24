

## Problem 1:

To build and run:

```bash
make problem1

# or provide the number of guests as an integer (default 100)

make problem1 100

# or manually

cargo b --release

target/release/problem1

# or provide the number of guests as an integer (default 100)

target/release/problem1 100
```

### Summary

The guests appoint the first person that enters the labyrinth as the "counter",
this guest will not eat any cake yet will count the number of times that they
notice that the cake is missing, replacing as they do so.

All other guests will not replace a missing cake,
but will eat a cake that is present if they have not eaten a cake already.

This allows the counter guest to conclude that when they have counted a number of eaten
cakes equal to the number of total guests, that all of the guests have gone through
the labyrinth at least once.


## Problem 2:

To build and run:

```bash
make problem2

# or provide the number of guests as an integer (default 100)

make problem2 100

# or manually

cargo b --release

target/release/problem2

# or provide the number of guests as an integer (default 100)

target/release/problem2 100
```

### Summary

I chose strategy 2 for this solution as
it maximizes the time that the vase room is occupied
and allows the guests to perform other tasks while the showroom is unavailable.
