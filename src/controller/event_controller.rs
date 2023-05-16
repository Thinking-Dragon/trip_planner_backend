use crate::{service::event_service::EventService, presenter::{event_presenter, event_type_presenter}, model::{event::Event, user::User}};

#[derive(Clone, Copy)]
pub struct EventController {
    event_service: EventService,
}

impl EventController {
    pub fn new(
        event_service: EventService
    ) -> EventController {
        return EventController { event_service };
    }

    pub fn get_events(self, user_id: String, trip_id: String) -> String {
        let events = self.event_service.get_trip_events(trip_id);
        return event_presenter::present_vec(events);
    }

    pub fn get_event(self, user_id: String, trip_id: String, event_id: String) -> String {
        let event = self.event_service.get_trip_event(trip_id, event_id);
        return event_presenter::present(event);
    }

    pub fn get_event_types(self) -> String {
        let event_types = self.event_service.get_event_types();
        return event_type_presenter::present_vec(event_types);
    }

    pub fn create_event(self, user_id: String, trip_id: String, event: Event) -> String {
        self.event_service.create_trip_event(trip_id, event.clone());
        return event_presenter::present(event);
    }

    pub fn update_event(self, user_id: String, trip_id: String, event: Event) -> String {
        self.event_service.update_trip_event(trip_id, event.clone());
        return event_presenter::present(event);
    }
}