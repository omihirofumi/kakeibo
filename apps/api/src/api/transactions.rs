use std::fmt::format;

use actix_web::{web, HttpResponse, Responder, ResponseError};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ulid::Ulid;

use crate::domain::{events::TransactionType, Currency};

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

pub async fn create_transaction(req: web::Json<CreateTransactionRequest>) -> impl Responder {
    let transaction_id = Ulid::new();

    // TODO: 値のバリデーション

    HttpResponse::Created().json(TransactionResponse {
        id: transaction_id.to_string(),
        amount: req.amount,
        currency: format!("{:?}", req.currency),
        transaction_type: format!("{:?}", req.transaction_type),
        description: req.description.clone(),
        recorded_at: Utc::now().to_rfc3339(),
    })
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/transactions"))
        .route("", web::post().to(create_transaction));
}
