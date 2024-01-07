use anyhow::Result;
use core::fmt;
use std::{isize, rc::Rc};

/// The basic object built using this API. It represents a custom request
/// to the JSON Placeholder API.
pub struct Request<S: RequestState> {
    method: MethodType,
    custody: Vec<Rc<dyn RequestState>>,
    state: Rc<S>,
}

/// Enums for the various request options - standard request methods, resource endpoints, etc.

pub enum MethodType {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl fmt::Display for MethodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodType::Get => write!(f, "GET"),
            MethodType::Post => write!(f, "POST"),
            MethodType::Put => write!(f, "PUT"),
            MethodType::Patch => write!(f, "PATCH"),
            MethodType::Delete => write!(f, "DELETE"),
        }
    }
}

pub enum CollectionType {
    Posts,
    Comments,
    Albums,
    Photos,
    Todos,
    Users,
}

impl fmt::Display for CollectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionType::Posts => write!(f, "/posts"),
            CollectionType::Comments => write!(f, "/comments"),
            CollectionType::Albums => write!(f, "/albums"),
            CollectionType::Photos => write!(f, "/photos"),
            CollectionType::Todos => write!(f, "/todos"),
            CollectionType::Users => write!(f, "/users"),
        }
    }
}

pub enum QueryType {
    PostId,
}

impl fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryType::PostId => write!(f, "postId"),
        }
    }
}

/// Generic trait typestate.
pub trait RequestState {
    fn get_param(&self) -> String;
}

/// State Structs

pub struct Initialized;
pub struct Method(MethodType);
pub struct Resource(CollectionType);
pub struct Id(isize);
pub struct Relation(CollectionType);
pub struct Query(QueryType, isize);
pub struct Build(String);

/// Implement RequestState and the get_param() function
/// for each state struct.

impl RequestState for Initialized {
    fn get_param(&self) -> String {
        String::new()
    }
}

impl RequestState for Method {
    fn get_param(&self) -> String {
        self.0.to_string()
    }
}

impl RequestState for Resource {
    fn get_param(&self) -> String {
        self.0.to_string()
    }
}

impl RequestState for Id {
    fn get_param(&self) -> String {
        format!("/{}", self.0)
    }
}

impl RequestState for Relation {
    fn get_param(&self) -> String {
        self.0.to_string()
    }
}

impl RequestState for Query {
    fn get_param(&self) -> String {
        format!("?{}={}", self.0, self.1)
    }
}

impl RequestState for Build {
    fn get_param(&self) -> String {
        String::new()
    }
}

/// Generic impl block, using "S" to represent the current RequestState.
impl<S: RequestState> Request<S> {
    fn transition<N: RequestState + 'static>(self, next: N) -> Request<N> {
        let mut custody = self.custody;
        let next = Rc::new(next);
        custody.push(next.clone());

        Request {
            method: self.method,
            custody,
            state: next,
        }
    }
}

impl Default for Request<Initialized> {
    fn default() -> Self {
        Request::new(MethodType::Get)
    }
}

/// The first state for the Request object, which includes the "new" creation function
/// and the "method" function for transitioning into the next state.
impl Request<Initialized> {
    pub fn new(method: MethodType) -> Request<Initialized> {
        Request {
            method,
            custody: vec![Rc::new(Initialized)],
            state: Rc::new(Initialized),
        }
    }

    /// The transition function for the Request<Method> state.
    /// Requires the resource to be set using the enum variants
    /// defined in the CollectionType enum.
    ///
    /// Consumes "self" (the request object in the <Method> state)
    /// and returns a new object of Request<Resource>.
    pub fn resource(self, resource: CollectionType) -> Request<Resource> {
        self.transition(Resource(resource))
    }
}

impl Request<Resource> {
    /// One of two (the other being "query") transition functions for the
    /// Request<Resource> state. Due to the JSON Placeholder API constraints,
    /// (at least according to their guide online), a request can either contain
    /// an id or a query. Accordingly, the typestate branches here between
    /// those options.
    ///
    /// Requires the id to be set using an isize integer value.
    ///
    /// Consumes "self" (the request object in the <Resource> state)
    /// and returns a new object of Request<Id>.
    pub fn id(self, id: isize) -> Request<Id> {
        self.transition(Id(id))
    }

    /// One of two (the other being "id") transition functions for the
    /// Request<Resource> state. Due to the JSON Placeholder API constraints,
    /// (at least according to their guide online), a request can either contain
    /// an id or a query. Accordingly, the typestate branches here between
    /// those options.
    ///
    /// Requires the query to be set with:
    ///     - a varient from the QueryType enum
    ///     - an id of isize
    ///
    /// Consumes "self" (the request object in the <Resource> state)
    /// and returns a new object of Request<Query>.
    pub fn query(self, query: QueryType, id: isize) -> Request<Query> {
        self.transition(Query(query, id))
    }
}

impl Request<Id> {
    /// The transition function for the Request<Id> state.
    ///
    /// Requires the resource to be set using the enum variants
    /// defined in the CollectionType enum.
    ///
    /// Consumes "self" (the request object in the <Id> state)
    /// and returns a new object of Request<Relation>.
    pub fn relation(self, relation: CollectionType) -> Request<Relation> {
        self.transition(Relation(relation))
    }
}

impl Request<Relation> {
    /// The transition function for the Request<Relation> state.
    ///
    /// This function takes no parameters. Instead, it takes the values
    /// from prior states and builds the corresponding request URL.
    ///
    /// Consumes "self" (the request object in the <Relation> state)
    /// and returns a new object of Request<Build>.
    pub fn build(self) -> Request<Build> {
        let mut path = String::new();
        self.custody
            .iter()
            .for_each(|val| path.push_str(val.get_param().as_str()));
        self.transition(Build(format!(
            "https://jsonplaceholder.typicode.com{}",
            path
        )))
    }
}

impl Request<Build> {
    pub fn url(&self) -> String {
        self.state.0.clone()
    }

    /// The transition function for the Request<Build> state.
    ///
    /// Uses the request URL built in the prior state to call the
    /// JSON Placeholder API using reqwest in a blocking manner
    /// (as this is just for demonstration purposes - no need for async yet).
    ///
    /// Consumes "self" (the request object in the <Build> state)
    /// and returns the response as a Result<String>.
    pub fn fetch(self) -> Result<String> {
        let res = reqwest::blocking::get(self.url())?.text()?;
        Ok(res)
    }
}
