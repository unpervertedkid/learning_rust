mod hasher{
    use crate::hash::hash::Hash;

    pub fn hash<T>(data: T) -> Hash<T>{
        return Hash {
            hash: String::from("Placeholder"),
            content: data,
        }
    }
}