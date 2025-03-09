use actix_web::{
    web::{self},
    HttpResponse, ResponseError,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ulid::Ulid;

use crate::domain::{events::TransactionType, Currency, Money};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("不正なリクエスト: {0}")]
    BadRequest(String),

    #[error("内部サーバーエラー: {0}")]
    InternalServerError(String),

    #[error("リソースが見つかりません: {0}")]
    NotFound(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ApiError::InternalServerError(msg) => HttpResponse::InternalServerError().json(msg),
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(msg),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub amount: i64,
    pub currency: Currency,
    pub transaction_type: TransactionType,
    pub description: String,
    #[serde(default)]
    pub recorded_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub transaction_type: String,
    pub description: String,
    pub recorded_at: String,
}

pub async fn create_transaction(
    req: web::Json<CreateTransactionRequest>,
) -> Result<HttpResponse, ApiError> {
    // 検証
    if req.description.trim().is_empty() {
        return Err(ApiError::BadRequest("説明は空にできません".to_string()));
    }

    if req.amount <= 0 {
        return Err(ApiError::BadRequest(
            "金額は正の数でなければなりません".to_string(),
        ));
    }

    let recorded_at = match &req.recorded_at {
        Some(date_str) => match DateTime::parse_from_rfc3339(date_str) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(e) => return Err(ApiError::BadRequest(format!("日時の形式が不正です: {}", e))),
        },
        None => Utc::now(),
    };

    let money = Money {
        amount: req.amount,
        currency: req.currency.clone(),
    };

    let transaction_id = Ulid::new();

    // TODO: 実際に記録を作って、登録する

    Ok(HttpResponse::Created().json(TransactionResponse {
        id: transaction_id.to_string(),
        amount: req.amount,
        currency: format!("{:?}", money.currency),
        transaction_type: format!("{:?}", req.transaction_type),
        description: req.description.clone(),
        recorded_at: recorded_at.to_rfc3339(),
    }))
}

pub async fn list_transactions() -> Result<HttpResponse, ApiError> {
    // TODO: 空のリストを返してるが、中身を返すようにする
    Ok(HttpResponse::Ok().json(Vec::<TransactionResponse>::new()))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/transactions")
            .route("", web::post().to(create_transaction))
            .route("", web::get().to(list_transactions)),
    );
}
