//! Transform layer — maps the raw IRD legacy format to the department DB schema.
//!
//! This is the only place that knows about both the legacy shape ([`crate::raw`])
//! and the department shape ([`crate::models`]). Swap the source format by writing
//! a new parser that feeds `RawIrdBatch`; the rest of the pipeline is unchanged.

use sqlx::types::Decimal;

use crate::{
    error::IngestError,
    models::{
        CitizenEntity, GstEntity, IncomeEntity, KiwiSaverEntity, TaxEntity, TransformedCitizen,
        WffEntity,
    },
    raw::RawIrdCitizen,
};

fn dec(v: f64) -> Decimal {
    Decimal::try_from(v).unwrap_or_default()
}

/// Transform a single raw citizen record into schema-aligned entities.
pub fn transform_citizen(raw: &RawIrdCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with ird_number {} has no DID",
            raw.ird_number
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        ird_number: raw.ird_number.clone(),
    };

    let income = raw.income.as_ref().map(|i| IncomeEntity {
        assessment_year: i.assessment_year,
        employment_income: Some(dec(i.employment_income)),
        self_employment_income: Some(dec(i.self_employment_income)),
        rental_income: Some(dec(i.rental_income)),
        other_income: Some(dec(i.other_income)),
        total_deductions: Some(dec(i.total_deductions)),
    });

    let tax = raw.tax_assessment.as_ref().map(|t| TaxEntity {
        assessment_year: t.assessment_year,
        tax_code: t.tax_code.clone(),
        total_income: dec(t.total_income),
        taxable_income: dec(t.taxable_income),
        tax_liability: dec(t.tax_liability),
        tax_paid: dec(t.tax_paid),
        tax_refund_due: dec(t.tax_refund_due),
        tax_owing: dec(t.tax_owing),
        assessment_status: t.assessment_status.clone(),
    });

    let gst = GstEntity {
        registered: raw.gst.registered,
        gst_number: raw.gst.gst_number.clone(),
        filing_frequency: raw.gst.filing_frequency.clone(),
    };

    let kiwisaver = raw.kiwisaver.as_ref().map(|k| KiwiSaverEntity {
        membership_status: k.membership_status.clone(),
        contribution_rate: dec(k.contribution_rate),
        employer_contribution_rate: k.employer_contribution_rate.map(dec),
        scheme: k.scheme.clone(),
        total_balance: k.total_balance.map(dec),
        government_contribution_eligible: k.government_contribution_eligible,
        first_home_buyer_eligible: k.first_home_buyer_eligible,
    });

    let wff = raw.working_for_families.as_ref().map(|w| WffEntity {
        eligible: w.eligible,
        number_of_dependant_children: w.number_of_dependant_children,
        income_threshold: dec(w.income_threshold),
        family_tax_credit: w.family_tax_credit.map(dec),
        in_work_tax_credit: w.in_work_tax_credit.map(dec),
        best_start_payment: w.best_start_payment.map(dec),
        minimum_family_tax_credit: w.minimum_family_tax_credit.map(dec),
        payment_frequency: w.payment_frequency.clone(),
    });

    Ok(TransformedCitizen {
        citizen,
        income,
        tax,
        gst,
        kiwisaver,
        wff,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawIrdCitizen {
        serde_json::from_value(serde_json::json!({
            "irdNumber": "123-456-789",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "income": {
                "assessmentYear": 2025,
                "employmentIncome": 65000.0,
                "selfEmploymentIncome": 0.0,
                "rentalIncome": 0.0,
                "otherIncome": 1200.0,
                "totalDeductions": 0.0
            },
            "taxAssessment": {
                "assessmentYear": 2025,
                "taxCode": "M",
                "totalIncome": 66200.0,
                "taxableIncome": 66200.0,
                "taxLiability": 14833.0,
                "taxPaid": 15000.0,
                "taxRefundDue": 167.0,
                "taxOwing": 0.0,
                "assessmentStatus": "final"
            },
            "gst": { "registered": false },
            "kiwisaver": {
                "membershipStatus": "active",
                "contributionRate": 3.0,
                "employerContributionRate": 3.0,
                "scheme": "ANZ Default KiwiSaver Scheme",
                "totalBalance": 18500.0,
                "governmentContributionEligible": true,
                "firstHomeBuyerEligible": true
            },
            "workingForFamilies": {
                "eligible": true,
                "numberOfDependantChildren": 2,
                "incomeThreshold": 42700.0,
                "familyTaxCredit": 127.0,
                "inWorkTaxCredit": 72.5,
                "paymentFrequency": "weekly"
            }
        }))
        .unwrap()
    }

    #[test]
    fn maps_core_fields() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.ird_number, "123-456-789");
        assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
        assert!(!t.gst.registered);
    }

    #[test]
    fn maps_income_and_tax() {
        let t = transform_citizen(&sample()).unwrap();
        let inc = t.income.unwrap();
        assert_eq!(inc.assessment_year, 2025);
        assert_eq!(inc.employment_income, Some(Decimal::try_from(65000.0).unwrap()));
        let tax = t.tax.unwrap();
        assert_eq!(tax.tax_refund_due, Decimal::try_from(167.0).unwrap());
        assert_eq!(tax.assessment_status, "final");
    }

    #[test]
    fn maps_kiwisaver_and_wff() {
        let t = transform_citizen(&sample()).unwrap();
        let ks = t.kiwisaver.unwrap();
        assert_eq!(ks.contribution_rate, Decimal::try_from(3.0).unwrap());
        assert_eq!(ks.total_balance, Some(Decimal::try_from(18500.0).unwrap()));
        let wff = t.wff.unwrap();
        assert!(wff.eligible);
        assert_eq!(wff.number_of_dependant_children, 2);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
