use std::collections::HashMap;

type QueryParams = Option<HashMap<String, String>>;
type SearchParams = Option<HashMap<String, String>>;

pub struct Request<T> {
    path: String,
    body: Option<T>,
    query: QueryParams,
    params: SearchParams,
}
