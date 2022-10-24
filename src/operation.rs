use strum::{EnumString, Display};

#[derive(Debug, Clone)]
pub struct Operation<B, O, A> {
    pub operation_type: OperationType,

    /// Closure to execute before timer is started.
    /// This closure returns a state that will be
    /// given muatbly to each iteration of `operation`
    /// and also to `after` which gets executed after
    /// timer is stopped.
    pub before: B,

    /// Maybe executed multiple times. Gets state returned by
    /// `before` as input, if any.
    /// These operations run after timer has been started.
    pub operation: O,

    /// Executed once after timer is stopped. Gets state returned
    /// by `before` as input, if any.
    pub after: A,
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum OperationType {
    Sign,
}
