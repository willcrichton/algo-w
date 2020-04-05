
use std::collections::HashMap;
use crate::lang::{Type, TypeSubstitution};


fn disagreement<'a>(tau1: &'a Type, tau2: &'a Type) -> Option<(&'a Type, &'a Type)> {
    match (tau1, tau2) {
        (Type::Var(x1), Type::Var(x2)) => {
            if x1 == x2 { None }
            else { Some((tau1, tau2)) }
        },
        (Type::Unit, Type::Unit) => None,
        (Type::Function{arg:arg1, result:result1}, Type::Function{arg:arg2, result:result2}) => {
            match (disagreement(arg1, arg2), disagreement(result1, result2)) {
                (Some(d), _) => Some(d),
                (_, Some(d)) => Some(d),
                _ => None
            }
        },
        _ => Some((tau1, tau2))
    }
}

// Robinson's unification algorithm: https://en.wikipedia.org/wiki/Unification_(computer_science)#A_unification_algorithm
pub fn unify(tau1: &Type, tau2: &Type) -> Option<TypeSubstitution> {
    let mut S = HashMap::new();
    loop {
        let (tau1_sig, tau2_sig) = (tau1.substitute_many(&S), tau2.substitute_many(&S));
        if tau1_sig == tau2_sig {
            return Some(S);
        }

        let (s, t) = disagreement(&tau1_sig, &tau2_sig).unwrap();
        match (s, t) {
            (Type::Var(x), _) => {
                // TODO: if x occurs in t, return None
                S.insert(x.clone(), t.clone());
            },
            (_, Type::Var(x)) => {
                // TODO: if x occurs in s, return None
                S.insert(x.clone(), s.clone());

            }
            _ => { return None; }
        };
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn unify_unit() {
        let tau1 = Type::Unit;
        let tau2 = Type::Unit;
        assert_eq!(unify(&tau1, &tau2), Some(hashmap!{}));
    }


    #[test]
    fn unify_unit_var() {
        let tau1 = Type::Var("x".to_string());
        let tau2 = Type::Unit;
        assert_eq!(unify(&tau1, &tau2), Some(hashmap!{
            "x".to_string() => Type::Unit
        }));
    }
}
