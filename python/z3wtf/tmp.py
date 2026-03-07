from z3 import Real, Solver, sat

s = Solver()

x = Real("x")
y = Real("y")
z = Real("z")

s.add(x * x + y == 16)
s.add(z**3 == 27)
s.add(x * z == 6)

if s.check() == sat:
    m = s.model()
    print("model:", m)
    print("x =", m.eval(x))
    print("y =", m.eval(y))
    print("z =", m.eval(z))
else:
    print("unsat")

