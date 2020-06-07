mod other {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hello, world!"
    }
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, outside world!"
}

use other::world;

fn main() {
    // error[E0425]: cannot find value `static_rocket_route_info_for_world` in this scope
    rocket::ignite().mount("/hello", routes![hello, world]);
}