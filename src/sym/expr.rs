use crate::data::id::Id;

pub trait ExprSYM {
    type Repr<A>;

    fn lit(n: u32) -> Box<Self::Repr<u32>>;
    fn add(lhs: Box<Self::Repr<u32>>, rhs: Box<Self::Repr<u32>>) -> Box<Self::Repr<u32>>;
}

struct ExprSYMEval();

impl ExprSYM for ExprSYMEval {
    type Repr<A> = Id<A>;

    fn lit(n: u32) -> Box<Self::Repr<u32>> {
        Box::new(Id(n))
    }

    fn add(lhs: Box<Self::Repr<u32>>, rhs: Box<Self::Repr<u32>>) -> Box<Self::Repr<u32>> {
        Box::new(Id(lhs.0 + rhs.0))
    }
}

#[test]
fn eval_test() {
    let actual =
        ExprSYMEval::add(
            ExprSYMEval::lit(5),
            ExprSYMEval::add(
                ExprSYMEval::lit(2),
                ExprSYMEval::lit(3)
            )
        );
    assert_eq!(actual.0, 10)
}

struct ExprSYMView();

impl ExprSYM for ExprSYMView {
    type Repr<A> = Id<String>;

    fn lit(n: u32) -> Box<Self::Repr<u32>> {
        Box::new(Id( n.to_string()))
    }

    fn add(lhs: Box<Self::Repr<u32>>, rhs: Box<Self::Repr<u32>>) -> Box<Self::Repr<u32>> {
        Box::new(Id(format!("{} + {}", lhs.0, rhs.0)))
    }
}

#[test]
fn view_test() {
    let actual =
        ExprSYMView::add(
            ExprSYMView::lit(5),
            ExprSYMView::add(
                ExprSYMView::lit(2),
                ExprSYMView::lit(3)
            )
        );
    assert_eq!(actual.0, "5 + 2 + 3")
}

