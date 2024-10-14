//! Types related to task management

use super::TaskContext;

// 在 config.rs 中
// 例如，定义系统调用的最大数量
const MAX_SYSCALL_NUM: usize = 500;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in its lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The count of system calls made by this task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// Uninitialized state
    UnInit,
    /// Ready to run
    Ready,
    /// Currently running
    Running,
    /// Exited state
    Exited,
}
