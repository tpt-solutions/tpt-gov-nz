//! Transform layer — maps the raw HUD legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{
        ApplicationEntity, CitizenEntity, MaintenanceRequestEntity, TenancyEntity,
        TransformedCitizen,
    },
    raw::RawHudCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawHudCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with client number {} has no DID",
            raw.client_number
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        client_number: raw.client_number.clone(),
    };

    let applications = raw
        .applications
        .iter()
        .map(|a| {
            Ok(ApplicationEntity {
                application_number: a.application_number.clone(),
                application_type: a.application_type.clone(),
                status: a.status.clone(),
                priority_band: a.priority_band.clone(),
                bedrooms_needed: a.bedrooms_needed,
                submitted_date: parse_date(&a.submitted_date, "submitted date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let tenancies = raw
        .tenancies
        .iter()
        .map(|t| {
            Ok(TenancyEntity {
                tenancy_id: t.tenancy_id.clone(),
                property_address: t.property_address.clone(),
                weekly_rent: t.weekly_rent,
                income_related_rent: t.income_related_rent,
                start_date: parse_date(&t.start_date, "start date")?,
                status: t.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let maintenance_requests = raw
        .maintenance_requests
        .iter()
        .map(|m| {
            Ok(MaintenanceRequestEntity {
                request_number: m.request_number.clone(),
                category: m.category.clone(),
                status: m.status.clone(),
                description: m.description.clone(),
                requested_date: parse_date(&m.requested_date, "requested date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        applications,
        tenancies,
        maintenance_requests,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawHudCitizen {
        serde_json::from_value(serde_json::json!({
            "clientNumber": "HUD-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "applications": [{ "applicationNumber": "HUD-A5001", "applicationType": "public-housing", "status": "waitlisted", "priorityBand": "B", "bedroomsNeeded": 2, "submittedDate": "2026-01-15" }],
            "tenancies": [{ "tenancyId": "HUD-TEN-1", "propertyAddress": "12 Totara Street, Porirua", "weeklyRent": 180, "incomeRelatedRent": true, "startDate": "2025-11-01", "status": "active" }],
            "maintenanceRequests": [{ "requestNumber": "HUD-M3001", "category": "plumbing", "status": "scheduled", "description": "Leaking tap", "requestedDate": "2026-06-20" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_application() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.client_number, "HUD-100001");
        assert_eq!(t.applications.len(), 1);
        assert_eq!(t.applications[0].application_number, "HUD-A5001");
        assert_eq!(t.applications[0].priority_band, Some("B".to_owned()));
    }

    #[test]
    fn maps_tenancy_and_maintenance() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.tenancies.len(), 1);
        assert_eq!(t.tenancies[0].tenancy_id, "HUD-TEN-1");
        assert_eq!(t.maintenance_requests.len(), 1);
        assert_eq!(t.maintenance_requests[0].request_number, "HUD-M3001");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_submitted_date() {
        let mut raw = sample();
        raw.applications[0].submitted_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
