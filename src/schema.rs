use async_graphql::{EmptyMutation, EmptySubscription, FieldResult, Object, Schema, SimpleObject};

#[derive(SimpleObject)]
pub struct Test {
    id: String,
}

pub struct Query;
#[Object(extends)]
impl Query {
    async fn test(&self) -> FieldResult<Test> {
        let test = Test {
            id: "test".to_string(),
        };
        Ok(test)
    }
}

pub type ProjectSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> ProjectSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}
