import sympy as sp
import sympy.physics.mechanics as mp

N = mp.ReferenceFrame("N")
t = sp.Symbol("t")

# Constants
g = sp.Symbol("g")
l = sp.Symbol("l")
m_c = sp.Symbol("m_c")
m_p = sp.Symbol("m_p")

# Dynamics
x = sp.Function("x")(t)
y = sp.Function("y")(t)
theta = sp.Function("theta")(t)
phi = sp.Function("phi")(t)

dx = x.diff(t)
dy = y.diff(t)
dtheta = theta.diff(t)
dphi = phi.diff(t)

# Points on system
O = mp.Point("O")
O.set_vel(N, 0)

# The Cart
C = O.locatenew("C", x * N.x + y * N.y)
C.set_vel(N, dx * N.x + dy * N.y)

# The tip of the pole
r_CP = (
    N.x * l * sp.sin(theta) * sp.cos(phi)
    + N.y * l * sp.sin(theta) * sp.sin(phi)
    + N.z * sp.cos(theta)
)
P = C.locatenew("P", r_CP)
P.set_vel(N, C.vel(N) + r_CP.diff(t, N))

# Mass and energy
cart = mp.Particle("cart", C, m_c)
pole = mp.Particle("pole", P, m_p)

# Generalized coordinates
q = [x, y, theta, phi]
u = [dx, dy, dtheta, dphi]

# Forces
Fx = sp.Symbol("Fx")
Fy = sp.Symbol("Fy")
force_list = [(C, Fx * N.x + Fy * N.y)]

# Setting up the Lagrangian
T = sp.Rational(1, 2) * m_c * (dx**2 + dy**2) + sp.Rational(1, 2) * m_p * (
    P.vel(N).magnitude() ** 2
)
V = m_p * g * l * sp.cos(theta)

LM = mp.LagrangesMethod(T - V, q, force_list, frame=N)
eom = LM.form_lagranges_equations()

sp.print_latex(eom[0])
