#[macro_use]extern crate mime;

extern crate iron;
extern crate router;
extern crate urlencoded;

use router::Router;
use iron::prelude::*;
use iron::status;
mod gcd;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", gcd::post_gcd, "gcd");
    println!("Serving on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
       <title>GCD Calculator</title>
        <form action="/gcd" method="post">
          <input type="text" name="n"/>
          <input type="text" name="n"/>
          <button type="submit">Compute GCD</button>
        </form>
    "#);
    Ok(response)
}


