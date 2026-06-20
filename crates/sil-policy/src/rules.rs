pub struct RiskInput {
    pub unicode_risk: f32,
    pub confusable_risk: f32,
    pub semantic_risk: f32,
}

pub fn evaluate_risk(input: RiskInput) -> f32 {
    (input.unicode_risk * 0.4) + (input.confusable_risk * 0.4) + (input.semantic_risk * 0.2)
}
