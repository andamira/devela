// devela::num::float::shared_docs
//
//! Defines constants for shared documentation on [`Float`] and [`ExtFloat`].
#![doc = crate::doclink!(custom devela "[`ExtFloat`]" "num/trait.ExtFloat.html")]
//
// notation: _FLOAT_FORMULA_*

crate::CONST! { hidden macro_export,
/* Rounding */

_FLOAT_FORMULA_FLOOR = r#"$$
\large \lfloor x \rfloor = \max \{ n \in ℤ \,|\, n \leq x \} $$"#;
_FLOAT_FORMULA_CEIL = r#"$$
\lceil x \rceil = \min \{ n \in ℤ \,|\, n \geq x \} $$"#;
_FLOAT_FORMULA_ROUND_TIES_AWAY = r#"$$
\text{round\\_ties\\_away}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \text{ or }
    (x - \lfloor x \rfloor = 0.5 \text{ and } x > 0) \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \text{ or }
    (x - \lfloor x \rfloor = 0.5 \text{ and } x < 0)
\end{cases}
$$"#;
_FLOAT_FORMULA_ROUND_TIES_EVEN = r#"$$
\text{round\\_ties\\_even}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is even} \cr
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is odd}
\end{cases}
$$"#;
_FLOAT_FORMULA_ROUND_TIES_ODD = r#"$$
\text{round\\_ties\\_odd}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is odd} \cr
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is even}
\end{cases}
$$"#;
_FLOAT_FORMULA_TRUNC = r#"$$
\text{trunc}(x) = \begin{cases}
\lfloor x \rfloor, & \text{if } x \geq 0 \\
\lceil x \rceil, & \text{if } x < 0
\end{cases}
$$"#;
_FLOAT_FORMULA_FRACT = r#"$$ \text{fract}(x) = x - \text{trunc}(x) $$"#;
_FLOAT_FORMULA_SPLIT = r#"$$ \text{split}(x) = (\text{trunc}(x), \text{fract}(x)) $$"#;


/* Linear interpolation */

_FLOAT_FORMULA_SCALE = r#"$$
\large \text{scale}(x, min, max, u, v) = (v - u) \frac{x - min}{max - min} + u $$"#;
_FLOAT_FORMULA_LERP = r#"$$ \large \text{lerp}(x, u, v) = (1 - x) \cdot u + x \cdot v $$"#;

/* Exponentials and Logarithms */

_FLOAT_FORMULA_POWF_SERIES = r#"$$ \large x^y = e^{y \cdot \ln(x)} $$"#;
_FLOAT_FORMULA_HYPOT_NR = r#"$$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$"#;
_FLOAT_FORMULA_HYPOT_FISR = r#"$$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$"#;

_FLOAT_FORMULA_EXP_SERIES = r#"$$
e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
For values $ x < 0 $ it uses the identity: $$ e^x = \frac{1}{e^-x} $$"#;
_FLOAT_FORMULA_EXP_M1_SERIES = r#"$$
e^x -1 = x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
For values $ x < 0 $ it uses the identity: $$ e^x -1 = -\frac{1}{e^{-x}+1} $$
For values $ x > 0.001 $ it uses [`exp_series`][Self::exp_series]."#;
_FLOAT_FORMULA_EXP2_SERIES = r#"
The series based on the formula $ 2^x = e^{x \ln(2)} $ is:
$$ 2^x = 1 + x \ln(2) + \frac{(x \ln(2))^2}{2!} +
\frac{(x \ln(2))^3}{3!} + \frac{(x \ln(2))^4}{4!} + \cdots $$"#;
_FLOAT_FORMULA_LN_SERIES = r#"$$
\ln(x) = 2 \left( \frac{x-1}{x+1} + \frac{1}{3} \left( \frac{x-1}{x+1} \right)^3 +
\frac{1}{5} \left( \frac{x-1}{x+1} \right)^5 + \cdots \right) $$"#;
_FLOAT_FORMULA_LOG_SERIES = r#"$$ \log_{\text{base}}(x) = \frac{\ln(x)}{\ln(\text{base})} $$"#;
_FLOAT_FORMULA_LOG2_SERIES = r#"$$ \log_{2}(x) = \frac{\ln(x)}{\ln(2)} $$"#;
_FLOAT_FORMULA_LOG10_SERIES = r#"$$ \log_{10}(x) = \frac{\ln(x)}{\ln(10)} $$"#;

/* Trigonometry */

_FLOAT_FORMULA_SIN_SERIES = r#"$$
\sin(x) = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \frac{x^7}{7!} + \cdots $$"#;
_FLOAT_FORMULA_COS_SERIES = r#"$$
\cos(x) = 1 - \frac{x^2}{2!} + \frac{x^4}{4!} - \frac{x^6}{6!} + \cdots $$"#;
_FLOAT_FORMULA_TAN_SERIES = r#"$$ \tan(x) = \frac{\sin(x)}{\cos(x)} $$"#;
_FLOAT_FORMULA_ASIN_SERIES = r#"$$
\arcsin(x) = x + \left( \frac{1}{2} \right) \frac{x^3}{3} +
\left( \frac{1}{2} \cdot \frac{3}{4} \right) \frac{x^5}{5} +
\left( \frac{1}{2} \cdot \frac{3}{4} \cdot \frac{5}{6} \right) \frac{x^7}{7} +
\cdots $$"#;
_FLOAT_FORMULA_ACOS_SERIES = r#"$$ \arccos(x)=2π-arcsin(x) $$"#;
_FLOAT_FORMULA_ATAN_SERIES = r#"$$
\arctan(x) = x - \frac{x^3}{3} + \frac{x^5}{5} - \frac{x^7}{7} + \cdots $$
For values $ |x| > 1 $ it uses the identity:
$$ \arctan(x) = \frac{\pi}{2} - \arctan(\frac{1}{x}) $$"#;
_FLOAT_FORMULA_SINH_SERIES = r#"$$ \sinh(x) = \frac{e^x - e^{-x}}{2} $$"#;
_FLOAT_FORMULA_COSH_SERIES = r#"$$ \cosh(x) = \frac{e^x + e^{-x}}{2} $$"#;
_FLOAT_FORMULA_TANH_SERIES = r#"$$ \tanh(x) = \frac{\sinh(x)}{\cosh(x)} $$"#;
_FLOAT_FORMULA_ASINH_SERIES = r#"$$ \text{asinh}(x) = \ln(x + \sqrt{x^2 + 1}) $$"#;
_FLOAT_FORMULA_ACOSH_SERIES = r#"$$ \text{acosh}(x) = \ln(x + \sqrt{x^2 - 1}) $$"#;
_FLOAT_FORMULA_ATANH_SERIES = r#"$$
\text{atanh}(x) = \frac{1}{2} \ln\left(\frac{1 + x}{1 - x}\right) $$"#;

/* Calculus */

_FLOAT_FORMULA_DERIVATIVE = r#"$$ f^{\prime}(x) \approx \frac{f(x + h) - f(x)}{h} $$"#;
_FLOAT_FORMULA_INTEGRATE = r#"$$
\int_a^b f(x) \, dx \approx \sum_{i=0}^{n-1} f(x_i) \cdot \Delta x $$
where $$ \Delta x = \frac{b-a}{n} $$"#;
_FLOAT_FORMULA_PARTIAL_DERIVATIVE_X = r#"
\frac{\partial f}{\partial x} \approx \frac{f(x + h, y) - f(x, y)}{h} $$"#;
_FLOAT_FORMULA_PARTIAL_DERIVATIVE_Y = r#"
\frac{\partial f}{\partial x} \approx \frac{f(x + h, y) - f(x, y)}{h} $$"#;
}
