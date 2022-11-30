mod mutex;
mod rwlock;

pub use mutex::Mutex;
pub use rwlock::RwLock;
// TODO: barrier

pub use std::sync::atomic;
pub use std::sync::mpsc;
pub use std::sync::Arc;
// TODO
pub use std::sync::Condvar;
pub use std::sync::LockResult;
// TODO: wrap for long lock held?
pub use std::sync::MutexGuard;
pub use std::sync::Once;
pub use std::sync::OnceState;
pub use std::sync::PoisonError;
// TODO: wrap for long lock held?
pub use std::sync::RwLockReadGuard;
// TODO: wrap for long lock held?
pub use std::sync::RwLockWriteGuard;
pub use std::sync::TryLockError;
pub use std::sync::TryLockResult;
pub use std::sync::WaitTimeoutResult;
pub use std::sync::Weak;
