use super::pid::Pid;
use crate::arch::paging::PageTable;
use spin::Spinlock;

/// Processes that are running on the system
pub static PROCESSES: Spinlock<Vec<Arc<Process>>> = Spinlock::new(Vec::new());

#[derive(Debug)]
pub struct Process {
    /// The identifier of the process
    pid: Pid,

    /// The virtual memory space of the process
    page_table: Spinlock<PageTable>,
}

impl Process {
    /// Create a new process
    #[must_use]
    pub fn new() -> Self {
        let pid = Pid::generate().expect("Failed to generate a new process ID");
        let page_table = Spinlock::new(PageTable::new());

        Self { pid, page_table }
    }

    /// Get the virtual memory space of the process
    #[must_use]
    pub const fn page_table(&self) -> &Spinlock<PageTable> {
        &self.page_table
    }

    /// Get the identifier of the process
    #[must_use]
    pub const fn pid(&self) -> Pid {
        self.pid
    }
}

impl Default for Process {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.pid.deallocate();
    }
}

/// Register a new process with the system
pub fn register(process: Process) {
    PROCESSES.lock().push(Arc::new(process));
}

/// Get a process by its identifier
pub fn get(pid: Pid) -> Option<Arc<Process>> {
    PROCESSES
        .lock()
        .iter()
        .find(|process| process.pid() == pid)
        .cloned()
}
