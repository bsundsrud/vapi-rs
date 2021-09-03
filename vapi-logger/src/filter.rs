use regex::Regex;

#[derive(Debug, Clone)]
pub enum LogFilter {
    ExactFilter(String),
    PatternFilter(Regex),
    AndFilter(Vec<LogFilter>),
    OrFilter(Vec<LogFilter>),
    NotFilter(Box<LogFilter>),
}

impl LogFilter {
    pub fn exact<S: Into<String>>(t: S) -> LogFilter {
        LogFilter::ExactFilter(t.into())
    }

    pub fn negate(self) -> LogFilter {
        LogFilter::NotFilter(Box::new(self))
    }

    pub fn not(f: LogFilter) -> LogFilter {
        LogFilter::NotFilter(Box::new(f))
    }

    pub fn pattern(p: &str) -> LogFilter {
        LogFilter::PatternFilter(Regex::new(p).unwrap())
    }

    pub fn and<F: IntoIterator<Item = LogFilter>>(filters: F) -> LogFilter {
        LogFilter::AndFilter(filters.into_iter().collect())
    }

    pub fn or<F: IntoIterator<Item = LogFilter>>(filters: F) -> LogFilter {
        LogFilter::OrFilter(filters.into_iter().collect())
    }

    pub fn matches(&self, value: &str) -> bool {
        match *self {
            LogFilter::ExactFilter(ref v) => v == value,
            LogFilter::PatternFilter(ref p) => p.is_match(value),
            LogFilter::AndFilter(ref filters) => {
                for filter in filters {
                    if !filter.matches(value) {
                        return false;
                    }
                }
                true
            }
            LogFilter::OrFilter(ref filters) => {
                for filter in filters {
                    if filter.matches(value) {
                        return true;
                    }
                }
                false
            }
            LogFilter::NotFilter(ref f) => !f.matches(value),
        }
    }
}
