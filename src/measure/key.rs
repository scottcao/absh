#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MeasureKey {
    WallTime,
    MaxRss,
    UserDefinedMetric,
}

impl MeasureKey {
    pub const ALL: &'static [MeasureKey] = &[MeasureKey::WallTime, MeasureKey::MaxRss, MeasureKey::UserDefinedMetric];

    pub fn index(&self) -> usize {
        match self {
            MeasureKey::WallTime => 0,
            MeasureKey::MaxRss => 1,
            MeasureKey::UserDefinedMetric => 2,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => MeasureKey::WallTime,
            1 => MeasureKey::MaxRss,
            2 => MeasureKey::UserDefinedMetric,
            _ => panic!("invalid index"),
        }
    }
}
