// devela_base_num::int::_docs
//
//! Defines constants for shared documentation on [`Int`] and [`NumInt`].
//
// notation: _INT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*

#![allow(unused, reason = "if only compiling either unsigned or signed…")]

crate::CONST! { hidden macro_export,
/* core */

_INT_FORMULA_SCALE = r#"$$ \large v^{\prime} = (b - a) \frac{v - min}{max - min} + a $$"#; // & wrap

/* combinatorics */

_INT_FORMULA_FACTORIAL = r#"$$ n! = 1 \cdot 2 \cdot 3 \cdot \ldots \cdot (n-1) \cdot n $$"#;
_INT_FORMULA_SUBFACTORIAL_RECURSIVE = r#"$$ !n = (n - 1) * (!(n - 1) + !(n - 2)) $$"#;
_INT_FORMULA_SUBFACTORIAL_SUMMATION = r#"$$ \large !n = n! \times \sum_{k=0}^{n} \frac{(-1)^k}{k!} $$"#;
_INT_FORMULA_SUBFACTORIAL_APPROXIMATION = r#" $$
\large !n = \left\lfloor \frac{n!}{e} + \frac{1}{2} \right\rfloor =
\left\lfloor n! \times \left(\frac{1}{e}\right) + \frac{1}{2} \right\rfloor $$"#;
_INT_ALGORITHM_SUBFACTORIAL = r#"$$
- Base cases: \( !0 = 1 \) and \( !1 = 0 \).
- For \( n > 1 \), compute \( !(n - 1) \) and \( !(n - 2) \) recursively, and combine them:
  $$ \large !n = (n - 1) \cdot (!(n - 1) + !(n - 2)). $$
"#;
_INT_FORMULA_PERMUTE = r#"$$ \large P(n,r) = \frac{n!}{(n−r)!} $$"#;
_INT_FORMULA_PERMUTE_REP = r#"$$ \large P_\text{rep}(n,r) = n_r $$"#;
_INT_FORMULA_COMBINE = r#"$$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$"#;
_INT_FORMULA_COMBINE_REP = r#"$$
\large C(n+r-1,r) = {n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$"#;

/* division */

_INT_NOTATION_DIV_CEIL = r#"$$ \large \left\lceil \frac{x}{y} \right\rceil $$"#;
_INT_FORMULA_DIV_CEIL = r#"$$
\large
\text{div\\_ceil}(a, b) =
\begin{cases}
\left\lfloor \frac{a}{b} \right\rfloor + 1 &
    \text{if } (r > 0 \land b > 0) \lor (r < 0 \land b < 0), \cr
\left\lfloor \frac{a}{b} \right\rfloor & \text{otherwise.}
\end{cases}
$$"#;
_INT_NOTATION_DIV_FLOOR = r#"$$ \large \left\lfloor \frac{x}{y} \right\rfloor $$"#;

/* primes */

_INT_NOTATION_PI = r#"$$ \pi(x) $$"#;
_INT_ALGORITHM_TOTIENT = r#"
This function iterates through all numbers from 2 up to the square
root of $|n|$. If it finds a divisor, it reduces `n` by its factors
and adjusts result accordingly. If after the loop, $n > 1$, it
means `n` has a prime factor greater than its square root, and the
function adjusts result for this last factor.
$$\large\varphi(n) =n \prod_{p\mid |n|} \left(1-\frac{1}{p}\right)$$
"#;

/* roots (square) */

_INT_FORMULA_IS_SQUARE = r#"$$
\large
\text{is\textunderscore square}(a) = \begin{cases}
\text{true} & \text{if } \left(\lfloor \sqrt{a} \rfloor\right)^2 = a \cr
\text{false} & \text{if } \left(\lfloor \sqrt{a} \rfloor\right)^2 \neq a
\end{cases}
$$"#;
_INT_ALGORITHM_SQRT_CEIL = r#"$$
\large
\begin{align}
\notag \left\lceil \sqrt{a} \thinspace\right\rceil = \begin{cases}
n & \text{if } n^2 = a \cr
n+1 & \text{if } n^2 < a \end{cases} \cr
\notag \normalsize\text{where } n = \lfloor \sqrt{a} \rfloor &
\end{align}
$$"#;
_INT_ALGORITHM_SQRT_FLOOR = r#"
$$ \large \left\lfloor \sqrt{a} \right\rfloor = n_{k} $$

Where $n_{k}$ is the result of a sequence of estimates that
starts with an initial $n_{0} = a/2$ which is updated using
[*Heron's method*](
https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method):

$$ \large
n_{i+1} = n_{i} - ( n_{i}^{2} - a) / 2n_{i},
\quad \small\text{for} \quad i = 0, 1, \ldots, k,
$$

Where $n_{i}$ is the current estimate, $n_{i+1}$ is the next
estimate, $a$ is self, and $k$ is the number of iterations
needed to converge to a solution, on the order of the number of
bits of self, about $O(\log_2 b)$, which for e.g. 128 bits would
be $ ±7 $ iterations.

Hence, the function continues updating the estimate until
reaching $n_{k}$, which provides the largest integer less than
or equal to the square root of `a`.
"#;
_INT_ALGORITHM_SQRT_ROUND = r#"$$
\begin{align}
\notag \left\lfloor\sqrt{a} \thinspace\right\rceil = \begin{cases}
n & \text{if } a - n^2 < (n+1)^2 - a \cr
n+1 & \text{if } a - n^2 \geq (n+1)^2 - a \end{cases} \cr
\notag \normalsize\text{where } n = \lfloor \sqrt{a} \rfloor &
\end{align}
$$"#;

/* roots */

_INT_FORMULA_ROOT_CEIL_SIGNED = r#"$$
\large \left\lceil |a|^{\frac{1}{n}} \right\rceil \cdot \text{sign}(a) = m $$"#;
_INT_PIECEWISE_ROOT_CEIL_SIGNED = r#"$$
\large
\begin{align}
\notag \text{If } n = 0, \text{ then error.} \cr
\notag \text{If } n = 1, \text{ then output } a. \cr
\notag \text{If } a = 0, \text{ then output } 0. \cr
\notag \text{If } a < 0 \text{ and } n \\% 2 = 0, \text{ then error.} \cr
\notag m = \lfloor |a|^{\frac{1}{n}} \rfloor \cr
\notag \left\lceil |a|^{\frac{1}{n}} \right\rceil =
\begin{cases}
m & \text{if } m^n = |a|, \cr
m+1 & \text{if } m^n < |a|.
\end{cases} \cr
\notag \text{Output: } m \cdot \text{sign}(a) &
\end{align}
$$"#;
_INT_ALGORITHM_ROOT_CEIL_SIGNED = r#"
The algorithm computes the smallest integer $ m $ such that:
$$ \large m^n \geq |a|. $$
Subject to the condition:
$$ \large a < 0 \quad \text{and} \quad n \\% 2 = 0 \quad \text{is invalid.} $$
The process is as follows:
1. Iteratively tests values starting from $ m = 1 $,
   calculating $ m^n $ until $ m^n > |a| $.
2. Computes the floored nth root as $ m - 1 $.
3. Checks if $ (m - 1)^n = |a| $.
   - If true, returns $ m - 1 \cdot \text{sign}(a) $.
   - Otherwise, returns $ m \cdot \text{sign}(a) $,
   the smallest integer such that $ m^n \geq |a| $."#;

_INT_FORMULA_ROOT_CEIL_UNSIGNED = r#"$$
\large \left\lceil a^{\frac{1}{n}} \right\rceil = m $$"#;
_INT_PIECEWISE_ROOT_CEIL_UNSIGNED = r#" $$
\large
\begin{align}
\notag \text{If } n = 0, \text{ then error.} \cr
\notag \text{If } n = 1, \text{ then output } a. \cr
\notag \text{If } a = 0, \text{ then output } 0. \cr
\notag m = \lfloor a^{\frac{1}{n}} \rfloor \cr
\notag \left\lceil a^{\frac{1}{n}} \right\rceil =
\begin{cases}
m & \text{if } m^n = a, \cr
m+1 & \text{if } m^n < a.
\end{cases} &
\end{align}
$$"#;
_INT_ALGORITHM_ROOT_CEIL_UNSIGNED = r#"
The algorithm computes the smallest integer $ m $ such that:
$$ \large m^n \geq a. $$
It first computes the floored nth root $ \lfloor a^{\frac{1}{n}} \rfloor $ and then:
1. Checks if $ \lfloor a^{\frac{1}{n}} \rfloor^n = a $.
2. If true, returns $ m = \lfloor a^{\frac{1}{n}} \rfloor $.
3. Otherwise, returns $ m = \lfloor a^{\frac{1}{n}} \rfloor + 1 $."#;

_INT_FORMULA_ROOT_FLOOR_SIGNED = r#"$$
\large \left\lfloor |a|^{\frac{1}{n}} \right\rfloor = m \cdot \text{sign}(a) $$"#;
_INT_PIECEWISE_ROOT_FLOOR_SIGNED = r#"$$
\large
\begin{align}
\notag \text{If } n = 0, \text{ then error.} \cr
\notag \text{If } n = 1, \text{ then output } a. \cr
\notag \text{If } a = 0, \text{ then output } 0. \cr
\notag \text{If } a < 0 \text{ and } n \\% 2 = 0, \text{ then error.} \cr
\notag m = \max \{ k \in ℤ \mid k^n \leq |a| \} \cr
\notag \text{Output: } m \cdot \text{sign}(a) &
\end{align}
$$"#;
_INT_ALGORITHM_ROOT_FLOOR_SIGNED = r#"
The algorithm computes the floored nth root,
$ \left\lfloor |a|^{\frac{1}{n}} \right\rfloor = m \cdot \text{sign}(a) $,
where $ m $ is the largest integer such that:
$$ \large m^n \leq |a|, $$
subject to the condition:
$$ \large a < 0 \quad \text{and} \quad n \\% 2 = 0 \quad \text{is invalid.} $$
The algorithm incrementally tests values starting from $ m = 1 $
and continues until the next value $ m+1 $ satisfies:
$$ \large (m+1)^n > |a|. $$
The function then returns $ m \cdot \text{sign}(a) $,
the largest integer such that $ m^n \leq |a| $,
with the sign adjusted for signed integers."#;

_INT_FORMULA_ROOT_FLOOR_UNSIGNED = r#"$$
\large \left\lfloor a^{\frac{1}{n}} \right\rfloor = m $$"#;
_INT_PIECEWISE_ROOT_FLOOR_UNSIGNED = r#"$$
\large
\begin{align}
\notag \text{If } n = 0, \text{ then error.} \cr
\notag \text{If } n = 1, \text{ then output } a. \cr
\notag \text{If } a = 0, \text{ then output } 0. \cr
\notag m = \max \{ k \in ℤ_{\geq 0} \mid k^n \leq a \} \cr
\notag \text{Output: } m &
\end{align}
$$"#;
_INT_ALGORITHM_ROOT_FLOOR_UNSIGNED = r#"
The algorithm computes the floored nth root,
$ \left\lfloor a^{\frac{1}{n}} \right\rfloor = m $,
where $ m $ is the largest integer such that:
$$ \large m^n \leq a. $$
It incrementally tests values starting from $ m = 1 $
and continues until the next value $ m+1 $ satisfies:
$$ \large (m+1)^n > a. $$
The function then returns $ m $."#;
}
