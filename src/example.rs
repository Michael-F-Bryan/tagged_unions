
/// Example
#[derive(Debug, Copy, Clone, TaggedUnion)]
pub enum Message {
    /// Stop the machine.
    Halt,
    /// The player moved.
    Move {
        x: f64,
        y: f64,
    },
    /// Wait for a number of milliseconds.
    Wait(u32),
}