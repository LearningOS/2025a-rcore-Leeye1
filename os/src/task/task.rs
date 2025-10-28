//! Types related to task management

use super::TaskContext;
//MAKE ID is 1 more than ID and the 0 index is not used
pub const MAX_SYSCALL_ID:usize =411;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// the array of syscall count
    pub task_syscall_count: [usize;MAX_SYSCALL_ID],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}




