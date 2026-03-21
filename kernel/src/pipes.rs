extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::VecDeque;

pub struct Pipe {
    buffer: VecDeque<u8>,
    max_size: usize,
    closed: bool,
}

impl Pipe {
    pub fn new(max_size: usize) -> Self {
        Pipe {
            buffer: VecDeque::new(),
            max_size,
            closed: false,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        if self.closed || self.is_full() {
            return 0;
        }

        let mut bytes_written = 0;
        for &byte in data.iter() {
            if self.buffer.len() >= self.max_size { break; }
            self.buffer.push_back(byte);
            bytes_written += 1;
        }
        bytes_written
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        if self.closed && self.is_empty() {
            return 0;
        }

        let mut bytes_read = 0;
        for byte in buf.iter_mut() {
            if self.buffer.is_empty() { break; }
            *byte = self.buffer.pop_front().unwrap();
            bytes_read += 1;
        }
        bytes_read
    }

    pub fn available(&self) -> usize {
        self.max_size - self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.max_size
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}

pub struct PipeManager {
    pipes: Vec<(u32, Pipe)>,
    next_id: u32,
}

impl PipeManager {
    pub fn new() -> Self {
        PipeManager {
            pipes: Vec::new(),
            next_id: 0,
        }
    }

    pub fn create_pipe(&mut self, max_size: usize) -> u32 {
        let id = self.next_id;
        self.pipes.push((id, Pipe::new(max_size)));
        self.next_id += 1;
        id
    }

    pub fn close_pipe(&mut self, id: u32) {
        if let Some(pipe) = self.pipes.iter_mut().find(|(pid, _)| *pid == id).map(|(_, pipe)| pipe) {
            pipe.close();
        }
    }

    pub fn write_pipe(&mut self, id: u32, data: &[u8]) -> Option<usize> {
        if let Some(pipe) = self.pipes.iter_mut().find(|(pid, _)| *pid == id).map(|(_, pipe)| pipe) {
            Some(pipe.write(data))
        } else {
            None
        }
    }

    pub fn read_pipe(&mut self, id: u32, buf: &mut [u8]) -> Option<usize> {
        if let Some(pipe) = self.pipes.iter_mut().find(|(pid, _)| *pid == id).map(|(_, pipe)| pipe) {
            Some(pipe.read(buf))
        } else {
            None
        }
    }

    pub fn pipe_available(&self, id: u32) -> Option<usize> {
        if let Some(pipe) = self.pipes.iter().find(|(pid, _)| *pid == id).map(|(_, pipe)| pipe) {
            Some(pipe.available())
        } else {
            None
        }
    }
}