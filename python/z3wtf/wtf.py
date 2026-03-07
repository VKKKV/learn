from z3 import *
from z3 import (
    BitVec,
    Distinct,
    Int,
    Ints,
    IntVector,
    Optimize,
    Or,
    Product,
    Real,
    Solver,
    Sum,
    sat,
)


# not work
def diophantine_equation():
    s = Solver()

    x = Int("x")
    y = Int("y")
    z = Int("z")

    s.add(x > 0, y > 0, z > 0)

    # s.add(x / (y + z) + y / (x + z) + z / (x + y) == 4)

    term_x = x * (x + z) * (x + y)
    term_y = y * (y + z) * (x + y)
    term_z = z * (y + z) * (x + z)
    right_side = 4 * (y + z) * (x + z) * (x + y)

    s.add(term_x + term_y + term_z == right_side)

    while s.check() == sat:
        m = s.model()
        print(m)
        val_x = m[x]
        val_y = m[y]
        val_z = m[z]
        s.add(Or(x != val_x, y != val_y, z != val_z))


def y():
    x = 10
    y = 61
    z = 3
    r = x / (y + z) + y / (x + z) + z / (x + y)
    print(r)


if __name__ == "__main__":
    diophantine_equation()
    # y()
