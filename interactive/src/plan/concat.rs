//! Concat expression plan.

use std::hash::Hash;

use timely::dataflow::Scope;

use differential_dataflow::{Collection, Data};
use plan::{Plan, Render};
use {TraceManager, Time, Diff};

/// Merges the source collections.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Concat<V> {
    /// Plan for the data source.
    pub plans: Vec<Plan<V>>,
}

impl<V: Data+Hash> Render for Concat<V> {

    type Value = V;

    fn render<S: Scope<Timestamp = Time>>(
        &self,
        scope: &mut S,
        arrangements: &mut TraceManager<Self::Value>) -> Collection<S, Vec<Self::Value>, Diff>
    {
        use timely::dataflow::operators::Concatenate;
        use differential_dataflow::AsCollection;

        let collections =
        self.plans
            .iter()
            .map(|plan| plan.render(scope, arrangements).inner)
            .collect::<Vec<_>>();

        scope
            .concatenate(collections)
            .as_collection()
    }
}
