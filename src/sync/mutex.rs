use crate::panic_if_within_tokio;

#[derive(Default)]
pub struct Mutex<T: ?Sized>(std::sync::Mutex<T>);

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self(std::sync::Mutex::new(t))
    }

    #[track_caller]
    pub fn lock<'a>(
        &'a self,
    ) -> Result<std::sync::MutexGuard<'a, T>, std::sync::PoisonError<std::sync::MutexGuard<'a, T>>>
    {
        panic_if_within_tokio("lock");
        self.0.lock()
    }

    pub fn try_lock<'a>(
        &'a self,
    ) -> Result<std::sync::MutexGuard<'a, T>, std::sync::TryLockError<std::sync::MutexGuard<'a, T>>>
    {
        self.0.try_lock()
    }

    pub fn is_poisoned(&self) -> bool {
        self.0.is_poisoned()
    }

    pub fn into_inner(self) -> Result<T, std::sync::PoisonError<T>> {
        self.0.into_inner()
    }

    pub fn get_mut(&mut self) -> Result<&mut T, std::sync::PoisonError<&mut T>> {
        self.0.get_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::Mutex;
    use std::sync::Arc;

    #[tokio::test]
    #[should_panic(expected = "lock called within an async context")]
    async fn can_create_but_lock_panics() {
        let m = Mutex::new(());
        drop(m.lock());
    }

    #[test]
    fn can_lock_in_spawn_blocking() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();

        let m = Arc::new(Mutex::new(0));
        let jh = rt.spawn_blocking({
            let m = m.clone();
            move || {
                let mut g = m.lock().unwrap();
                *g += 1;
            }
        });

        rt.block_on(async move {
            jh.await
                .expect("lock within spawn blocking should had been fine")
        });

        assert_eq!(*m.lock().unwrap(), 1);
    }

    #[test]
    fn can_lock_in_block_in_place() {
        let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();

        let m = Mutex::new(0);

        rt.block_on(async {
            tokio::task::block_in_place(|| {
                let mut g = m.lock().unwrap();
                *g += 1;
            });
        });
    }
}
