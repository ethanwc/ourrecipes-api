use ourrecipes_api;
use rocket_lamb::RocketExt;

fn main() {
    ourrecipes_api::rocket().lambda().launch();
}
