pub(crate) trait KucTestExpect<T> {
    #[track_caller]
    fn kuc_expect(self, message: &str) -> T;

    #[track_caller]
    fn kuc_unwrap(self) -> T;
}

impl<T> KucTestExpect<T> for Option<T> {
    #[track_caller]
    fn kuc_expect(self, message: &str) -> T {
        let Some(value) = self else {
            let present = false;
            assert!(present, "{message}");
            loop {
                std::thread::park();
            }
        };
        value
    }

    #[track_caller]
    fn kuc_unwrap(self) -> T {
        self.kuc_expect("expected Some value")
    }
}

impl<T, E> KucTestExpect<T> for Result<T, E> {
    #[track_caller]
    fn kuc_expect(self, message: &str) -> T {
        let Ok(value) = self else {
            let ok = false;
            assert!(ok, "{message}");
            loop {
                std::thread::park();
            }
        };
        value
    }

    #[track_caller]
    fn kuc_unwrap(self) -> T {
        self.kuc_expect("expected Ok value")
    }
}
