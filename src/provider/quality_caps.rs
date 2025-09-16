pub struct QualityCaps {
    pub max_multipv: usize,
}

impl Default for QualityCaps {
    fn default() -> Self {
        Self { max_multipv: 10 }
    }
}
