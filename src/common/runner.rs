pub(crate) trait Runner {
    fn init() -> Self;
    fn run(&self);
}