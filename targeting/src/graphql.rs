use crate::resolvers::well::{WellMutation, WellQuery};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub fn root_schema_builder() -> SchemaBuilder<RootQuery, RootMutation, EmptySubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription,
    )
}

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

#[derive(Debug, Clone, Default, MergedObject)]
pub struct RootQuery(WellQuery);

#[derive(Debug, Clone, Default, MergedObject)]
pub struct RootMutation(WellMutation);