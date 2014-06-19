use std::io::net::ip::{Port};

use http::server::{Request, ResponseWriter};
use http::method;

use router::Router;
use middleware::Middleware;
use server::Server;
use request;
use response;

///Floor is the application object. It's the surface that 
///holds all public APIs.

#[deriving(Clone)]
pub struct Floor{
    router: Router,
    middleware: Middleware,
    server: Option<Server>
}
impl Floor {

    /// In order to use Floors API one first has to create an instance.
    ///
    /// # Example
    /// ```rust
    /// let mut server = Floor::new();
    /// ```
    pub fn new() -> Floor {
        let router = Router::new();
        let middleware = Middleware::new();
        Floor {
            router: router,
            middleware: middleware,
            server: None,
        }
    }

    /// Registers a handler to be used for a specific GET request.
    /// Handlers are assigned to paths and paths are allowed to contain
    /// variables and wildcards.
    ///
    /// # Example without variables and wildcards
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) { 
    ///     response.send("This matches /user"); 
    /// };
    /// server.get("/user", handler);
    /// ```
    /// # Example with variables
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     let text = format!("This is user: {}", request.params.get(&"userid".to_string()));
    ///     response.send(text.as_slice());
    /// };
    /// server.get("/user/:userid", handler);
    /// ```
    /// # Example with simple wildcard
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches /user/list/4711 but not /user/extended/list/4711");  
    /// };
    /// server.get("/user/*/:userid", handler);
    /// ```
    /// # Example with double wildcard
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches /user/list/4711 and also /user/extended/list/4711");  
    /// };
    /// server.get("/user/**/:userid", handler);
    /// ```
    pub fn get(&mut self, uri: &str, handler: fn(request: &request::Request, response: &mut response::Response)){
        self.router.add_route(method::Get, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific POST request.
    /// 
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches a POST request to /a/post/request");  
    /// };
    /// server.post("/a/post/request", handler);
    /// ```
    /// Take a look at `get()` for a more detailed description.
    pub fn post(&mut self, uri: &str, handler: fn(request: &request::Request, response: &mut response::Response)){
        self.router.add_route(method::Post, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific PUT request.
    /// 
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches a POST request to /a/put/request");  
    /// };
    /// server.put("/a/put/request", handler);
    /// ```
    /// Take a look at `get(..)` for a more detailed description.
    pub fn put(&mut self, uri: &str, handler: fn(request: &request::Request, response: &mut response::Response)){
        self.router.add_route(method::Put, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific DELETE request.
    /// 
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches a DELETE request to /a/delete/request");  
    /// };
    /// server.delete("/a/delete/request", handler);
    /// ```
    /// Take a look at `get(...)` for a more detailed description.
    pub fn delete(&mut self, uri: &str, handler: fn(request: &request::Request, response: &mut response::Response)){
        self.router.add_route(method::Put, String::from_str(uri), handler);
    }

    /// Registers a middleware handler which will be invoked among other middleware
    /// handlers before each request. Middleware can be stacked and is invoked in the
    /// same order it was registered.
    ///
    /// A middleware handler is nearly identical to a regular route handler with the only
    /// difference that it expects a return value of boolean. That is to indicate whether
    /// other middleware handler (if any) further down the stack should continue or if the
    /// middleware invocation should be stopped after the current handler.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn logger (req: &Request, res: &mut Response) -> bool{
    ///     println!("logging request: {}", req.origin.request_uri);
    ///     true
    /// }
    /// ```
    pub fn utilize(&mut self, handler: fn(request: &request::Request, response: &mut response::Response) -> bool){
        self.middleware.add(handler);
    }

    /// Bind and listen for connections on the given host and port
    ///
    /// # Example
    /// ```rust
    /// let mut server = Floor::new();
    /// server.listen(6767);
    /// ```
    pub fn listen(mut self, port: Port) {
        self.server = Some(Server::new(self.router, self.middleware, port));
        self.server.unwrap().serve();
    }
}
