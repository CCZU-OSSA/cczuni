#[derive(Debug, Clone)]
pub struct GradeData {
    pub name: String,
    pub point: String,
    pub grade: String,
}

#[derive(Debug, Clone)]
pub struct TechPlanData {
    pub term: String,
    pub code: String,
    pub name: String,
    pub category: String,
    pub period: String,
    pub credit: String,
    pub exam: String,
    pub exp_period: String,
    pub exp_credit: String,
    pub practice_period: String,
    pub specialization: String,
    pub faculty: String,
}
