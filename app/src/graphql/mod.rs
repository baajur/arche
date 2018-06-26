macro_rules! gq {
    ($x:expr, $y:expr) => {
        ge!($y.call(&$x.context()))
    };
}

macro_rules! ge {
    ($x:expr) => {
        match $x {
            Ok(v) => Ok(v),
            Err(e) => Err(juniper::FieldError::new(e, juniper::Value::Null)),
        }
    };
}

pub mod mutation;
pub mod query;
pub mod schema;

use std::net::SocketAddr;

use juniper_rocket::{graphiql_source, GraphQLRequest, GraphQLResponse};
use rocket::response::content::Html;
use rocket::{Route, State};

use nut::{graphql::context::Context, i18n::Locale, jwt::Token, orm::Connection as Database};

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/", routes!(doc, get))]
}

#[get("/doc")]
fn doc() -> Html<String> {
    graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get(
    db: Database,
    token: Token,
    locale: Locale,
    remote: SocketAddr,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    let Token(token) = token;
    request.execute(
        &schema,
        &Context {
            locale: locale.name,
            remote: remote,
            db: db,
            token: token,
        },
    )
}
