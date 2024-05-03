use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};

use crate::{
    models::{ProfileRequest, UserRequest},
    prisma::{profile_with_user, user, user_with_profile, Database},
};

type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

enum AppError {
    PrismaError(QueryError),
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        AppError::PrismaError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status: StatusCode = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(error) if error.is_prisma_error::<RecordNotFound>() => {
                StatusCode::NOT_FOUND
            }
            AppError::PrismaError(_) => StatusCode::BAD_REQUEST,
        };

        status.into_response()
    }
}

pub fn create_route() -> Router {
    Router::new()
        .route("/user", get(handle_user_get).post(handle_user_post))
        .route(
            "/user/:username",
            put(handle_user_put).delete(handle_user_delete),
        )
        .route("/profile", post(handle_profile_post))
}

async fn handle_user_get(db: Database) -> AppJsonResult<Vec<user_with_profile::Data>> {
    let users: Vec<user_with_profile::Data> = db
        .user()
        .find_many(vec![])
        .include(user_with_profile::include())
        .exec()
        .await?;

    Ok(Json::from(users))
}

async fn handle_user_post(
    db: Database,
    Json(input): Json<UserRequest>,
) -> AppJsonResult<user_with_profile::Data> {
    let data: user_with_profile::Data = db
        .user()
        .create(input.username, vec![])
        .include(user_with_profile::include())
        .exec()
        .await?;

    Ok(Json::from(data))
}

async fn handle_user_put(
    db: Database,
    Path(username): Path<String>,
    Json(input): Json<UserRequest>,
) -> AppJsonResult<user_with_profile::Data> {
    let updated_user: user_with_profile::Data = db
        .user()
        .update(
            user::username::equals(username),
            vec![user::username::set(input.username)],
        )
        .include(user_with_profile::include())
        .exec()
        .await?;

    Ok(Json::from(updated_user))
}

async fn handle_user_delete(db: Database, Path(username): Path<String>) -> AppResult<StatusCode> {
    db.user()
        .delete(user::username::equals(username))
        .exec()
        .await?;

    Ok(StatusCode::OK)
}

async fn handle_profile_post(
    db: Database,
    Json(req): Json<ProfileRequest>,
) -> AppJsonResult<profile_with_user::Data> {
    let profile: profile_with_user::Data = db
        .profile()
        .create(user::username::equals(req.username), vec![])
        .include(profile_with_user::include())
        .exec()
        .await?;

    Ok(Json::from(profile))
}
