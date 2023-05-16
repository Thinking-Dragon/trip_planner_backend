use crate::model::trip::Trip;

#[derive(Clone, Copy)]
pub struct TripService;

impl TripService {
    pub fn get_all_trips(self, user_id: String) -> Vec<Trip> {
        let trips: Vec<Trip> = Vec::new();
        return trips;
    }

    pub fn get_trip(self, trip_id: String) -> Trip {
        let trip: Trip = Trip { id: Some("".to_string()), name: "".to_string(), events: Vec::new() };
        return trip;
    }

    pub fn create_trip(self, trip: Trip) -> String {
        return "trip_id".to_string();
    }

    pub fn update_trip(self, trip_id: String, trip: Trip) {

    }

    pub fn check_ownership(self, user_id: String, trip_id: String) -> bool {
        return true;
    }
}