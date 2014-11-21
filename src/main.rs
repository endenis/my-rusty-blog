extern crate nickel;

use std::io::net::ip::Ipv4Addr;
use nickel::{Nickel, Request, Response, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    
    fn index_page_handler (_request: &Request, response: &mut Response) { 
        response.send("Hello my-rusty-blog"); 
    }

    server.get("/", index_page_handler);
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
