use std::{error::Request, rc::Rc};

/// The basic object to be built - a custom request to the JSON Placeholder API
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

/// Generic RequestState trait typestate
trait RequestState {}

/// State Structs
struct Initialized;
struct Method(MethodType);
struct Resource;
struct Id;
struct Relation;
struct Query;
struct Build;
struct Fetch;

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
