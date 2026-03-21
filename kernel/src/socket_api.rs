extern crate alloc;
use alloc::vec::Vec;

pub enum SocketType {
    Tcp,
    Udp,
    Raw,
}

pub enum SocketState {
    Closed,
    Listen,
    SynSent,
    Established,
    CloseWait,
    TimeWait,
}

pub struct Socket {
    pub id: u32,
    pub socket_type: SocketType,
    pub state: SocketState,
    pub local_ip: [u8; 4],
    pub local_port: u16,
    pub remote_ip: [u8; 4],
    pub remote_port: u16,
    pub rx_buffer: Vec<u8>,
    pub tx_buffer: Vec<u8>,
    pub rx_buffer_size: usize,
}

impl Socket {
    pub fn new_tcp(id: u32) -> Self {
        Self {
            id,
            socket_type: SocketType::Tcp,
            state: SocketState::Closed,
            local_ip: [0; 4],
            local_port: 0,
            remote_ip: [0; 4],
            remote_port: 0,
            rx_buffer: Vec::new(),
            tx_buffer: Vec::new(),
            rx_buffer_size: 65536,
        }
    }

    pub fn bind(&mut self, ip: [u8; 4], port: u16) {
        self.local_ip = ip;
        self.local_port = port;
    }

    pub fn listen(&mut self) {
        self.state = SocketState::Listen;
    }

    pub fn connect(&mut self, ip: [u8; 4], port: u16) {
        self.remote_ip = ip;
        self.remote_port = port;
        self.state = SocketState::SynSent;
    }

    pub fn send(&mut self, data: &[u8]) -> usize {
        self.tx_buffer.extend_from_slice(data);
        data.len()
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> usize {
        let n = buf.len().min(self.rx_buffer.len());
        for i in 0..n {
            buf[i] = self.rx_buffer[i];
        }
        self.rx_buffer.drain(..n);
        n
    }

    pub fn close(&mut self) {
        self.state = SocketState::Closed;
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.state, SocketState::Established)
    }
}

pub struct SocketTable {
    pub sockets: Vec<Socket>,
    pub next_id: u32,
}

impl SocketTable {
    pub fn new() -> Self {
        Self {
            sockets: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_tcp(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.sockets.push(Socket::new_tcp(id));
        id
    }

    pub fn get(&self, id: u32) -> Option<&Socket> {
        self.sockets.iter().find(|s| s.id == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Socket> {
        self.sockets.iter_mut().find(|s| s.id == id)
    }

    pub fn close_socket(&mut self, id: u32) {
        if let Some(s) = self.get_mut(id) {
            s.close();
        }
    }
}
