use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use crate::Status;
use crate::primitives::frozen::implements::RubF;
use crate::primitives::tax_frozen::implements::Nds22;

calculated_primitives!(
    RubC,
    Decimal,
    RubF,
    "Руб.");

math_rules!(Add, add, +, RubF, RubF => RubC);
math_rules!(Add, add, +, RubF, RubC => RubC);
math_rules!(Add, add, +, RubC, RubF => RubC);
math_rules!(Add, add, +, RubC, RubC => RubC);
math_rules!(Sub, sub, -, RubF, RubF => RubC);
math_rules!(Sub, sub, -, RubF, RubC => RubC);
math_rules!(Sub, sub, -, RubC, RubF => RubC);
math_rules!(Sub, sub, -, RubC, RubC => RubC);
math_rules!(Mul, mul, *, RubF, Nds22 => RubC);
math_rules!(Mul, mul, *, RubC, Nds22 => RubC);
math_rules!(Mul, mul, *, Nds22, RubF => RubC);
math_rules!(Mul, mul, *, Nds22, RubC => RubC);