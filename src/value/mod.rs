use crate::state::State;
use paste::paste;
use std::ops::Add;

mod cost;
pub use cost::SearchCost;
mod heuristic;
pub use heuristic::SearchHeuristic;

/// A wrapper around tuples to allow addition of costs
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddWrapper<T>(T);

/// allows us to generically add tuples of costs
macro_rules! impl_add_wrapper {
    // implications for tuples of size = N
    (: $($t:ident)+) => {paste! {
        impl<$($t),+> Add for AddWrapper<($($t),+)>
        where
            $($t: Add<Output = $t>),+
        {
            type Output = AddWrapper<($($t),+)>;
            fn add(self, rhs: Self) -> Self::Output {
                let Self(($([< lhs_ $t:lower >]),+)) = self;
                let Self(($([< rhs_ $t:lower >]),+)) = rhs;
                AddWrapper(($([< lhs_ $t:lower >] + [< rhs_ $t:lower >]),+))
            }
        }

        impl<S: State, $($t),+> SearchCost<S> for ($($t),+)
        where
            $($t: SearchCost<S>),+
        {
            type Cost = AddWrapper<($($t::Cost),+)>;
            fn cost(&self, action: &S::Action) -> Self::Cost {
                let ($([< cost_ $t:lower >]),+) = self;
                AddWrapper(($([< cost_ $t:lower >].cost(action)),+))
            }
        }

        impl<S: State, $($t),+> SearchHeuristic<S> for ($($t),+)
          where
            $($t: SearchHeuristic<S>),+ {
            type Cost = AddWrapper<($($t::Cost),+)>;
            fn value(&self, observed: &S::Observation) -> Self::Cost {
                let ($([< value_ $t:lower >]),+) = self;
                AddWrapper(($([< value_ $t:lower >].value(observed)),+))
            }
        }
    }};

    // tail recursion for all tuples of size <= N
    ($x:ident $($y:ident)+) => {
      impl_add_wrapper!($($y)+);
      impl_add_wrapper!(: $x $($y)+);
    };
    ($x:ident) => {};
}

impl_add_wrapper!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
