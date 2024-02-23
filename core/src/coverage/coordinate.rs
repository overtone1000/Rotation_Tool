#[derive(Eq, Hash, PartialEq, Clone)]
pub struct CoverageCoordinates {
    pub site: String,
    pub subspecialty: String,
    pub context: String,
    //modality: String,
    pub weekday: chrono::Weekday,
}

impl PartialOrd for CoverageCoordinates {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.site.partial_cmp(&other.site) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.subspecialty.partial_cmp(&other.subspecialty) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.context.partial_cmp(&other.context) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        /*
        match self.modality.partial_cmp(&other.modality) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        */
        self.weekday
            .num_days_from_monday()
            .partial_cmp(&other.weekday.num_days_from_monday())
    }
}

impl Ord for CoverageCoordinates {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(x) => x,
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl Default for CoverageCoordinates {
    fn default() -> Self {
        Self {
            site: Default::default(),
            subspecialty: Default::default(),
            context: Default::default(),
            //modality: Default::default(),
            weekday: chrono::Weekday::Sun,
        }
    }
}
