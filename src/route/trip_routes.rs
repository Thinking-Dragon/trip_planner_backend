use rocket::{State, http::CookieJar, serde::json::Json};

use crate::{controller::{trip_controller::TripController, error_messages::make_authentication_failed_error}, model::trip::Trip, service::authentication::authentication_service::{self, AuthenticationService}};

#[get("/trips")]
pub async fn get_trips(
    authentication_service: &State<AuthenticationService>,
    trip_controller: &State<TripController>,
    cookie_jar: &CookieJar<'_>
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    return trip_controller.get_trips(user_id.unwrap());
}

#[get("/trips/<trip_id>")]
pub async fn get_trip(
    authentication_service: &State<AuthenticationService>,
    trip_controller: &State<TripController>,
    trip_id: String,
    cookie_jar: &CookieJar<'_>
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    return trip_controller.get_trip(user_id.unwrap(), trip_id.clone());
}

#[post("/trips", format="json", data="<body>")]
pub async fn post_trip(
    authentication_service: &State<AuthenticationService>,
    trip_controller: &State<TripController>,
    cookie_jar: &CookieJar<'_>,
    body: Json<Trip>
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    let new_trip = body.0;
    return trip_controller.create_trip(user_id.unwrap(), new_trip);
}

#[put("/trips/<trip_id>", format="json", data="<body>")]
pub async fn put_trip(
    authentication_service: &State<AuthenticationService>,
    trip_controller: &State<TripController>,
    cookie_jar: &CookieJar<'_>,
    trip_id: String,
    body: Json<Trip>
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    let new_trip = body.0;
    return trip_controller.update_trip(user_id.unwrap(), trip_id, new_trip);
}