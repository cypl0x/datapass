use serde::{Deserialize, Serialize};

/// Represents mobile data usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataUsage {
    /// Remaining data in GB
    pub remaining_gb: f64,

    /// Total data plan in GB
    pub total_gb: f64,

    /// Used data in GB
    pub used_gb: f64,

    /// Usage percentage
    pub percentage: f64,

    /// Plan name (e.g., "MagentaMobil Prepaid L")
    pub plan_name: Option<String>,

    /// Valid until date (e.g., "12. February 2026")
    pub valid_until: Option<String>,
}

impl DataUsage {
    pub fn new(
        remaining_gb: f64,
        total_gb: f64,
        plan_name: Option<String>,
        valid_until: Option<String>,
    ) -> Self {
        let used_gb = total_gb - remaining_gb;
        let percentage = if total_gb > 0.0 {
            (used_gb / total_gb) * 100.0
        } else {
            0.0
        };

        Self {
            remaining_gb,
            total_gb,
            used_gb,
            percentage,
            plan_name,
            valid_until,
        }
    }

    pub fn remaining_percentage(&self) -> f64 {
        100.0 - self.percentage
    }
}
