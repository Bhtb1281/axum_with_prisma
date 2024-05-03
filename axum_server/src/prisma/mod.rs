#[allow(warnings, unused)]
mod prisma;
pub use prisma::*;

use axum::Extension;
use std::sync::Arc;

pub type Database = Extension<Arc<PrismaClient>>;

user::include!(user_with_profile { profile });
profile::include!(profile_with_user { user });

pub async fn get_prisma_client() -> Database {
    let client: PrismaClient = PrismaClient::_builder().build().await.unwrap();

    return Extension(Arc::new(client));
}
