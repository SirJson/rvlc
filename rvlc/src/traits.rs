pub(crate) trait PointerAccess<T> {
    fn get_ptr(&self) -> T;
}
