//! Transform layer — maps the raw MOH legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{
        AppointmentEntity, CitizenEntity, GpEntity, PrescriptionEntity, TransformedCitizen,
        VaccinationEntity,
    },
    raw::RawMohCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

fn parse_datetime(s: &str, field: &str) -> Result<chrono::DateTime<chrono::Utc>, IngestError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| IngestError::Transform(format!("invalid {field} datetime '{s}': {e}")))
}

/// Transform a single raw citizen record into schema-aligned entities.
pub fn transform_citizen(raw: &RawMohCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with NHI {} has no DID",
            raw.nhi
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        nhi: raw.nhi.clone(),
    };

    let gp = raw.gp_enrolment.as_ref().map(|g| GpEntity {
        practice_name: g.practice_name.clone(),
        address: g.address.clone(),
        phone: g.phone.clone(),
        enrolled_at: g
            .enrolled_at
            .as_deref()
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
    });

    let mut prescriptions = Vec::with_capacity(raw.prescriptions.len());
    for p in &raw.prescriptions {
        prescriptions.push(PrescriptionEntity {
            medication: p.medication.clone(),
            dose: p.dose.clone(),
            repeats_total: p.repeats_total,
            repeats_remaining: p.repeats_remaining,
            issued_at: parse_date(&p.issued_at, "issued at")?,
        });
    }

    let mut appointments = Vec::with_capacity(raw.appointments.len());
    for a in &raw.appointments {
        appointments.push(AppointmentEntity {
            provider: a.provider.clone(),
            appt_date: parse_datetime(&a.date, "appointment date")?,
            r#type: a.r#type.clone(),
            status: a.status.clone(),
        });
    }

    let mut vaccinations = Vec::with_capacity(raw.vaccinations.len());
    for v in &raw.vaccinations {
        vaccinations.push(VaccinationEntity {
            vaccine: v.vaccine.clone(),
            vaccine_date: parse_date(&v.date, "vaccine date")?,
            due_for_booster: v.due_for_booster,
        });
    }

    Ok(TransformedCitizen {
        citizen,
        gp,
        prescriptions,
        appointments,
        vaccinations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMohCitizen {
        serde_json::from_value(serde_json::json!({
            "nhi": "NBA1234",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "gpEnrolment": { "practiceName": "Pukekohe Family Health", "address": "12 Queen St", "phone": "09 238 0000", "enrolledAt": "2023-03-01" },
            "prescriptions": [
                { "medication": "Atorvastatin", "dose": "20mg", "repeatsTotal": 3, "repeatsRemaining": 2, "issuedAt": "2026-06-01" }
            ],
            "appointments": [
                { "provider": "Dr. K. Pewhairangi", "date": "2026-07-20T10:30:00+12:00", "type": "General check-up", "status": "booked" }
            ],
            "vaccinations": [
                { "vaccine": "Influenza", "date": "2025-04-15", "dueForBooster": false }
            ]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_gp() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.nhi, "NBA1234");
        assert!(t.gp.is_some());
        assert_eq!(t.gp.as_ref().unwrap().practice_name, "Pukekohe Family Health");
    }

    #[test]
    fn maps_prescriptions_and_vaccinations() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.prescriptions.len(), 1);
        assert_eq!(t.prescriptions[0].medication, "Atorvastatin");
        assert_eq!(t.vaccinations.len(), 1);
        assert!(!t.vaccinations[0].due_for_booster);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_appointment_datetime() {
        let mut raw = sample();
        raw.appointments[0].date = "not-a-date".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
