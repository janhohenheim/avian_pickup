macro_rules! single {
    ($query:expr) => {
        match $query.get_single() {
            Ok(q) => q,
            _ => {
                return;
            }
        }
    };
}

pub(crate) use single;
