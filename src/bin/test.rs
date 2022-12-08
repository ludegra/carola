use carola::handler::RequestHandler;

fn main() {
    let handler = RequestHandler::new();
    handler.listen(8080);
}