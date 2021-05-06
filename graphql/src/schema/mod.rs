use async_graphql::*;

mod client;
mod converter;
mod mutation;
mod object;
mod query;

use mutation::*;
use object::*;
use query::*;

pub type BlogSchema = Schema<Query, Mutation, EmptySubscription>;
pub fn schema() -> BlogSchema {
    Schema::build(Query, Mutation, EmptySubscription).finish()
}
