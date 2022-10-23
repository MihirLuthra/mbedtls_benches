use strum::{EnumString, Display};

#[derive(Debug, Clone)]
pub struct Operation<B, O, A> {
    pub operation_type: OperationType,

    /// Closure to execute before threads are spawned.
    /// This closure returns a state that will be
    /// given muatbly to each iteration of `operation`
    /// and also to `after` which gets executed after
    /// threads are done.
    pub before: B,

    /// Maybe executed multiple times. Gets state returned by
    /// `before` as input, if any.
    pub operation: O,

    /// Executed once after threads are done. Gets state returned
    /// by `before` as input, if any.
    pub after: A,
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum OperationType {
    Sign,
}
