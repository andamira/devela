// devela::num::float::wrapper::shared_docs
//
//! Defines constants for shared documentation on [`Float`] and [`ExtFloat`].
//

crate::CONST! {
/* Rounding */

pub(crate) FORMULA_FLOOR = r#"$$
\lfloor x \rfloor = \max \{ n \in \mathbb{Z} \,|\, n \leq x \} $$"#;
pub(crate) FORMULA_CEIL = r#"$$
$$ \lceil x \rceil = \min \{ n \in \mathbb{Z} \,|\, n \geq x \} $$"#;
pub(crate) FORMULA_ROUND_TIES_AWAY = r#"$$
\text{round\\_ties\\_away}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \text{ or }
    (x - \lfloor x \rfloor = 0.5 \text{ and } x > 0) \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \text{ or }
    (x - \lfloor x \rfloor = 0.5 \text{ and } x < 0)
\end{cases}
$$"#;
pub(crate) FORMULA_ROUND_TIES_EVEN = r#"$$
\text{round\\_ties\\_even}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is even} \cr
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is odd}
\end{cases}
$$"#;
pub(crate) FORMULA_ROUND_TIES_ODD = r#"$$
\text{round\\_ties\\_odd}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is odd} \cr
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is even}
\end{cases}
$$"#;
pub(crate) FORMULA_TRUNC = r#"$$
\text{trunc}(x) = \begin{cases}
\lfloor x \rfloor, & \text{if } x \geq 0 \\
\lceil x \rceil, & \text{if } x < 0
\end{cases}
$$"#;
pub(crate) FORMULA_FRACT = r#"$$ \text{fract}(x) = x - \text{trunc}(x) $$"#;
pub(crate) FORMULA_SPLIT = r#"$$ \text{split}(x) = (\text{trunc}(x), \text{fract}(x)) $$"#;


/* Linear interpolation */

pub(crate) FORMULA_SCALE = r#"$$
\large \text{scale}(x, min, max, u, v) = (v - u) \frac{x - min}{max - min} + u $$"#;
pub(crate) FORMULA_LERP = r#"$$ \large \text{lerp}(x, u, v) = (1 - x) \cdot u + x \cdot v $$"#;

/* Exponentials and Logarithms */

pub(crate) FORMULA_POWF_SERIES = r#"$$ \large x^y = e^{y \cdot \ln(x)} $$"#;
pub(crate) FORMULA_HYPOT_NR = r#"$$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$"#;
pub(crate) FORMULA_HYPOT_FISR = r#"$$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$"#;

pub(crate) FORMULA_EXP_SERIES = r#"$$
e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
For values $ x < 0 $ it uses the identity: $$ e^x = \frac{1}{e^-x} $$"#;
pub(crate) FORMULA_EXP_M1_SERIES = r#"$$
e^x -1 = x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
For values $ x < 0 $ it uses the identity: $$ e^x -1 = -\frac{1}{e^{-x}+1} $$
For values $ x > 0.001 $ it uses [`exp_series`][Self::exp_series]."#;
pub(crate) FORMULA_EXP2_SERIES = r#"$$
The series based on the formula $ 2^x = e^{x \ln(2)} $ is:
$$ 2^x = 1 + x \ln(2) + \frac{(x \ln(2))^2}{2!} +
\frac{(x \ln(2))^3}{3!} + \frac{(x \ln(2))^4}{4!} + \cdots $$"#;
pub(crate) FORMULA_LN_SERIES = r#"$$
\ln(x) = 2 \left( \frac{x-1}{x+1} + \frac{1}{3} \left( \frac{x-1}{x+1} \right)^3 +
\frac{1}{5} \left( \frac{x-1}{x+1} \right)^5 + \cdots \right) $$"#;
pub(crate) FORMULA_LOG_SERIES = r#"$$ \log_{\text{base}}(x) = \frac{\ln(x)}{\ln(\text{base})} $$"#;
pub(crate) FORMULA_LOG2_SERIES = r#"$$ \log_{2}(x) = \frac{\ln(x)}{\ln(2)} $$"#;
pub(crate) FORMULA_LOG10_SERIES = r#"$$ \log_{10}(x) = \frac{\ln(x)}{\ln(10)} $$"#;

/* Trigonometry */

pub(crate) FORMULA_SIN_SERIES = r#"$$
\sin(x) = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \frac{x^7}{7!} + \cdots $$"#;
pub(crate) FORMULA_COS_SERIES = r#"$$
\cos(x) = 1 - \frac{x^2}{2!} + \frac{x^4}{4!} - \frac{x^6}{6!} + \cdots $$"#;
pub(crate) FORMULA_TAN_SERIES = r#"$$ \tan(x) = \frac{\sin(x)}{\cos(x)} $$"#;
pub(crate) FORMULA_ASIN_SERIES = r#"$$
\arcsin(x) = x + \left( \frac{1}{2} \right) \frac{x^3}{3} +
\left( \frac{1}{2} \cdot \frac{3}{4} \right) \frac{x^5}{5} +
\left( \frac{1}{2} \cdot \frac{3}{4} \cdot \frac{5}{6} \right) \frac{x^7}{7} +
\cdots $$"#;
pub(crate) FORMULA_ACOS_SERIES = r#"$$ \arccos(x)=2π-arcsin(x) $$"#;
pub(crate) FORMULA_ATAN_SERIES = r#"$$
\arctan(x) = x - \frac{x^3}{3} + \frac{x^5}{5} - \frac{x^7}{7} + \cdots $$
For values $ |x| > 1 $ it uses the identity:
$$ \arctan(x) = \frac{\pi}{2} - \arctan(\frac{1}{x}) $$"#;
pub(crate) FORMULA_SINH_SERIES = r#"$$ \sinh(x) = \frac{e^x - e^{-x}}{2} $$"#;
pub(crate) FORMULA_COSH_SERIES = r#"$$ \cosh(x) = \frac{e^x + e^{-x}}{2} $$"#;
pub(crate) FORMULA_TANH_SERIES = r#"$$ \tanh(x) = \frac{\sinh(x)}{\cosh(x)} $$"#;
pub(crate) FORMULA_ASINH_SERIES = r#"$$ \text{asinh}(x) = \ln(x + \sqrt{x^2 + 1}) $$"#;
pub(crate) FORMULA_ACOSH_SERIES = r#"$$ \text{acosh}(x) = \ln(x + \sqrt{x^2 - 1}) $$"#;
pub(crate) FORMULA_ATANH_SERIES = r#"$$
\text{atanh}(x) = \frac{1}{2} \ln\left(\frac{1 + x}{1 - x}\right) $$"#;

/* Calculus */

pub(crate) FORMULA_DERIVATIVE = r#"$$ f'(x) \approx \frac{f(x + h) - f(x)}{h} $$"#;
pub(crate) FORMULA_INTEGRATE = r#"$$
\int_a^b f(x) \, dx \approx \sum_{i=0}^{n-1} f(x_i) \cdot \Delta x $$
where $$ \Delta x = \frac{b-a}{n} $$"#;
pub(crate) FORMULA_PARTIAL_DERIVATIVE_X = r#"
\frac{\partial f}{\partial x} \approx \frac{f(x + h, y) - f(x, y)}{h} $$"#;
pub(crate) FORMULA_PARTIAL_DERIVATIVE_Y = r#"
\frac{\partial f}{\partial x} \approx \frac{f(x + h, y) - f(x, y)}{h} $$"#;
}