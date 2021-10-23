#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Keep greeting to world forever."
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await;
}
