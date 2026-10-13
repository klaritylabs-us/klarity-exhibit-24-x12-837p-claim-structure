
use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_claim_file(text: &str) -> ValidationReport {
    let t = text.trim();
    if t.starts_with('{') {
        return validate_claim_json(t).unwrap_or_else(|e| {
            let mut r = ValidationReport::ok();
            r.push(ValidationIssue::error("json", e, "json-parse"));
            r
        });
    }
    validate_837p_sketch(t)
}

fn validate_claim_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Claim") {
        r.push(ValidationIssue::error("resourceType", "professional claim exhibit uses Claim", "claim-type"));
    }
    if v.pointer("/patient/reference").is_none() {
        r.push(ValidationIssue::error("patient", "patient reference required", "claim-patient"));
    }
    if v.get("item").and_then(|i| i.as_array()).is_none_or(|a| a.is_empty()) {
        r.push(ValidationIssue::error("item", "at least one claim item expected", "claim-item"));
    }
    Ok(r)
}

fn validate_837p_sketch(text: &str) -> ValidationReport {
    let mut r = ValidationReport::ok();
    if !text.contains("ST*") {
        r.push(ValidationIssue::error("x12", "837P sketch expects ST transaction set header", "st"));
    }
    if !text.contains("CLM*") {
        r.push(ValidationIssue::error("x12", "837P sketch expects CLM claim segment", "clm"));
    }
    if !text.contains("SV1*") {
        r.push(ValidationIssue::warning("x12", "professional lines often use SV1 service segment", "sv1"));
    }
    r
}
