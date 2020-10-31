use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Input {
    a: bool,
    b: bool,
    c: bool,
    d: f64,
    e: i32,
    f: i32,
}

impl Input {
    pub fn from_str(s: &str) -> Option<Self> {
        // Use YAML parser to save development time
        serde_yaml::from_str(s).ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HValue {
    M,
    P,
    T,
}

#[derive(Debug, PartialEq)]
pub struct Output {
    h: HValue,
    k: f64,
}

// Implement Display trait and use .to_string() for serialization
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "H: {:?}\nK: {}\n", self.h, self.k)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Substitution {
    Base,
    Custom1,
    Custom2,
}

// Use OnceCell to store helper map for H calculations
#[allow(clippy::type_complexity)]
static HVALMAP: OnceCell<HashMap<Substitution, HashMap<HValue, (bool, bool, bool)>>> =
    OnceCell::new();

impl Substitution {
    // helper method to transform string to Substitution variant
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "base" => Some(Self::Base),
            "custom1" => Some(Self::Custom1),
            "custom2" => Some(Self::Custom2),
            _ => None,
        }
    }

    // This method transforms Input into Output according to current substitution variant
    pub fn get_output(&self, input: &Input) -> Option<Output> {
        let h = self.get_h_value(&input)?;
        let k = self.get_k_value(&input, h);

        Some(Output { h, k })
    }

    // Method for H calculations
    // To make it simplier H will be calculated by comparing tuples of (A, B, C)
    // I.e A && B && !C is equal to (A, B, C) == (true, true, false)
    fn get_h_value(&self, input: &Input) -> Option<HValue> {
        let map = HVALMAP.get_or_init(|| {
            let mut map = HashMap::new();
            let mut base_map = HashMap::new();
            base_map.insert(HValue::M, (true, true, false));
            base_map.insert(HValue::P, (true, true, true));
            base_map.insert(HValue::T, (false, true, true));

            // Override expressions for Custom2
            let mut custom2_map = base_map.clone();
            custom2_map.insert(HValue::T, (true, true, false));
            custom2_map.insert(HValue::M, (true, false, true));
            map.insert(Self::Custom2, custom2_map);

            map.insert(Self::Base, base_map);
            map
        });

        // Use Base if there is no overrides for current Substitution
        let subst_map = map.get(self).or_else(|| map.get(&Self::Base))?;

        for (h, expectation) in subst_map {
            if *expectation == (input.a, input.b, input.c) {
                return Some(*h);
            }
        }

        None
    }

    // Method for K calculations
    fn get_k_value(&self, input: &Input, value_h: HValue) -> f64 {
        let d = input.d;
        let e = input.e as f64;
        let f = input.f as f64;

        match (self, value_h) {
            // override expressions for Custom2
            (Self::Custom2, HValue::M) => f + d + d * e / 100.,
            // override expressions for Custom1
            (Self::Custom1, HValue::P) => 2. * d + d * e / 100.,
            // base expressions. It uses _ to match any Substitution
            (_, HValue::M) => d + d * e / 10.,
            (_, HValue::P) => d + d * (e - f) / 25.5,
            (_, HValue::T) => d - d * f / 30.,
        }
    }
}

#[cfg(test)]
mod test_types {
    use super::*;

    #[test]
    fn input_can_be_parsed() {
        let input_str = r"
A: true
B: true
C: false
D: 33.3
E: 10
F: 7
";
        let expected_input = Input {
            a: true,
            b: true,
            c: false,
            d: 33.3,
            e: 10,
            f: 7,
        };

        let input = Input::from_str(input_str);
        assert_eq!(input, Some(expected_input));
    }

    #[test]
    fn input_from_str_returns_none() {
        let input_str = "BAD STRING";

        let input = Input::from_str(input_str);
        assert_eq!(input, None);
    }

    #[test]
    fn output_can_be_serialized() {
        let output = Output {
            h: HValue::M,
            k: 33.33,
        };

        assert_eq!(output.to_string(), "H: M\nK: 33.33\n");
    }

    #[test]
    fn substitution_get_output_works() {
        let input = Input {
            a: true,
            b: true,
            c: false,
            d: 33.3,
            e: 10,
            f: 7,
        };
        let expexted_output = Output {
            h: HValue::M,
            k: 66.6,
        };

        let subst = Substitution::Base;
        let output_opt = subst.get_output(&input);
        assert_eq!(output_opt, Some(expexted_output));
    }

    #[test]
    fn substitution_get_output_returns_none() {
        let input = Input {
            a: true,
            b: false,
            c: true,
            d: 33.3,
            e: 10,
            f: 7,
        };

        let subst = Substitution::Base;
        let output_opt = subst.get_output(&input);
        assert_eq!(output_opt, None);
    }

    #[test]
    fn get_h_value_works() {
        let input = Input {
            a: true,
            b: true,
            c: false,
            d: 30.,
            e: 10,
            f: 7,
        };

        let h_opt = Substitution::Base.get_h_value(&input);
        assert_eq!(h_opt, Some(HValue::M));
    }

    #[test]
    fn get_h_value_returns_none() {
        let input = Input {
            a: true,
            b: false,
            c: true,
            d: 30.,
            e: 10,
            f: 7,
        };

        let h_opt = Substitution::Base.get_h_value(&input);
        assert_eq!(h_opt, None);
    }

    #[test]
    fn get_h_value_uses_overrides() {
        let input = Input {
            a: true,
            b: true,
            c: false,
            d: 30.,
            e: 10,
            f: 7,
        };

        let h_for_base = Substitution::Base.get_h_value(&input);
        let h_for_custom = Substitution::Custom2.get_h_value(&input);
        assert_eq!(h_for_base, Some(HValue::M));
        assert_eq!(h_for_custom, Some(HValue::T));
    }

    #[test]
    fn get_k_value_works() {
        let input = Input {
            a: true,
            b: true,
            c: false,
            d: 30.,
            e: 10,
            f: 7,
        };

        let k = Substitution::Base.get_k_value(&input, HValue::M);
        assert_eq!(k, 60.);
    }

    #[test]
    fn get_k_value_uses_overrides() {
        let input = Input {
            a: true,
            b: true,
            c: false,
            d: 30.,
            e: 10,
            f: 7,
        };

        let k_for_base = Substitution::Base.get_k_value(&input, HValue::M);
        let k_for_custom = Substitution::Custom2.get_k_value(&input, HValue::M);
        assert_eq!(k_for_base, 60.);
        assert_eq!(k_for_custom, 40.);
    }
}
