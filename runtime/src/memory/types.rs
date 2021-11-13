pub type MemResult<T> = Result<T, MemoryErrorType>;

#[derive(Debug)]
pub enum MemoryErrorType {
    LocationOutOfBounds,
    RegLocationNotAligned,
    LocationNotAligned,
    CannotWriteArrayToRegister,
    #[allow(dead_code)]
    FunctionalityNotImplemented
}
