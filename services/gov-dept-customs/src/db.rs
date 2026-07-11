use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub traveller_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TravelRow {
    pub id: Uuid,
    pub passport_number: String,
    pub last_arrival: chrono::NaiveDate,
    pub arrival_port: String,
    pub frequent_traveller: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DeclarationRow {
    pub id: Uuid,
    pub declaration_id: String,
    pub date: chrono::NaiveDate,
    pub country_from: String,
    pub goods_declared: String,
    pub status: String,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, traveller_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_travel(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<TravelRow>> {
    sqlx::query_as!(
        TravelRow,
        "SELECT id, passport_number, last_arrival, arrival_port, frequent_traveller \
         FROM customs_travel WHERE citizen_id = $1 ORDER BY last_arrival DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_declarations(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<DeclarationRow>> {
    sqlx::query_as!(
        DeclarationRow,
        "SELECT id, declaration_id, date, country_from, goods_declared, status \
         FROM customs_declarations WHERE citizen_id = $1 ORDER BY date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn log_action(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: serde_json::Value,
    performed_by: &str,
    ai_level: Option<&str>,
    result_success: bool,
    result_message: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO actions_log (citizen_id, action_type, parameters, performed_by, ai_level, result_success, result_message)
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        citizen_id,
        action_type,
        parameters,
        performed_by,
        ai_level,
        result_success,
        result_message
    )
    .execute(pool)
    .await?;
    Ok(())
}
