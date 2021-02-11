#[macro_export]
macro_rules! cb {
    (($($arg:ident),*) $closure:expr) => {{
        let closure = enclose::enc!(($($arg),*) $closure);

        // return the closure using the cloned Rc
        yew::Callback::from(closure)
    }};
}
