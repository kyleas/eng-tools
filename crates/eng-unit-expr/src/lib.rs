#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    pub m: i8,
    pub l: i8,
    pub t: i8,
    pub th: i8,
    pub n: i8,
}

impl Signature {
    pub const fn new(m: i8, l: i8, t: i8, th: i8, n: i8) -> Self {
        Self { m, l, t, th, n }
    }

    pub const fn dimless() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EvaluatedQuantity {
    pub value_si: f64,
    pub signature: Signature,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprError {
    Parse(String),
    UnknownUnit(String),
    AmbiguousUnit(String),
}

impl std::fmt::Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "{}", s),
            Self::UnknownUnit(s) => write!(f, "unknown unit '{}'", s),
            Self::AmbiguousUnit(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ExprError {}

#[derive(Clone, Copy, Debug)]
struct Qty {
    value: f64,
    sig: Signature,
}

fn sig_add(a: Signature, b: Signature, factor: i8) -> Signature {
    Signature::new(
        a.m + b.m * factor,
        a.l + b.l * factor,
        a.t + b.t * factor,
        a.th + b.th * factor,
        a.n + b.n * factor,
    )
}

fn sig_scale(s: Signature, p: i8) -> Signature {
    Signature::new(s.m * p, s.l * p, s.t * p, s.th * p, s.n * p)
}

#[derive(Clone, Copy, Debug)]
enum Tok<'a> {
    Number(f64),
    Ident(&'a str),
    Plus,
    Minus,
    Mul,
    Div,
    LPar,
    RPar,
    Caret,
}

struct Parser<'a> {
    tokens: Vec<Tok<'a>>,
    i: usize,
    src: &'a str,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            tokens: tokenize(src),
            i: 0,
            src,
        }
    }

    fn parse_expr(&mut self) -> Result<Qty, ExprError> {
        let mut lhs = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Tok::Plus) => {
                    self.next();
                    let rhs = self.parse_term()?;
                    if lhs.sig != rhs.sig {
                        return Err(ExprError::Parse(format!(
                            "cannot add/subtract differing dimensions in '{}': left {:?}, right {:?}",
                            self.src, lhs.sig, rhs.sig
                        )));
                    }
                    lhs.value += rhs.value;
                }
                Some(Tok::Minus) => {
                    self.next();
                    let rhs = self.parse_term()?;
                    if lhs.sig != rhs.sig {
                        return Err(ExprError::Parse(format!(
                            "cannot add/subtract differing dimensions in '{}': left {:?}, right {:?}",
                            self.src, lhs.sig, rhs.sig
                        )));
                    }
                    lhs.value -= rhs.value;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_term(&mut self) -> Result<Qty, ExprError> {
        let mut lhs = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(Tok::Mul) => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    lhs = Qty {
                        value: lhs.value * rhs.value,
                        sig: sig_add(lhs.sig, rhs.sig, 1),
                    };
                }
                Some(Tok::Div) => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    lhs = Qty {
                        value: lhs.value / rhs.value,
                        sig: sig_add(lhs.sig, rhs.sig, -1),
                    };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Qty, ExprError> {
        match self.peek() {
            Some(Tok::Plus) => {
                self.next();
                self.parse_unary()
            }
            Some(Tok::Minus) => {
                self.next();
                let mut v = self.parse_unary()?;
                v.value = -v.value;
                Ok(v)
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Qty, ExprError> {
        match self.next() {
            Some(Tok::Number(v)) => {
                let unit = self.parse_optional_unit_expr_after_number()?;
                match unit {
                    Some((factor, sig)) => Ok(Qty {
                        value: v * factor,
                        sig,
                    }),
                    None => Ok(Qty {
                        value: v,
                        sig: Signature::dimless(),
                    }),
                }
            }
            Some(Tok::LPar) => {
                let inner = self.parse_expr()?;
                match self.next() {
                    Some(Tok::RPar) => Ok(inner),
                    _ => Err(ExprError::Parse(format!("missing ')' in '{}'", self.src))),
                }
            }
            _ => Err(ExprError::Parse(format!(
                "invalid quantity expression '{}'",
                self.src
            ))),
        }
    }

    fn parse_optional_unit_expr_after_number(
        &mut self,
    ) -> Result<Option<(f64, Signature)>, ExprError> {
        let Some(next) = self.peek() else {
            return Ok(None);
        };
        if !matches!(next, Tok::Ident(_) | Tok::LPar) {
            return Ok(None);
        }
        let saved = self.i;
        match self.try_parse_unit_expr() {
            Ok(Some(v)) => Ok(Some(v)),
            Ok(None) => {
                self.i = saved;
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn try_parse_unit_expr(&mut self) -> Result<Option<(f64, Signature)>, ExprError> {
        let mut lhs = match self.try_parse_unit_factor()? {
            Some(v) => v,
            None => return Ok(None),
        };
        loop {
            let op = match self.peek() {
                Some(Tok::Mul) => 1,
                Some(Tok::Div) => -1,
                _ => break,
            };
            if !matches!(
                self.tokens.get(self.i + 1),
                Some(Tok::Ident(_)) | Some(Tok::LPar)
            ) {
                break;
            }
            self.next();
            let rhs = self.try_parse_unit_factor()?.ok_or_else(|| {
                ExprError::Parse(format!("expected unit factor in '{}'", self.src))
            })?;
            lhs = if op == 1 {
                (lhs.0 * rhs.0, sig_add(lhs.1, rhs.1, 1))
            } else {
                (lhs.0 / rhs.0, sig_add(lhs.1, rhs.1, -1))
            };
        }
        Ok(Some(lhs))
    }

    fn try_parse_unit_factor(&mut self) -> Result<Option<(f64, Signature)>, ExprError> {
        let mut base = match self.peek() {
            Some(Tok::Ident(id)) => {
                self.next();
                resolve_atom(id)?
            }
            Some(Tok::LPar) => {
                self.next();
                let inner = self.try_parse_unit_expr()?.ok_or_else(|| {
                    ExprError::Parse(format!("expected unit expression in '{}'", self.src))
                })?;
                match self.next() {
                    Some(Tok::RPar) => inner,
                    _ => return Err(ExprError::Parse(format!("missing ')' in '{}'", self.src))),
                }
            }
            _ => return Ok(None),
        };

        if matches!(self.peek(), Some(Tok::Caret)) {
            self.next();
            let sign = if matches!(self.peek(), Some(Tok::Minus)) {
                self.next();
                -1
            } else if matches!(self.peek(), Some(Tok::Plus)) {
                self.next();
                1
            } else {
                1
            };
            let p = match self.next() {
                Some(Tok::Number(n)) if n.fract().abs() < f64::EPSILON => (n as i8) * sign,
                _ => {
                    return Err(ExprError::Parse(format!(
                        "expected integer exponent in '{}'",
                        self.src
                    )));
                }
            };
            base = (base.0.powi(i32::from(p)), sig_scale(base.1, p));
        }

        Ok(Some(base))
    }

    fn peek(&self) -> Option<Tok<'a>> {
        self.tokens.get(self.i).copied()
    }

    fn next(&mut self) -> Option<Tok<'a>> {
        let tok = self.peek();
        if tok.is_some() {
            self.i += 1;
        }
        tok
    }
}

fn tokenize(input: &str) -> Vec<Tok<'_>> {
    let mut out = Vec::new();
    let b = input.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let c = b[i] as char;
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        match c {
            '+' => {
                out.push(Tok::Plus);
                i += 1;
            }
            '-' => {
                out.push(Tok::Minus);
                i += 1;
            }
            '*' => {
                out.push(Tok::Mul);
                i += 1;
            }
            '/' => {
                out.push(Tok::Div);
                i += 1;
            }
            '(' => {
                out.push(Tok::LPar);
                i += 1;
            }
            ')' => {
                out.push(Tok::RPar);
                i += 1;
            }
            '^' => {
                out.push(Tok::Caret);
                i += 1;
            }
            '0'..='9' | '.' => {
                let start = i;
                i += 1;
                while i < b.len() && ((b[i] as char).is_ascii_digit() || (b[i] as char) == '.') {
                    i += 1;
                }
                if i < b.len() && ((b[i] as char) == 'e' || (b[i] as char) == 'E') {
                    i += 1;
                    if i < b.len() && ((b[i] as char) == '+' || (b[i] as char) == '-') {
                        i += 1;
                    }
                    while i < b.len() && (b[i] as char).is_ascii_digit() {
                        i += 1;
                    }
                }
                if let Ok(v) = input[start..i].parse::<f64>() {
                    out.push(Tok::Number(v));
                }
            }
            _ => {
                let start = i;
                while i < b.len() {
                    let ch = b[i] as char;
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '%' {
                        i += 1;
                    } else {
                        break;
                    }
                }
                out.push(Tok::Ident(&input[start..i]));
            }
        }
    }
    out
}

fn resolve_atom(raw: &str) -> Result<(f64, Signature), ExprError> {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower == "c" || lower == "f" || lower == "degc" || lower == "degf" {
        return Err(ExprError::AmbiguousUnit(
            "affine temperatures are not supported in equation conversions; use Kelvin ('K')"
                .to_string(),
        ));
    }
    let atom = match lower.as_str() {
        "1" => (1.0, Signature::dimless()),
        "kg" => (1.0, Signature::new(1, 0, 0, 0, 0)),
        "g" => (1e-3, Signature::new(1, 0, 0, 0, 0)),
        "lbm" => (0.453_592_37, Signature::new(1, 0, 0, 0, 0)),
        "slug" => (14.593_902_937, Signature::new(1, 0, 0, 0, 0)),
        "m" => (1.0, Signature::new(0, 1, 0, 0, 0)),
        "cm" => (1e-2, Signature::new(0, 1, 0, 0, 0)),
        "mm" => (1e-3, Signature::new(0, 1, 0, 0, 0)),
        "ft" => (0.3048, Signature::new(0, 1, 0, 0, 0)),
        "in" => (0.0254, Signature::new(0, 1, 0, 0, 0)),
        "s" | "sec" => (1.0, Signature::new(0, 0, 1, 0, 0)),
        "min" => (60.0, Signature::new(0, 0, 1, 0, 0)),
        "hr" | "h" => (3600.0, Signature::new(0, 0, 1, 0, 0)),
        "k" | "kelvin" => (1.0, Signature::new(0, 0, 0, 1, 0)),
        "mol" => (1.0, Signature::new(0, 0, 0, 0, 1)),
        "n" => (1.0, Signature::new(1, 1, -2, 0, 0)),
        "j" => (1.0, Signature::new(1, 2, -2, 0, 0)),
        "w" => (1.0, Signature::new(1, 2, -3, 0, 0)),
        "pa" => (1.0, Signature::new(1, -1, -2, 0, 0)),
        "kpa" => (1e3, Signature::new(1, -1, -2, 0, 0)),
        "mpa" => (1e6, Signature::new(1, -1, -2, 0, 0)),
        "bar" => (1e5, Signature::new(1, -1, -2, 0, 0)),
        "psia" => (6_894.757_293_168, Signature::new(1, -1, -2, 0, 0)),
        "psi" => {
            return Err(ExprError::AmbiguousUnit(
                "ambiguous pressure unit 'psi'; use 'psia' (absolute) or 'psig' (gauge)"
                    .to_string(),
            ));
        }
        "lbf" => (4.448_221_615_260_5, Signature::new(1, 1, -2, 0, 0)),
        "kw" => (1e3, Signature::new(1, 2, -3, 0, 0)),
        "cp" => (1e-3, Signature::new(1, -1, -1, 0, 0)),
        "l" => (1e-3, Signature::new(0, 3, 0, 0, 0)),
        "gal" => (0.003_785_411_784, Signature::new(0, 3, 0, 0, 0)),
        _ => {
            if let Some((base, p)) = split_compact_power(trimmed)
                && let Ok(base_atom) = resolve_atom(base)
            {
                return Ok((base_atom.0.powi(i32::from(p)), sig_scale(base_atom.1, p)));
            }
            return Err(ExprError::UnknownUnit(trimmed.to_string()));
        }
    };
    Ok(atom)
}

fn split_compact_power(input: &str) -> Option<(&str, i8)> {
    let mut cut = input.len();
    for (i, ch) in input.char_indices().rev() {
        if ch.is_ascii_digit() || ch == '-' {
            cut = i;
        } else {
            break;
        }
    }
    if cut == input.len() || cut == 0 {
        return None;
    }
    let (base, exp_txt) = input.split_at(cut);
    if base.is_empty() {
        return None;
    }
    let p = exp_txt.parse::<i8>().ok()?;
    Some((base, p))
}

pub fn evaluate(src: &str) -> Result<EvaluatedQuantity, ExprError> {
    let mut p = Parser::new(src);
    let out = p.parse_expr()?;
    if p.peek().is_some() {
        return Err(ExprError::Parse(format!(
            "unexpected trailing token in '{}'",
            src
        )));
    }
    Ok(EvaluatedQuantity {
        value_si: out.value,
        signature: out.sig,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_pressure_expression() {
        let q = evaluate("5 MPa + 12 psia").unwrap();
        let expected = 5.0e6 + 12.0 * 6_894.757_293_168;
        assert!((q.value_si - expected).abs() < 1e-6);
    }

    #[test]
    fn rejects_ambiguous_psi() {
        let err = evaluate("12 psi").unwrap_err();
        assert!(err.to_string().contains("ambiguous pressure unit"));
    }
}
