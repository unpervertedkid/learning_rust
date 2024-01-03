pub mod storage {
    use crate::hash::hash::Hash;

    pub trait Storage {
        fn get() -> Hash<String>;
        fn put() -> bool;
    }
}
