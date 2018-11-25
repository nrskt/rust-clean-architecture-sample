pub trait ToDomain<T> {
    fn to_domain(self) -> Result<T, ()>;
}
