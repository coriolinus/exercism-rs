pub type Callback<'a, T> = Box<dyn 'a + FnMut(T)>;

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}
