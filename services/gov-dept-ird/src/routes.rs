use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    actions,
    db,
    error::IrdError,
};

// ── Health ────────────────────────────────────────────────────────────────────

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "ird" }))
}

// ── Resolve citizen ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ResolveRequest {
    pub did: String,
}

#[derive(Serialize)]
pub struct ResolveResponse {
    pub did: String,
    #[serde(rename = "deptLocalId")]
    pub dept_local_id: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

pub async fn resolve_citizen(
    State(pool): State<PgPool>,
    Json(req): Json<ResolveRequest>,
) -> Result<Json<ResolveResponse>, IrdError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(IrdError::CitizenNotFound)?;

    Ok(Json(ResolveResponse {
        did: row.did,
        dept_local_id: row.ird_number,
        display_name: None,
    }))
}

// ── Fetch consented data ──────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct DataRequest {
    pub did: String,
    pub scopes: Vec<String>,
}

pub async fn fetch_data(
    State(pool): State<PgPool>,
    Json(req): Json<DataRequest>,
) -> Result<Json<Value>, IrdError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(IrdError::CitizenNotFound)?;

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    // Always return the current tax assessment if income or tax-summary scope granted
    let current_tax = if has_scope("ird:income") || has_scope("ird:tax-summary") {
        db::fetch_current_tax_assessment(&pool, citizen.id).await?
    } else {
        None
    };

    let tax_history = if has_scope("ird:tax-summary") {
        Some(db::fetch_all_tax_assessments(&pool, citizen.id).await?)
    } else {
        None
    };

    let gst_reg = db::fetch_gst_registration(&pool, citizen.id).await?;
    let gst_periods = if has_scope("ird:gst") || has_scope("ird:gst-history") {
        Some(db::fetch_gst_periods(&pool, citizen.id).await?)
    } else {
        None
    };

    let kiwisaver = if has_scope("ird:kiwisaver") {
        db::fetch_kiwisaver(&pool, citizen.id).await?
    } else {
        None
    };

    let wff = if has_scope("ird:wff") {
        db::fetch_wff(&pool, citizen.id).await?
    } else {
        None
    };

    // Merge income detail into the current tax assessment
    let current_tax_with_income = if let Some(ref ta) = current_tax {
        let income = db::fetch_income_for_year(&pool, citizen.id, ta.assessment_year).await?;
        Some(json!({
            "assessmentYear": ta.assessment_year,
            "taxCode": ta.tax_code,
            "employmentIncome": income.as_ref().and_then(|i| i.employment_income.as_ref().map(|v| v.to_string())),
            "selfEmploymentIncome": income.as_ref().and_then(|i| i.self_employment_income.as_ref().map(|v| v.to_string())),
            "rentalIncome": income.as_ref().and_then(|i| i.rental_income.as_ref().map(|v| v.to_string())),
            "otherIncome": income.as_ref().and_then(|i| i.other_income.as_ref().map(|v| v.to_string())),
            "totalDeductions": income.as_ref().and_then(|i| i.total_deductions.as_ref().map(|v| v.to_string())),
            "totalIncome": ta.total_income.to_string(),
            "taxableIncome": ta.taxable_income.to_string(),
            "taxLiability": ta.tax_liability.to_string(),
            "taxPaid": ta.tax_paid.to_string(),
            "taxRefundDue": ta.tax_refund_due.to_string(),
            "taxOwing": ta.tax_owing.to_string(),
            "assessmentStatus": ta.assessment_status,
        }))
    } else {
        None
    };

    Ok(Json(json!({
        "irdNumber": citizen.ird_number,
        "currentTaxYear": current_tax_with_income,
        "taxHistory": tax_history.map(|h| h.iter().map(|ta| json!({
            "assessmentYear": ta.assessment_year,
            "taxCode": ta.tax_code,
            "totalIncome": ta.total_income.to_string(),
            "taxRefundDue": ta.tax_refund_due.to_string(),
            "taxOwing": ta.tax_owing.to_string(),
            "assessmentStatus": ta.assessment_status,
        })).collect::<Vec<_>>()),
        "gstRegistered": gst_reg.as_ref().map(|r| r.registered).unwrap_or(false),
        "gstPeriods": gst_periods.map(|ps| ps.iter().map(|p| json!({
            "periodId": p.id.to_string(),
            "periodStart": p.period_start.to_string(),
            "periodEnd": p.period_end.to_string(),
            "filingDue": p.filing_due.to_string(),
            "status": p.status,
            "refundOrPayment": p.refund_or_payment.as_ref().map(|v| v.to_string()),
        })).collect::<Vec<_>>()),
        "kiwiSaver": kiwisaver.map(|k| json!({
            "membershipStatus": k.membership_status,
            "contributionRate": k.contribution_rate.to_string(),
            "employerContributionRate": k.employer_contribution_rate.as_ref().map(|v| v.to_string()),
            "scheme": k.scheme,
            "totalBalance": k.total_balance.as_ref().map(|v| v.to_string()),
            "lastContributionDate": k.last_contribution_date.as_ref().map(|d| d.to_string()),
            "governmentContributionEligible": k.government_contribution_eligible,
            "firstHomeBuyerEligible": k.first_home_buyer_eligible,
        })),
        "workingForFamilies": wff.map(|w| json!({
            "eligible": w.eligible,
            "numberOfDependantChildren": w.number_of_dependant_children,
            "incomeThreshold": w.income_threshold.to_string(),
            "currentEntitlement": if w.eligible { Some(json!({
                "familyTaxCredit": w.family_tax_credit.as_ref().map(|v| v.to_string()),
                "inWorkTaxCredit": w.in_work_tax_credit.as_ref().map(|v| v.to_string()),
                "bestStartPayment": w.best_start_payment.as_ref().map(|v| v.to_string()),
                "minimumFamilyTaxCredit": w.minimum_family_tax_credit.as_ref().map(|v| v.to_string()),
                "totalWeeklyEntitlement": w.family_tax_credit.as_ref()
                    .and_then(|ftc| w.in_work_tax_credit.as_ref().map(|iwc| {
                        (ftc + iwc).to_string()
                    })),
                "paymentFrequency": w.payment_frequency,
            })) } else { None },
            "nextReviewDate": w.next_review_date.as_ref().map(|d| d.to_string()),
        })),
    })))
}

// ── Submit action ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ActionRequest {
    pub did: String,
    pub r#type: String,
    pub parameters: Value,
    #[serde(default = "default_performed_by")]
    pub performed_by: String,
    pub ai_level: Option<String>,
}

fn default_performed_by() -> String {
    "citizen".to_owned()
}

pub async fn submit_action(
    State(pool): State<PgPool>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<Value>, IrdError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(IrdError::CitizenNotFound)?;

    actions::execute(
        &pool,
        citizen.id,
        &req.r#type,
        &req.parameters,
        &req.performed_by,
        req.ai_level.as_deref(),
    )
    .await
}

// ── List tax years ────────────────────────────────────────────────────────────

pub async fn list_tax_years(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, IrdError> {
    let citizen = db::resolve_by_did(&pool, &did)
        .await?
        .ok_or(IrdError::CitizenNotFound)?;

    let years = sqlx::query_scalar!(
        "SELECT assessment_year FROM tax_assessments WHERE citizen_id = $1 ORDER BY assessment_year DESC",
        citizen.id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(json!({ "assessmentYears": years })))
}

// ── List GST periods ──────────────────────────────────────────────────────────

pub async fn list_gst_periods(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, IrdError> {
    let citizen = db::resolve_by_did(&pool, &did)
        .await?
        .ok_or(IrdError::CitizenNotFound)?;

    let periods = db::fetch_gst_periods(&pool, citizen.id).await?;

    Ok(Json(json!({
        "periods": periods.iter().map(|p| json!({
            "periodId": p.id.to_string(),
            "periodStart": p.period_start.to_string(),
            "periodEnd": p.period_end.to_string(),
            "filingDue": p.filing_due.to_string(),
            "status": p.status,
        })).collect::<Vec<_>>()
    })))
}
