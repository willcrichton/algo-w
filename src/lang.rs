use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

pub type Id = String;

static FRESH_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub fn fresh() -> Id {
    let ctr = FRESH_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("a{}", ctr)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var(Id),
    App { fun: Box<Expr>, arg: Box<Expr> },
    Lambda { param: Id, body: Box<Expr> },
    Let { var: Id, value: Box<Expr>, body: Box<Expr> },
    Unit
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(Id),
    Unit,
    Function { arg: Box<Type>, result: Box<Type> }
}

pub type TypeSubstitution = HashMap<Id, Type>;

impl Type {
    pub fn substitute(&self, x: &Id, ty: &Type) -> Type {
        match self {
            Type::Var(x2) => if x == x2 { ty.clone() } else { self.clone() },
            Type::Unit => self.clone(),
            Type::Function { arg, result } => {
                Type::Function { 
                    arg: Box::new(arg.substitute(x, ty)),
                    result: Box::new(result.substitute(x, ty))
                }
            }
        }
    }

    pub fn substitute_many(&self, S: &TypeSubstitution) -> Type {
        S.iter().fold(self.clone(), |ty, (x, ty2)| { ty.substitute(x, ty2) })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeScheme {
    Type(Type),
    Poly { var: Id, body: Box<TypeScheme> }
}

impl TypeScheme {
    pub fn substitute(&self, x: &Id, ty: &Type) -> TypeScheme {
        match self {
            TypeScheme::Poly { var, body } => {
                if x == var { self.clone() }
                else { TypeScheme::Poly { var: var.clone(), body: Box::new(body.substitute(x, ty))} }
            },
            TypeScheme::Type(ty2) => TypeScheme::Type(ty2.substitute(x, ty))
        }
    }

   pub fn instantiate_with_fresh_vars(&self) -> Type {
        match self {
            TypeScheme::Poly { var, body } => {
                body
                    .substitute(var, &Type::Var(fresh()))
                    .instantiate_with_fresh_vars()
            },
            TypeScheme::Type (ty) => ty.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn substitute() {
        let sigma = TypeScheme::Type(Type::Var("a".to_string()));
        assert_eq!(sigma.substitute(&"a".to_string(), &Type::Unit), TypeScheme::Type(Type::Unit));
    }
}