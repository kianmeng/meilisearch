use actix_web::web::Payload;
use actix_web::{delete, get, post, put};
use actix_web::{web, HttpResponse};
use indexmap::IndexMap;
use log::error;
use milli::update::{IndexDocumentsMethod, UpdateFormat};
use serde::Deserialize;
use serde_json::Value;

use crate::Data;
use crate::error::ResponseError;
use crate::helpers::Authentication;
use crate::routes::IndexParam;

const DEFAULT_RETRIEVE_DOCUMENTS_OFFSET: usize = 0;
const DEFAULT_RETRIEVE_DOCUMENTS_LIMIT: usize = 20;

macro_rules! guard_content_type {
    ($fn_name:ident, $guard_value:literal) => {
        fn $fn_name(head: &actix_web::dev::RequestHead) -> bool {
            if let Some(content_type) = head.headers.get("Content-Type") {
                content_type.to_str().map(|v| v.contains($guard_value)).unwrap_or(false)
            } else {
                false
            }
        }
    };
}

guard_content_type!(guard_json, "application/json");

type Document = IndexMap<String, Value>;

#[derive(Deserialize)]
struct DocumentParam {
    _index_uid: String,
    _document_id: String,
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_document)
        .service(delete_document)
        .service(get_all_documents)
        .service(add_documents_json)
        .service(update_documents)
        .service(delete_documents)
        .service(clear_all_documents);
}

#[get(
    "/indexes/{index_uid}/documents/{document_id}",
    wrap = "Authentication::Public"
)]
async fn get_document(
    _data: web::Data<Data>,
    _path: web::Path<DocumentParam>,
) -> Result<HttpResponse, ResponseError> {
    todo!()
}

#[delete(
    "/indexes/{index_uid}/documents/{document_id}",
    wrap = "Authentication::Private"
)]
async fn delete_document(
    _data: web::Data<Data>,
    _path: web::Path<DocumentParam>,
) -> Result<HttpResponse, ResponseError> {
    todo!()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BrowseQuery {
    offset: Option<usize>,
    limit: Option<usize>,
    attributes_to_retrieve: Option<String>,
}

#[get("/indexes/{index_uid}/documents", wrap = "Authentication::Public")]
async fn get_all_documents(
    data: web::Data<Data>,
    path: web::Path<IndexParam>,
    params: web::Query<BrowseQuery>,
) -> Result<HttpResponse, ResponseError> {
    let attributes_to_retrieve = params
        .attributes_to_retrieve
        .as_ref()
        .map(|attrs| attrs
            .split(",")
            .collect::<Vec<_>>());

    match data.retrieve_documents(
        &path.index_uid,
        params.offset.unwrap_or(DEFAULT_RETRIEVE_DOCUMENTS_OFFSET),
        params.limit.unwrap_or(DEFAULT_RETRIEVE_DOCUMENTS_LIMIT),
        attributes_to_retrieve.as_deref()) {
        Ok(docs) => {
            let json = serde_json::to_string(&docs).unwrap();
            Ok(HttpResponse::Ok().body(json))
        }
        Err(_) => { todo!() }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct UpdateDocumentsQuery {
    _primary_key: Option<String>,
}

async fn update_multiple_documents(
    _data: web::Data<Data>,
    _path: web::Path<IndexParam>,
    _params: web::Query<UpdateDocumentsQuery>,
    _body: web::Json<Vec<Document>>,
    _is_partial: bool,
) -> Result<HttpResponse, ResponseError> {
    todo!()
}

/// Route used when the payload type is "application/json"
#[post(
    "/indexes/{index_uid}/documents",
    wrap = "Authentication::Private",
    guard = "guard_json"
)]
async fn add_documents_json(
    data: web::Data<Data>,
    path: web::Path<IndexParam>,
    _params: web::Query<UpdateDocumentsQuery>,
    body: Payload,
) -> Result<HttpResponse, ResponseError> {
    let addition_result = data
        .add_documents(
            path.into_inner().index_uid,
            IndexDocumentsMethod::UpdateDocuments,
            UpdateFormat::Json,
            body
        ).await;

    match addition_result {
        Ok(update) => {
            let value = serde_json::to_string(&update).unwrap();
            let response = HttpResponse::Ok().body(value);
            Ok(response)
        }
        Err(e) => {
            error!("{}", e);
            todo!()
        }
    }
}


/// Default route for addign documents, this should return an error en redirect to the docuentation
#[post("/indexes/{index_uid}/documents", wrap = "Authentication::Private")]
async fn add_documents_default(
    _data: web::Data<Data>,
    _path: web::Path<IndexParam>,
    _params: web::Query<UpdateDocumentsQuery>,
    _body: web::Json<Vec<Document>>,
) -> Result<HttpResponse, ResponseError> {
    error!("Unknown document type");
    todo!()
}

#[put("/indexes/{index_uid}/documents", wrap = "Authentication::Private")]
async fn update_documents(
    data: web::Data<Data>,
    path: web::Path<IndexParam>,
    params: web::Query<UpdateDocumentsQuery>,
    body: web::Json<Vec<Document>>,
) -> Result<HttpResponse, ResponseError> {
    update_multiple_documents(data, path, params, body, true).await
}

#[post(
    "/indexes/{index_uid}/documents/delete-batch",
    wrap = "Authentication::Private"
)]
async fn delete_documents(
    _data: web::Data<Data>,
    _path: web::Path<IndexParam>,
    _body: web::Json<Vec<Value>>,
) -> Result<HttpResponse, ResponseError> {
    todo!()
}

#[delete("/indexes/{index_uid}/documents", wrap = "Authentication::Private")]
async fn clear_all_documents(
    _data: web::Data<Data>,
    _path: web::Path<IndexParam>,
) -> Result<HttpResponse, ResponseError> {
    todo!()
}
