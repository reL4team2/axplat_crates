#[def_plat_interface]
pub trait Sel4TaskIf {
    /// Switches to the given seL4 task.
    ///
    /// It returns the previous task's ID.
    fn switch_task(prev_task: usize, next_task: usize) -> usize;

    /// Creates a new seL4 task with the given parameters.
    ///
    /// It returns the created task's ID.
    fn create_task(
        tid: usize,
        entry_point: usize,
        stack_top: usize,
        priority: usize,
        cpu_id: usize,
    ) -> usize;

    /// Destroys the seL4 task with the given ID.
    fn destroy_task(task_id: usize);

    /// Migrates the seL4 task with the given ID to the target CPU.
    fn migrate_task(task_id: usize, target_cpu_id: usize);

    /// Starts the seL4 task with the given ID.
    fn start_task(task_id: usize);

    /// Stops the seL4 task with the given ID.
    fn stop_task(task_id: usize);

    /// Checks if the current task is the initial task.
    fn is_init_task() -> bool;
}
