pub trait UpdateFrom<T> {

    fn update_from(&mut self, other: T);
}