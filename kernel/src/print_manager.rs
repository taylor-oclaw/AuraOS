extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum PrinterType {
    Local,
    Network,
    Virtual,
}

pub enum PrintJobStatus {
    Queued,
    Printing(u8),
    Complete,
    Failed(String),
    Cancelled,
}

pub struct Printer {
    pub id: u64,
    pub name: String,
    pub printer_type: PrinterType,
    pub online: bool,
    pub paper_size: String,
    pub color: bool,
}

pub struct PrintJob {
    pub id: u64,
    pub printer_id: u64,
    pub document_name: String,
    pub pages: u32,
    pub copies: u8,
    pub status: PrintJobStatus,
    pub submitted_at: u64,
}

pub struct PrintManager {
    pub printers: Vec<Printer>,
    pub jobs: Vec<PrintJob>,
    pub next_printer_id: u64,
    pub next_job_id: u64,
}

impl PrintManager {
    pub fn new() -> Self {
        Self {
            printers: Vec::new(),
            jobs: Vec::new(),
            next_printer_id: 1,
            next_job_id: 1,
        }
    }

    pub fn add_printer(&mut self, name: &str, pt: PrinterType, color: bool) -> u64 {
        let id = self.next_printer_id;
        self.next_printer_id += 1;
        self.printers.push(Printer {
            id,
            name: String::from(name),
            printer_type: pt,
            online: true,
            paper_size: String::from("A4"),
            color,
        };
        id
    }

    pub fn print(&mut self, printer_id: u64, doc: &str, pages: u32, copies: u8) -> u64 {
        let id = self.next_job_id;
        self.next_job_id += 1;
        self.jobs.push(PrintJob {
            id,
            printer_id,
            document_name: String::from(doc),
            pages,
            copies,
            status: PrintJobStatus::Queued,
            submitted_at: 0,
        };
        id
    }

    pub fn cancel_job(&mut self, id: u64) {
        if let Some(j) = self.jobs.iter_mut().find(|j| j.id == id) {
            j.status = PrintJobStatus::Cancelled;
        }
    }

    pub fn pending_jobs(&self) -> usize {
        self.jobs
            .iter()
            .filter(|j| matches!(j.status, PrintJobStatus::Queued | PrintJobStatus::Printing(_)))
            .count()
    }
))}
