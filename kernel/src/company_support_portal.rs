extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod company_support_portal {
    use super::*;

    pub struct SupportPortal {
        tickets: Vec<Ticket>,
    }

    impl SupportPortal {
        pub fn new() -> Self {
            SupportPortal { tickets: Vec::new() }
        }

        pub fn create_ticket(&mut self, title: String, description: String) -> usize {
            let ticket = Ticket {
                id: self.tickets.len(),
                title,
                description,
                status: TicketStatus::Open,
            };
            self.tickets.push(ticket);
            ticket.id
        }

        pub fn get_ticket(&self, id: usize) -> Option<&Ticket> {
            self.tickets.get(id)
        }

        pub fn close_ticket(&mut self, id: usize) -> bool {
            if let Some(ticket) = self.tickets.get_mut(id) {
                ticket.status = TicketStatus::Closed;
                true
            } else {
                false
            }
        }

        pub fn list_open_tickets(&self) -> Vec<&Ticket> {
            self.tickets.iter().filter(|t| t.status == TicketStatus::Open).collect()
        }

        pub fn search_tickets(&self, query: &str) -> Vec<&Ticket> {
            self.tickets
                .iter()
                .filter(|t| t.title.contains(query) || t.description.contains(query))
                .collect()
        }
    }

    #[derive(Debug)]
    struct Ticket {
        id: usize,
        title: String,
        description: String,
        status: TicketStatus,
    }

    #[derive(Debug, PartialEq)]
    enum TicketStatus {
        Open,
        Closed,
    }
}

#[cfg(test)]
mod tests {
    use super::company_support_portal::*;

    #[test]
    fn test_create_ticket() {
        let mut portal = SupportPortal::new();
        let id = portal.create_ticket(String::from("Test Title"), String::from("Test Description"));
        assert_eq!(id, 0);
        assert_eq!(portal.tickets.len(), 1);
    }

    #[test]
    fn test_get_ticket() {
        let mut portal = SupportPortal::new();
        let id = portal.create_ticket(String::from("Test Title"), String::from("Test Description"));
        let ticket = portal.get_ticket(id).unwrap();
        assert_eq!(ticket.title, "Test Title");
        assert_eq!(ticket.description, "Test Description");
    }

    #[test]
    fn test_close_ticket() {
        let mut portal = SupportPortal::new();
        let id = portal.create_ticket(String::from("Test Title"), String::from("Test Description"));
        assert!(portal.close_ticket(id));
        let ticket = portal.get_ticket(id).unwrap();
        assert_eq!(ticket.status, TicketStatus::Closed);
    }

    #[test]
    fn test_list_open_tickets() {
        let mut portal = SupportPortal::new();
        portal.create_ticket(String::from("Test Title 1"), String::from("Test Description 1"));
        portal.create_ticket(String::from("Test Title 2"), String::from("Test Description 2"));
        let open_tickets = portal.list_open_tickets();
        assert_eq!(open_tickets.len(), 2);
    }

    #[test]
    fn test_search_tickets() {
        let mut portal = SupportPortal::new();
        portal.create_ticket(String::from("Test Title 1"), String::from("Test Description 1"));
        portal.create_ticket(String::from("Test Title 2"), String::from("Test Description 2"));
        let search_results = portal.search_tickets("Title 1");
        assert_eq!(search_results.len(), 1);
    }
}
