/// Which role to use at a node.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Decision {
    Quality,
    Popularity,
    Hybrid,
}
