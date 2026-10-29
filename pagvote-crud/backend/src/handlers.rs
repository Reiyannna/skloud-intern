use crate::models::{Contestant, ContestantInput, ErrorResponse};
use crate::store::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

pub type SharedState = Arc<AppState>;

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

pub async fn list_contestants(State(state): State<SharedState>) -> Json<Vec<Contestant>> {
    Json(state.list())
}

pub async fn get_contestant(
    State(state): State<SharedState>,
    Path(id): Path<u64>,
) -> Result<Json<Contestant>, (StatusCode, Json<ErrorResponse>)> {
    match state.get(id) {
        Some(contestant) => Ok(Json(contestant)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("Contestant not found")),
        )),
    }
}

pub async fn create_contestant(
    State(state): State<SharedState>,
    Json(input): Json<ContestantInput>,
) -> Result<(StatusCode, Json<Contestant>), (StatusCode, Json<ErrorResponse>)> {
    if let Some(message) = input.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::new(&message))));
    }
    if state.number_in_use(input.contestant_number, None) {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse::new("Contestant Number already in use")),
        ));
    }
    let (name, bio) = input.cleaned();
    let contestant = state.create(
        name,
        input.contestant_number,
        bio,
        input.status_or_default(),
    );
    Ok((StatusCode::CREATED, Json(contestant)))
}

pub async fn update_contestant(
    State(state): State<SharedState>,
    Path(id): Path<u64>,
    Json(input): Json<ContestantInput>,
) -> Result<Json<Contestant>, (StatusCode, Json<ErrorResponse>)> {
    if let Some(message) = input.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::new(&message))));
    }
    // Existence first, so updating a missing id is always 404,
    // even if the requested number is already used by someone else.
    if state.get(id).is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("Contestant not found")),
        ));
    }
    if state.number_in_use(input.contestant_number, Some(id)) {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse::new("contestantNumber already in use")),
        ));
    }
    let (name, bio) = input.cleaned();
    match state.update(
        id,
        name,
        input.contestant_number,
        bio,
        input.status_or_default(),
    ) {
        Some(contestant) => Ok(Json(contestant)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("Contestant not found")),
        )),
    }
}

pub async fn delete_contestant(
    State(state): State<SharedState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    if state.delete(id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("Contestant not found")),
        ))
    }
}
