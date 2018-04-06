extern crate rocket;

use static_rocket_route_info_for_index;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn serverOKTest() {
    let testServer = rocket::ignite().mount("/", routes![index]);
    let testClient = Client::new(testServer).unwrap();
    let mut serverResponse = testClient.get("/").dispatch();
    let htmlFile = include_str!("../files/index.html").to_owned();
    assert_eq!(serverResponse.body_string(), Some(htmlFile));
    assert_eq!(serverResponse.status(), Status::Ok);
}