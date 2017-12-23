use {InvalidTag, TaggedUnion};

/// Example
#[derive(Debug, Copy, Clone, TaggedUnion)]
pub enum Message {
    /// Stop the machine.
    Halt,
    /// The player moved.
    Translate(Point),
    /// Wait for a number of milliseconds.
    Wait(u32),
}

// Everything below is what I'd *expect* the custom derive to generate

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}
