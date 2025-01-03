pub trait Proto {
    fn to_short_string(&self) -> String;

    fn parse(buf: &mut &[u8]) -> eyre::Result<Self>
    where
        Self: Sized;
}
