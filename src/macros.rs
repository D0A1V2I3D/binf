#[macro_use]

#[macro_export]
macro_rules! flag_new {
    ( $($f:expr),* ) => {
        {
            let mut tflag = crate::Flag::new();
            $(
                tflag.set_flag($f, true);
            )*
            tflag
        }
    }
}