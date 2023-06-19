use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "Test object")]
struct Test {
    id: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    #[graphql(description = "Test query")]
    fn test() -> FieldResult<Test> {
        Ok(Test {
            id: "test".to_owned(),
        })
    }
}
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
