extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut ticket_system = AutoTaskTicketCreate::new();
    ticket_system.create_ticket("Task 1", "Description of Task 1");
    ticket_system.create_ticket("Task 2", "Description of Task 2");
    ticket_system.assign_ticket(0, "Engineer A");
    ticket_system.update_ticket_status(0, "In Progress");
    let status = ticket_system.get_ticket_status(0);
    println!("Ticket Status: {}", status);
}

pub struct AutoTaskTicketCreate {
    tickets: Vec<Ticket>,
}

impl AutoTaskTicketCreate {
    pub fn new() -> Self {
        AutoTaskTicketCreate {
            tickets: Vec::new(),
        }
    }

    pub fn create_ticket(&mut self, title: &str, description: &str) {
        let ticket = Ticket {
            id: self.tickets.len() as u32,
            title: String::from(title),
            description: String::from(description),
            assigned_to: None,
            status: String::from("Open"),
        };
        self.tickets.push(ticket);
    }

    pub fn assign_ticket(&mut self, ticket_id: usize, engineer: &str) {
        if let Some(ticket) = self.tickets.get_mut(ticket_id) {
            ticket.assigned_to = Some(String::from(engineer));
        }
    }

    pub fn update_ticket_status(&mut self, ticket_id: usize, status: &str) {
        if let Some(ticket) = self.tickets.get_mut(ticket_id) {
            ticket.status = String::from(status);
        }
    }

    pub fn get_ticket_status(&self, ticket_id: usize) -> Option<&String> {
        self.tickets.get(ticket_id).map(|ticket| &ticket.status)
    }

    pub fn list_tickets(&self) -> Vec<String> {
        self.tickets
            .iter()
            .map(|ticket| format!("ID: {}, Title: {}", ticket.id, ticket.title))
            .collect()
    }
}

struct Ticket {
    id: u32,
    title: String,
    description: String,
    assigned_to: Option<String>,
    status: String,
}
