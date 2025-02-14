use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct LocAnalysis {
    pub loc: i32,
    pub blanks: i32,
}
