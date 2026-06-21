#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    Ok = 0,
    AccountNotFound = 1,
    WrongPassword = 2,
    AlreadyOnline = 3,
    ServerFull = 4,
    ClientVersionTooOld = 5,
    InternalError = 99,
}
