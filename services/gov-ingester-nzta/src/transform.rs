//! Transform layer — maps the raw NZTA legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, DriverLicenceEntity, RucEntity, TransformedCitizen, VehicleEntity},
    raw::RawNztaCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawNztaCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with licence {} has no DID",
            raw.driver_licence_number
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        driver_licence_number: raw.driver_licence_number.clone(),
    };

    let driver_licence = raw.driver_licence.as_ref().map(|p| DriverLicenceEntity {
        licence_number: raw.driver_licence_number.clone(),
        full_name: p.full_name.clone(),
        licence_class: p.licence_class.clone(),
        expiry_date: parse_date(&p.expiry_date, "expiry date")?,
        conditions: p.conditions.clone(),
    });

    let vehicles = raw
        .vehicles
        .iter()
        .map(|v| {
            Ok(VehicleEntity {
                registration: v.registration.clone(),
                make: v.make.clone(),
                model: v.model.clone(),
                year: v.year,
                fuel_type: v.fuel_type.clone(),
                registration_expiry: parse_date(&v.registration_expiry, "registration expiry")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let ruc = raw
        .ruc
        .iter()
        .map(|r| {
            Ok(RucEntity {
                vehicle_rego: r.vehicle_rego.clone(),
                licence_type: r.licence_type.clone(),
                expiry_date: parse_date(&r.expiry_date, "RUC expiry")?,
                units_remaining: r.units_remaining,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        driver_licence,
        vehicles,
        ruc,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawNztaCitizen {
        serde_json::from_value(serde_json::json!({
            "driverLicenceNumber": "NZ1234567",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "driverLicence": { "fullName": "Alex Tane", "licenceClass": "1 (car)", "expiryDate": "2028-09-30" },
            "vehicles": [ { "registration": "ABC123", "make": "Toyota", "model": "Corolla", "year": 2021, "fuelType": "Petrol", "registrationExpiry": "2026-12-01" } ],
            "ruc": [ { "vehicleRego": "ABC123", "licenceType": "Heavy vehicle RUC", "expiryDate": "2027-06-30", "unitsRemaining": 1500 } ]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_driver_licence() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.driver_licence_number, "NZ1234567");
        assert!(t.driver_licence.is_some());
        assert_eq!(t.driver_licence.as_ref().unwrap().full_name, "Alex Tane");
        assert_eq!(t.driver_licence.as_ref().unwrap().expiry_date.to_string(), "2028-09-30");
    }

    #[test]
    fn maps_vehicles_and_ruc() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.vehicles.len(), 1);
        assert_eq!(t.vehicles[0].registration, "ABC123");
        assert_eq!(t.vehicles[0].year, 2021);
        assert_eq!(t.ruc.len(), 1);
        assert_eq!(t.ruc[0].units_remaining, 1500);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_expiry_date() {
        let mut raw = sample();
        raw.driver_licence.as_mut().unwrap().expiry_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
