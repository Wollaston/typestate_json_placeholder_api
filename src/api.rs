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
struct Method;
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
