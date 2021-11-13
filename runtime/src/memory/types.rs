pub type MemResult<T> = Result<T, MemoryErrorType>;

#[derive(Debug, Eq, PartialEq)]
pub enum MemoryErrorType {
    LocationOutOfBounds,
    RegLocationNotAligned,
    LocationNotAligned,
    CannotWriteArrayToRegister,
    #[allow(dead_code)]
    FunctionalityNotImplemented
}
