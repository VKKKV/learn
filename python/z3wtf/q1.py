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

s = Solver()

x = Real("x")
y = Real("y")
z = Real("z")

s.add(x * x + y == 16)
s.add(z**3 == 27)
s.add(x * z == 6)

if s.check() == sat:
    m = s.model()
    print(m)

    x = m.eval(x).as_long()
    y = m.eval(y).as_long()
    z = m.eval(z).as_long()
    result = x * y * z
    print(result)
else:
    print("unsat")
