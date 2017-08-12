pub fn _map<T, F>(list: &[T], func: &F) -> Vec<T>
where
    T: Clone,
    F: Fn(T) -> T,
{
    let mut out = Vec::with_capacity(list.len());
    for item in list {
        out.push(func(item.clone()));
    }
    out
}

pub fn map_function<T, F>(list: Vec<T>, func: &F) -> Vec<T>
where
    T: Clone,
    F: Fn(T) -> T,
{
    _map(&list, func)
}

pub fn map_closure<T, F>(list: Vec<T>, func: F) -> Vec<T>
where
    T: Clone,
    F: Fn(T) -> T,
{
    _map(&list, &func)
}
