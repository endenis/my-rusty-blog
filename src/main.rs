extern crate nickel;

use std::io::net::ip::Ipv4Addr;
use nickel::{Nickel, Request, Response, HttpRouter, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    let mut router = Nickel::router();

    fn index_page_handler (_request: &Request, response: &mut Response) { 
        response.send("Hello my-rusty-blog"); 
    }

    router.get("/", index_page_handler);

    server.utilize(router);
    server.utilize(StaticFilesHandler::new("public/"));
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
