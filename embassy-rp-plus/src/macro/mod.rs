#[macro_export]
macro_rules! impl_from {
    ($t:ident::$val:ident($pin:ty),$doc:expr) => {
        #[doc = concat!("support ",stringify!($doc),".into()")]
        impl<'d> From<Peri<'d, $pin>> for $t<'d> {
            #[inline]
            fn from(value: Peri<'d, $pin>) -> Self {
                Self::$val(value)
            }
        }
    };
}