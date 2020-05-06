use std::collections::{hash_map::Entry, HashMap};

use crate::stats::Stats;

/// Project Stats
///
/// Contains contributions for all contributors separated on language
#[derive(Debug, Default)]
pub struct ProjectStats {
    stats: HashMap<String, HashMap<&'static str, Stats>>,
}

impl ProjectStats {
    /// Construct a new ProjectStats
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    /// Get total contributions by name
    ///
    /// # Parameters
    ///
    /// name: S where S: AsRef<str> - name of the contributor
    ///
    /// # Returns
    ///
    /// Option<Stats>
    pub fn total_contribs_by_name<S: AsRef<str>>(&self, name: S) -> Option<Stats> {
        let contributions = self.stats.get(name.as_ref())?;
        Some(
            contributions
                .values()
                .fold(Stats::default(), |total, stats| total + stats),
        )
    }

    /// Get contributions by name broken down by language
    ///
    /// # Parameters
    ///
    /// name: S where S: AsRef<str> - name of the contributor
    ///
    /// # Returns
    ///
    /// Option<&HashMap<&'static str, Stats>>
    pub fn contribs_by_name<S: AsRef<str>>(
        &self,
        name: S,
    ) -> Option<&HashMap<&'static str, Stats>> {
        self.stats.get(name.as_ref())
    }

    /// Calculates the total amount of lines contributed to the Git Repository
    ///
    /// # Returns
    ///
    /// usize
    pub fn total_lines(&self) -> usize {
        let mut sum: usize = 0;
        for contributor in self.stats.values() {
            for stats in contributor.values() {
                sum += stats.lines;
            }
        }
        sum
    }

    /// Get an Iterator over all Contributors
    ///
    /// # Returns
    ///
    /// Iterator<Item = &String>
    pub fn contributors(&self) -> impl Iterator<Item = &String> {
        self.stats.keys()
    }

    /// Inserts into the nested HashMap<HashMap> structure
    ///
    /// # Parameters
    ///
    /// name: S where S: Into<String> - name of contributor
    ///
    /// lang: &'static str - language name
    ///
    /// stats: Stats
    pub(crate) fn insert<S: Into<String>>(&mut self, name: S, lang: &'static str, stats: Stats) {
        match self.stats.entry(name.into()) {
            Entry::Occupied(mut occupied) => {
                let stats_map = occupied.get_mut();
                let lang_stats = stats_map.entry(lang).or_insert(Stats::default());
                *lang_stats += stats;
            }
            Entry::Vacant(vacant) => {
                let mut lang_stat = HashMap::new();
                lang_stat.insert(lang, stats);
                vacant.insert(lang_stat);
            }
        }
    }
}