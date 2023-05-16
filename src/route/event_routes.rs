use rocket::{State, http::CookieJar, serde::json::Json};

use crate::{controller::{event_controller::EventController, error_messages::make_authentication_failed_error}, service::authentication::authentication_service::AuthenticationService, model::event::Event};

#[get("/trips/<trip_id>/events")]
pub async fn get_trip_events(
    authentication_service: &State<AuthenticationService>,
    event_controller: &State<EventController>,
    cookie_jar: &CookieJar<'_>,
    trip_id: String
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    return event_controller.get_events(user_id.unwrap(), trip_id);
}

#[get("/trips/<trip_id>/events/<event_id>")]
pub async fn get_trip_event(
    authentication_service: &State<AuthenticationService>,
    event_controller: &State<EventController>,
    cookie_jar: &CookieJar<'_>,
    trip_id: String,
    event_id: String
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    return event_controller.get_event(user_id.unwrap(), trip_id, event_id);
}

#[post("/trips/<trip_id>/events", format="json", data="<body>")]
pub async fn post_trip_event(
    authentication_service: &State<AuthenticationService>,
    event_controller: &State<EventController>,
    cookie_jar: &CookieJar<'_>,
    trip_id: String,
    body: Json<Event>
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    let event = body.0;
    return event_controller.create_event(user_id.unwrap(), trip_id, event);
}

#[put("/trips/<trip_id>/events/<event_id>", format="json", data="<body>")]
pub async fn put_trip_event(
    authentication_service: &State<AuthenticationService>,
    event_controller: &State<EventController>,
    cookie_jar: &CookieJar<'_>,
    trip_id: String,
    event_id: String,
    body: Json<Event>
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    let event = body.0;
    return event_controller.update_event(user_id.unwrap(), trip_id, event);
}

#[get("/event_types")]
pub async fn get_event_types(
    authentication_service: &State<AuthenticationService>,
    event_controller: &State<EventController>,
    cookie_jar: &CookieJar<'_>
) -> String {
    let user_id = authentication_service.validate_authentication(cookie_jar).await;

    if user_id.is_none() {
        return make_authentication_failed_error();
    }

    return event_controller.get_event_types();
}