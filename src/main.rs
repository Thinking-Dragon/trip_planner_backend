use controller::{trip_controller::TripController, event_controller::EventController};
use service::{trip_service::TripService, event_service::EventService, user_service::UserService, authentication::authentication_service::AuthenticationService};
use route::trip_routes::{
    get_trips,
    get_trip,
    post_trip,
    put_trip,
};
use route::event_routes::{
    get_trip_events,
    get_trip_event,
    post_trip_event,
    put_trip_event,
    get_event_types,
};

mod model;
mod service;
mod controller;
mod presenter;
mod route;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let trip_service = TripService;
    let event_service = EventService;
    let user_service = UserService;
    let authentication_service = AuthenticationService::new(user_service);

    let trip_controller = TripController::new(trip_service);
    let event_controller = EventController::new(event_service);
    rocket::build()
        .manage(authentication_service)
        .manage(trip_controller)
        .manage(event_controller)
        .mount("/", routes![
            get_trips,
            get_trip,
            post_trip,
            put_trip,
            get_trip_events,
            get_trip_event,
            post_trip_event,
            put_trip_event,
            get_event_types,
        ])
}