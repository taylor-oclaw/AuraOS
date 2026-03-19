extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum Syscall {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Exit = 4,
    Fork = 5,
    Exec = 6,
    GetPid = 7,
    Sleep = 8,
    Mmap = 9
}

pub struct SyscallResult {
    pub value: i64,
    pub error: bool
}

pub struct SyscallHandler {
    pub call_count: u64
}

impl SyscallHandler {
    pub fn new() -> Self {
        SyscallHandler { call_count: 0 }
    }

    pub fn dispatch(&mut self, num: u64, arg1: u64, arg2: u64, arg3: u64) -> SyscallResult {
        self.call_count += 1;
        match num {
            0 => self.sys_read(arg1, arg2, arg3),
            1 => self.sys_write(arg1, arg2, arg3),
            4 => self.sys_exit(arg1),
            7 => self.sys_getpid(),
            _ => SyscallResult { value: -1, error: true }
        }
    }

    pub fn sys_write(&self, fd: u64, buf: u64, len: u64) -> SyscallResult {
        // Stub implementation
        SyscallResult { value: len as i64, error: false }
    }

    pub fn sys_read(&self, fd: u64, buf: u64, len: u64) -> SyscallResult {
        // Stub implementation
        SyscallResult { value: 0, error: false }
    }

    pub fn sys_exit(&self, code: u64) -> SyscallResult {
        // Stub implementation
        SyscallResult { value: code as i64, error: false }
    }

    pub fn sys_getpid(&self) -> SyscallResult {
        // Stub implementation
        SyscallResult { value: 1, error: false }
    }
}
