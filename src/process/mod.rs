//! TODO(BSFishy): document this

use core::ptr::{addr_of, addr_of_mut};

/// TODO(BSFishy): document this
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TaskState {
    /// TODO(BSFishy): document this
    Embryo,
    /// TODO(BSFishy): document this
    Sleeping,
    /// TODO(BSFishy): document this
    Runnable,
    /// TODO(BSFishy): document this
    Running,
    /// TODO(BSFishy): document this
    Zombie,
}

/// TODO(BSFishy): document this
/// TODO: make this into an architecture independent structure
#[derive(Debug)]
#[repr(C)]
pub struct TaskContext {
    edi: u32,
    esi: u32,
    ebx: u32,
    ebp: u32,
    eip: u32,
}

extern "C" fn safe_switch_task<'a, 'b>(old: &'a TaskContext, new: &'b TaskContext) -> &'a TaskContext {
    let mut tmp: *const TaskContext = old;

    unsafe { switch_task(addr_of_mut!(tmp), new) };
    let ret: &'a TaskContext = unsafe { tmp.as_ref() }.unwrap();

    ret
}

#[naked]
unsafe extern "C" fn switch_task(old: *mut *const TaskContext, new: *const TaskContext) {
    asm!(
        "mov rax, [rsp+4]",
        "mov rdx, [rsp+8]",

        "push rbp",
        "push rbx",
        "push rsi",
        "push rdi",

        "mov [rax], rsp",
        "mov rsp, rdx",

        "pop rdi",
        "pop rsi",
        "pop rbx",
        "pop rbp",
        "ret",
        options(noreturn),
    );
}

/// TODO(BSFishy): document this
pub struct Task<'a> {
    pid: usize,
    state: TaskState,
    context: &'a TaskContext,
}

impl Task<'_> {
    /// TODO(BSFishy): document this
    pub fn suspend(&self) {
        todo!()
    }
}

/// TODO(BSFishy): document this
pub struct TaskManager {}

impl TaskManager {
    /// TODO(BSFishy): document this
    #[no_mangle]
    pub fn switch(&self, from: &mut Task, to: &Task) {
        todo!()
    }

    /// TODO(BSFishy): document this
    pub fn schedule(&self) {
        todo!()
    }
}
