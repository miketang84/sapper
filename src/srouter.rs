


pub type SRouter = HashMap<Method, Vec<(&str, Box<SHandler>)>>;


impl SRouter {
    /// Construct a new, empty `Router`.
    ///
    /// ```
    /// # use router::Router;
    /// let router = Router::new();
    /// ```
    pub fn new() -> Router {
        HashMap::new()
    }

    pub fn route<H, S>(&mut self, method: method::Method,
                       glob: S, handler: H) -> &mut Router
    where H: Handler, S: AsRef<str> {
        self.routers.entry(method).or_insert(Recognizer::new())
                    .add(glob.as_ref(), Box::new(handler));
        self
    }

    /// Like route, but specialized to the `Get` method.
    pub fn get<H: Handler, S: AsRef<str>>(&mut self, glob: S, handler: H) -> &mut Router {
        self.route(method::Get, glob, handler)
    }

    /// Like route, but specialized to the `Post` method.
    pub fn post<H: Handler, S: AsRef<str>>(&mut self, glob: S, handler: H) -> &mut Router {
        self.route(method::Post, glob, handler)
    }

    /// Like route, but specialized to the `Put` method.
    pub fn put<H: Handler, S: AsRef<str>>(&mut self, glob: S, handler: H) -> &mut Router {
        self.route(method::Put, glob, handler)
    }

    /// Like route, but specialized to the `Delete` method.
    pub fn delete<H: Handler, S: AsRef<str>>(&mut self, glob: S, handler: H) -> &mut Router {
        self.route(method::Delete, glob, handler)
    }

    /// Like route, but specialized to the `Head` method.
    pub fn head<H: Handler, S: AsRef<str>>(&mut self, glob: S, handler: H) -> &mut Router {
        self.route(method::Head, glob, handler)
    }

    /// Like route, but specialized to the `Patch` method.
    pub fn patch<H: Handler, S: AsRef<str>>(&mut self, glob: S, handler: H) -> &mut Router {
        self.route(method::Patch, glob, handler)
    }

    /// Like route, but specialized to the `Options` method.
    pub fn options<H: Handler, S: AsRef<str>>(&mut self, glob: S, handler: H) -> &mut Router {
        self.route(method::Options, glob, handler)
    }
}

