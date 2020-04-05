use std::collections::HashMap;
use crate::{
    lang::{Expr, Type, TypeScheme, TypeSubstitution, Id, fresh},
    unify::unify
};

pub type Assumptions = HashMap<Id, TypeScheme>;

fn apply_substitution(A: &Assumptions, S: &TypeSubstitution) -> Assumptions {
    A.clone().into_iter()
        .chain(S.clone().into_iter()
            .map(|(x, ty)| (x, TypeScheme::Type(ty))))
        .collect()
}

pub fn algo_w(A: &Assumptions, e: &Expr) -> Option<(TypeSubstitution, Type)> {
    match e {
        Expr::Unit => Some((HashMap::new(), Type::Unit)),
        Expr::Var(x) => {
            let sigma = A.get(x).unwrap();
            Some((HashMap::new(), sigma.clone().instantiate_with_fresh_vars()))
        },
        Expr::App { fun, arg } => {
            let (S1, tau_fun) = algo_w(A, fun)?;
            let (S2, tau_arg) = algo_w(&apply_substitution(A, &S1), arg)?;
            let beta = Type::Var(fresh());
            let V = unify(
                &tau_fun.substitute_many(&S2), 
                &Type::Function{arg: Box::new(tau_arg), result: Box::new(beta.clone())})?;
            
            let tau = beta.substitute_many(&V);

            let mut S = S1;
            S.extend(S2.into_iter());
            S.extend(V.into_iter());

            Some((S, tau))
        },
        Expr::Lambda{param, body} => {
            let beta = Type::Var(fresh());
            let mut A = A.clone();
            A.insert(param.clone(), TypeScheme::Type(beta.clone()));
            let (S, tau_body) = algo_w(&A, body)?;
            let tau_func = Type::Function{
                arg: Box::new(beta.substitute_many(&S)),
                result: Box::new(tau_body)
            };
            Some((S, tau_func))
        }
        _ => panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn w_var() {
        let A: Assumptions = hashmap! {
            "x".to_string() => TypeScheme::Poly{
                var: "a".to_string(), 
                body: Box::new(TypeScheme::Type(Type::Var("a".to_string())))}
        };
        let e = Expr::Var("x".to_string());
        let (S, tau) = algo_w(&A, &e).unwrap();
        assert_eq!(tau, Type::Var("a1".to_string()));
        assert_eq!(S, hashmap!{});
    }

    #[test]
    fn w_app() {
        let A: Assumptions = hashmap! {
            "x".to_string() => TypeScheme::Type(Type::Function{arg:Box::new(Type::Var("a".to_string())), result:Box::new(Type::Unit)})
        };       

        let e = Expr::App{fun: Box::new(Expr::Var("x".to_string())), arg: Box::new(Expr::Unit)};
        let (S, tau) = algo_w(&A, &e).unwrap();

        assert_eq!(S["a"], Type::Unit);
        assert_eq!(tau, Type::Unit);
    }

    #[test]
    fn w_lam() {
        let A: Assumptions = hashmap!{};
        let e = Expr::Lambda {
            param: "x".to_string(),
            body: Box::new(Expr::Var("x".to_string()))
        };
        let (S, tau) = algo_w(&A, &e).unwrap();
        assert_eq!(S, hashmap!{});
        assert_eq!(tau, Type::Function{arg: Box::new(Type::Var("a1".to_string())), result: Box::new(Type::Var("a1".to_string()))});
    }
}
