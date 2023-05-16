use crate::{service::trip_service::TripService, presenter::trip_presenter, model::trip::Trip};

use super::error_messages::make_trip_ownership_error;

#[derive(Clone, Copy)]
pub struct TripController {
    trip_service: TripService,
}

impl TripController {
    pub fn new(
        trip_service: TripService
    ) -> TripController {
        return TripController { trip_service };
    }

    pub fn get_trips(self, user_id: String) -> String {
        let trips = self.trip_service.get_all_trips(user_id);
        return trip_presenter::present_vec(trips);
    }

    pub fn get_trip(self, user_id: String, trip_id: String) -> String {
        let trip = self.trip_service.get_trip(trip_id);
        return trip_presenter::present(trip);
    }

    pub fn create_trip(self, user_id: String, trip: Trip) -> String {
        let trip_id = self.trip_service.create_trip(trip.clone());
        let added_trip = self.trip_service.get_trip(trip_id);

        return trip_presenter::present(added_trip);
    }

    pub fn update_trip(self, user_id: String, trip_id: String, trip: Trip) -> String {
        let user_owns_trip = self.trip_service.check_ownership(user_id.clone(), trip_id.clone());

        if user_owns_trip {
            self.trip_service.update_trip(trip_id, trip.clone());
            return trip_presenter::present(trip);
        }
        else {
            return make_trip_ownership_error(user_id, trip_id);
        }
    }
}