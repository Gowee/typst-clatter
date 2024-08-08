macro_rules! parse_as_str {
    ($ident:ident) => {
        str::from_utf8($ident)
            .expect(concat!(stringify!($ident), " is valid utf-8"))
            .parse()
            .expect(concat!(stringify!($ident), " is valid"))
    };
}
pub(crate) use parse_as_str;
