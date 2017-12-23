use {InvalidTag, TaggedUnion};

/// Example
#[derive(Debug, Copy, Clone, TaggedUnion)]
pub enum Message {
    /// Stop the machine.
    Halt,
    /// The player moved.
    Move(Point),
    /// Wait for a number of milliseconds.
    Wait(u32),
}

// Everything below is what I'd *expect* the custom derive to generate

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

pub struct TaggedMessage {
    pub tag: u32,
    pub kind: MessageKind,
}

pub union MessageKind {
    pub empty: (),
    pub wait: u32,
    pub move_: Point,
}

pub const MESSAGE_HALT: u32 = 0;
pub const MESSAGE_MOVE: u32 = 1;
pub const MESSAGE_WAIT: u32 = 2;

impl TaggedUnion for Message {
    type Target = TaggedMessage;

    fn tag(&self) -> u32 {
        match *self {
            Message::Halt => MESSAGE_HALT,
            Message::Move(_) => MESSAGE_MOVE,
            Message::Wait(_) => MESSAGE_WAIT,
        }
    }

    fn as_tagged(&self) -> Self::Target {
        match *self {
            Message::Halt => TaggedMessage {
                tag: MESSAGE_HALT,
                kind: MessageKind { empty: () },
            },
            Message::Wait(n) => TaggedMessage {
                tag: MESSAGE_WAIT,
                kind: MessageKind { wait: n },
            },
            Message::Move(n) => TaggedMessage {
                tag: MESSAGE_MOVE,
                kind: MessageKind { move_: n },
            },
        }
    }

    unsafe fn from_tagged(tagged: &Self::Target) -> Result<Self, InvalidTag> {
        match tagged.tag {
            MESSAGE_HALT => Ok(Message::Halt),
            MESSAGE_WAIT => Ok(Message::Wait(tagged.kind.wait)),
            MESSAGE_MOVE => Ok(Message::Move(tagged.kind.move_)),
            _ => Err(InvalidTag {
                got: tagged.tag,
                possible_tags: 0..3,
            }),
        }
    }
}
