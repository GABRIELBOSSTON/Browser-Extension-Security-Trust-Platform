use crate::domain::errors::Result;
use crate::domain::rules::RuleSet;
use crate::infrastructure::rules::source::RuleSource;

pub struct RuleRepository {
    source: Box<dyn RuleSource>,
}

impl RuleRepository {
    pub fn new(source: Box<dyn RuleSource>) -> Self {
        Self { source }
    }

    pub fn load_rules(&self) -> Result<RuleSet> {
        self.source.fetch()
    }
}
