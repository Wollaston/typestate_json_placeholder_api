use std::{error::Request, isize, rc::Rc};

/// The basic object built using this API. It represents a custom request
/// to the JSON Placeholder API.
pub struct Request<S: RequestState> {
    custody: Vec<Rc<dyn RequestState>>,
    state: Rc<S>,
}

/// Enums for the various request options - standard request methods, resource endpoints, etc.

enum MethodType {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

enum CollectionType {
    Posts,
    Comments,
    Albums,
    Photos,
    Todos,
    Users,
}

enum QueryType {
    PostId,
}

/// Generic trait typestate.
trait RequestState {
    fn get_param(&self) -> String;
}

/// State Structs

struct Initialized;
struct Method(MethodType);
struct Resource(CollectionType);
struct Id(isize);
struct Relation(CollectionType);
struct Query(QueryType, isize);
struct Build;
struct Fetch;

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
        self.0.to_string()
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
impl RequestState for Fetch {
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
            custody,
            state: next,
        }
    }
}

/// The first state for the Request object, which includes the "new" creation function
/// and the "method" function for transitioning into the next state.
impl Request<Initialized> {
    pub fn new() -> Request<Initialized> {
        Request {
            custody: vec![Rc::new(Initialized)],
            state: Rc::new(Initialized),
        }
    }

    /// The transition function for the Request<Initialized> state.
    /// Requires the method to be set using the enum variants defined in the
    /// MethodType enum.
    ///
    /// Consumes "self" (the Request object in the <Initialized> state)
    /// and returns a new object of Request<Method>.
    pub fn method(self, method: MethodType) -> Request<Method> {
        self.transition(Method(method))
    }
}

impl Request<Method> {
    /// The transition function for the Request<Method> state.
    /// Requires the resource to be set using the enum variants
    /// defined in the CollectionType enum.
    ///
    /// Consumes "self" (the request object in the <Method> state)
    /// and returns a new object of Request<Resource>.
    pub fn resource(self, resource: CollectionType) {
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
        let my_iter = self
            .custody
            .iter()
            .enumerate()
            .map(|i, val| path.push_str(val.get_param().as_str()));
        path
    }
}
