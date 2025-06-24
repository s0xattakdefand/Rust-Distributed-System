// TODO: src
//! Very small Boolean interpreter: variables & AND | OR.

use std::collections::HashMap;

#[derive(Clone)]
pub enum Expr {
    Var(String),
    And(Box<Expr>,Box<Expr>),
    Or(Box<Expr>,Box<Expr>)
}
impl Expr {
    pub fn eval(&self, ctx: &HashMap<String,bool>) -> bool {
        match self {
            Expr::Var(v) => ctx[v],
            Expr::And(a,b) => a.eval(ctx) && b.eval(ctx),
            Expr::Or(a,b)  => a.eval(ctx) || b.eval(ctx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn truth() {
        let e = Expr::And(Box::new(Expr::Var("x".into())), Box::new(Expr::Var("y".into())));
        let mut ctx = HashMap::new(); ctx.insert("x".into(), true); ctx.insert("y".into(), false);
        assert!(!e.eval(&ctx));
    }
}
