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


def equationA(x):
    s = Solver()
    s.add(x + 4 == 7)
    result = s.check()

    if result == sat:
        print(s.model())
    else:
        print("UNSAT")


def equationB(x, y):
    s = Solver()
    s.add(x + y == 17)
    s.add(2 * x == y)

    result = s.check()

    if result == sat:
        print(s.model())
    else:
        print("UNSAT")


def multi_solution(x):
    s = Solver()
    s.add(x**2 == 4)

    while s.check() == sat:
        m = s.model()
        print(m)
        s.add(x != m[x])


def circle_equation(x, y):
    s = Solver()
    s.add(x**2 + y**2 == 25)

    while s.check() == sat:
        m = s.model()
        print(m)
        val_x = m[x]
        val_y = m[y]

        # De Morgan's Laws
        s.add(Or(x != val_x, y != val_y))


def wtf():
    s = Solver()

    a, b, c, d = Ints("a b c d")
    s.add(a > 0, b > 0, c > 0, d > 0)
    s.add(a + b == c + d)
    s.add(a * b == c * d)

    # We can add many constraints in one `add` call
    s.add(a != b, a != c, a != d, b != c, b != d, c != d)

    print(s.check())


def wtf2():
    n = 3
    lhs = IntVector("lhs", n)
    rhs = IntVector("rhs", n)

    # s = Solver()

    s = Optimize()
    s.minimize(Sum(lhs))

    for x in lhs + rhs:
        s.add(x >= 0)
    s.add(sum(lhs) == sum(rhs))
    s.add(Product(lhs) == Product(rhs))
    s.add(Distinct(lhs + rhs))

    if s.check() == sat:
        m = s.model()
        l = [m[l].as_long() for l in lhs]
        r = [m[r].as_long() for r in rhs]
        print(l, r)
        print(f"sum={sum(l)} prod={Product(l)}")
    else:
        print("unsat")


if __name__ == "__main__":
    # wtf()
    wtf2()

    # x = Int("x")
    # y = Int("y")
    # circle_equation(x, y)

    # x = Int("x")
    # y = Int("y")
    # equationA(x)

    # x = Real("x")
    # y = Real("y")
    # equationB(x, y)

    # x = Int("x")
    # multi_solution(x)
