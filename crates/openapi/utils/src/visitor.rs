pub mod visit;
pub mod visit_mut;
pub mod visit_ref;

pub mod type_visit;
pub mod type_visit_mut;
pub mod type_visit_ref;

pub mod reference_rewriter;

pub trait VisitorOutcome {
    fn new() -> Self;

    fn reduce(left: Self, right: Self) -> Self;
}

impl VisitorOutcome for () {
    fn new() -> Self {}

    fn reduce(_left: Self, _right: Self) -> Self {}
}
