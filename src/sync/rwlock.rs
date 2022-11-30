use crate::panic_if_within_tokio;

pub struct RwLock<T: ?Sized>(std::sync::RwLock<T>);

impl<T> RwLock<T> {
    pub fn new(t: T) -> Self {
        Self(std::sync::RwLock::new(t))
    }

    #[track_caller]
    pub fn read(
        &self,
    ) -> Result<
        std::sync::RwLockReadGuard<'_, T>,
        std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>,
    > {
        panic_if_within_tokio("read");
        self.0.read()
    }

    pub fn try_read(
        &self,
    ) -> Result<
        std::sync::RwLockReadGuard<'_, T>,
        std::sync::TryLockError<std::sync::RwLockReadGuard<'_, T>>,
    > {
        self.0.try_read()
    }

    #[track_caller]
    pub fn write(
        &self,
    ) -> Result<
        std::sync::RwLockWriteGuard<'_, T>,
        std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>,
    > {
        panic_if_within_tokio("write");
        self.0.write()
    }

    pub fn try_write(
        &self,
    ) -> Result<
        std::sync::RwLockWriteGuard<'_, T>,
        std::sync::TryLockError<std::sync::RwLockWriteGuard<'_, T>>,
    > {
        self.0.try_write()
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
    use super::RwLock;
    use std::sync::Arc;

    #[tokio::test]
    #[should_panic(expected = "write called within an async context")]
    async fn can_create_but_write_panics() {
        let rw = RwLock::new(());
        let _ = rw.write();
    }

    #[test]
    fn can_write_in_spawn_blocking() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();

        let m = Arc::new(RwLock::new(0));
        let jh = rt.spawn_blocking({
            let m = m.clone();
            move || {
                let mut g = m.write().unwrap();
                *g += 1;
            }
        });

        rt.block_on(async move {
            jh.await
                .expect("write within spawn blocking should had been fine")
        });

        assert_eq!(*m.read().unwrap(), 1);
    }

    #[test]
    fn can_write_in_block_in_place() {
        let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();

        let m = RwLock::new(0);

        rt.block_on(async {
            tokio::task::block_in_place(|| {
                let mut g = m.write().unwrap();
                *g += 1;
            });
        });
    }

    #[tokio::test]
    #[should_panic(expected = "read called within an async context")]
    async fn can_create_but_read_panics() {
        let rw = RwLock::new(());
        let _ = rw.read();
    }

    #[test]
    fn can_read_in_spawn_blocking() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();

        let m = Arc::new(RwLock::new(0));
        let jh = rt.spawn_blocking({
            let m = m.clone();
            move || {
                let g = m.read().unwrap();
                *g + 1
            }
        });

        let answer = rt.block_on(async move {
            jh.await
                .expect("read within spawn blocking should had been fine")
        });

        assert_eq!(answer, 1);
    }

    #[test]
    fn can_read_in_block_in_place() {
        let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();

        let m = RwLock::new(0);

        let val = rt.block_on(async {
            tokio::task::block_in_place(|| {
                let g = m.read().unwrap();
                *g
            })
        });

        assert_eq!(val, 0);
    }
}
