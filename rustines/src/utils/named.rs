pub trait Named {
    fn name(&self) -> &str;
}

#[macro_export]
macro_rules! impl_named {
    ($class_name:ty) => {
        impl Named for $class_name {
            fn name(&self) -> &str {
                stringify!($class_name)
            }
        }
    }
}
