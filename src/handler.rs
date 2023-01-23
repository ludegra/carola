use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, collections::HashMap, path::PathBuf, error::Error, fmt::Debug, fs, sync::{Arc, Mutex},
};

use crate::{http::{self, HTTPResponse, HTTPStatusCode, HTTPMethod, HTTPRequest}, thread_pool::{self, ThreadPool}, file::get_supported_filetypes};

use self::listener::RequestListener;

mod listener;

/// # RequestHandler
/// 
/// The base entry class for the carola library
/// 
/// This class is used to listen to and handle http requests
/// 
/// ## Example
/// 
/// ```rs
/// use carola::handler::RequestHandler;
/// use carola::http::{HTTPMethod, HTTPRequest, HTTPResponse};
/// 
/// let mut handler = RequestHandler::new();
/// handler.set_listener("GET", "/example", |request: HTTPRequest| {
///   HTTPResponse::new("1.1", HTTPStatusCode::OK, HashMap::new(), Some(String::from("Hello World!")))
/// });
/// handler.listen(8080).unwrap();
/// ```
pub struct RequestHandler {
    listeners: HashMap<(String, HTTPMethod), RequestListener<'static>>,
    public_folder: Option<PathBuf>,
    not_found_callback: Box<dyn FnMut(HTTPRequest) -> HTTPResponse + Send + 'static>
}

impl RequestHandler {
    /// # RequestHandler::new
    /// 
    /// Creates a new instance of a request handler with default configuration
    /// 
    /// ## Example
    /// 
    /// ```rs
    /// use carola::handler::RequestHandler;
    /// 
    /// let handler = RequestHandler::new();
    /// // Configure handler
    /// handler.listen(8080)?;
    /// ```
    pub fn new() -> Self {
        RequestHandler {
            listeners: HashMap::new(),
            public_folder: None,
            not_found_callback: Box::new(Self::default404)
        }
    }

    /// # RequestHandler::set_listener
    /// 
    /// Sets a listener for a specific path and http method
    /// 
    /// ## Arguments
    /// 
    /// * `method` - The http method to be used on this path
    /// * `path` - The path to listen on
    /// * `callback` - The callback to be called when a request is made to this path
    /// 
    /// ## Example
    /// 
    /// ```rs
    /// use carola::handler::RequestHandler;
    /// use carola::http::{HTTPMethod, HTTPRequest, HTTPResponse};
    /// 
    /// let mut handler = RequestHandler::new();
    /// handler.set_listener("GET", "/example", |request: HTTPRequest| {
    ///    HTTPResponse::new("1.1", HTTPStatusCode::OK, HashMap::new(), Some(String::from("Hello World!")))
    /// });
    /// ```
    pub fn set_listener<C: 'static + Send + FnMut(HTTPRequest) -> HTTPResponse>(&mut self, method: &str, path: &str, callback: C) {
        self.listeners.insert(
            (path.to_owned(), HTTPMethod::from(method)),
            RequestListener::new(
            path.to_string(),
            HTTPMethod::from(method),
            callback
        ));
    }

    /// # RequestHandler::set_public_folder
    /// 
    /// Sets the public folder for the server which will be used to serve static files.
    /// 
    /// Files in this folder automatically have a lower priority than expicitly set listeners.
    /// 
    /// ## Arguments
    /// 
    /// * `path` - The path to the public folder, either an absolute path or one that is realtive to the project root
    /// 
    /// ## Examples
    /// 
    /// ```rs
    /// use carola::handler::RequestHandler;
    /// 
    /// let mut handler = RequestHandler::new();
    /// handler.set_public_folder("./public"); // Will asign the public folder to the ./public folder
    /// ```
    pub fn set_public_folder(&mut self, path: &str) {
        let path = PathBuf::from(path);
        self.public_folder = Some(path);
    }

    /// # RequestHandler::set_not_found_callback
    /// 
    /// Sets the callback to be called when a request is made to a path that has no listener
    /// 
    /// ## Arguments
    /// 
    /// * `callback` - The callback to be called when a request is made to a path that has no listener
    /// 
    /// ## Example
    /// 
    /// ```rs
    /// use carola::handler::RequestHandler;
    /// use carola::http::{HTTPMethod, HTTPRequest, HTTPResponse};
    /// 
    /// let mut handler = RequestHandler::new();
    /// handler.set_not_found_callback(|request: HTTPRequest| {
    ///    HTTPResponse::new("1.1", HTTPStatusCode::NOT_FOUND, HashMap::new(), Some(String::from("404 Not Found!")))
    /// });
    /// ```
    pub fn set_not_found_callback<C: 'static + Send + FnMut(HTTPRequest) -> HTTPResponse>(&mut self, callback: C) {
        self.not_found_callback = Box::new(callback);
    }

    /// # RequestHandler::listen
    /// 
    /// Activates the server and starts listening for requests. This method will take control of the current thread and will not return until the server is stopped.
    /// 
    /// ## Arguments
    /// 
    /// * `port` - The port to listen on
    /// 
    /// ## Example
    /// 
    /// ```rs
    /// use carola::handler::RequestHandler;
    /// 
    /// let mut handler = RequestHandler::new();
    /// // -- Configure handler --
    /// handler.listen(8080).unwrap();
    /// ```
    pub fn listen(mut self, port: usize) -> Result<(), Box<dyn Error>> {
        println!("{:?}", self);
        let listener = TcpListener::bind(&format!("127.0.0.1:{}", port))?;
        let thread_pool = ThreadPool::new(16);

        let arc = Arc::new(Mutex::new(self));

        for stream in listener.incoming() {
            let arc = arc.clone();
            thread_pool.execute(move || {
                let mut handler = arc.lock().unwrap();
                let stream = stream.unwrap();
                handler.handle_request(stream);
            });
        }

        unreachable!()
    }

    fn handle_request(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request = http::HTTPRequest::parse(buf_reader.lines().map(|l| l.unwrap()));

        let response = match request {
            Ok(req) => self.match_request(req),
            Err(err) => {
                HTTPResponse::new("1.1", HTTPStatusCode::BadRequest, HashMap::new(), Some(err))
            }
        }.construct();
        println!("{}", response);

        stream.write_all(response.as_bytes()).unwrap();
    }

    fn match_request(&mut self, request: HTTPRequest) -> HTTPResponse {
        let path = request.get_uri().to_owned();
        let method = request.get_method().to_owned();

        // If there is a listener for this path, call it
        if let Some(listener) = self.listeners.get_mut(&(path, method)) {
            (listener.callback)(request)
        }
        // If there is a public folder and the file exists, serve it
        else if let Some(public_folder) = &self.public_folder {
            let path = public_folder.join(request.get_uri());
            if path.exists() {
                let mut headers = HashMap::new();

                // Define content type based on file extension
                let content_type = match path.extension() {
                    None => "text/plain",
                    Some(ext) => get_supported_filetypes().get(ext).unwrap_or(&"text/plain")
                };

                headers.insert("Content-Type".to_string(), content_type.to_string());

                // Read file contents
                let body = fs::read_to_string(path).unwrap();

                // Construct response
                HTTPResponse::new("1.1", HTTPStatusCode::OK, headers, Some(body))
            }
            else {
                (self.not_found_callback)(request)
            }
        }
        // If there is no listener and no public folder, return 404
        else {
            (self.not_found_callback)(request)
        }
    }

    fn default404(_: HTTPRequest) -> HTTPResponse {
        let body = include_str!("../assets/404.html").to_string();
        HTTPResponse::new("1.1", HTTPStatusCode::NotFound, HashMap::new(), Some(body))
    }
}

impl Debug for RequestHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestHandler")
            .field("listeners", &self.listeners)
            .field("public_folder", &self.public_folder)
            .finish()
    }
}