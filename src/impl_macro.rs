macro_rules! take_range_method {
    {
        $(#[$attr:ident])*
        fn take_range(&mut $self:ident, $range:ident: $Range:ty)
        $body:block
    } => {
        $(#[$attr])*
        fn take_range(&mut $self, $range: $Range) -> Self::Output
        $body
    };
    {
        $(#[$attr:ident])*
        fn remove_range(&mut $self:ident, $range:ident: $Range:ty)
        $body:block
    } => {
        $(#[$attr])*
        fn remove_range(&mut $self, $range: $Range)
        $body
    };
}

macro_rules! impl_take_range {
    {
        $(
            <$Range:ty> for $T:ty {
                $(
                    $(#[$attr:ident])*
                    fn $method:ident(&mut $self:ident, $range:ident)
                    $body:block
                )*
            }
        )*
    } => {
        $(
            impl $crate::TakeRange<$Range> for $T {
                type Output = $T;

                $(
                    take_range_method! {
                        $(#[$attr])*
                        fn $method(&mut $self, $range: $Range)
                        $body
                    }
                )*
            }
        )*
    };
}
