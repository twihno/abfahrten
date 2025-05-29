use crate::api::error::ErrorPage;
use crate::model::demo::Demo;

use crate::schema::demo::dsl::*;
use crate::{
    api::error::{replace_with_400_page, replace_with_500_page},
    util::app_state::{get_db_connection_from_app_state, AppStateFull},
};
use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn handle_get_demo(
    State(app_state): State<AppStateFull>,
) -> Result<Json<Vec<Demo>>, ErrorPage> {
    let mut conn = get_db_connection_from_app_state(&app_state).await?;

    // Check if flow already exists
    let res: Vec<Demo> = demo
        .select(Demo::as_select())
        .load(&mut conn)
        .await
        .map_err(replace_with_500_page)?;

    Ok(Json(res))
}

pub async fn handle_create_demo(
    State(app_state): State<AppStateFull>,
    new_demos: Result<Json<Vec<Demo>>, JsonRejection>,
) -> Result<Json<Vec<Demo>>, ErrorPage> {
    let Json(new_demos) = new_demos.map_err(replace_with_400_page)?;

    let mut conn = get_db_connection_from_app_state(&app_state).await?;

    // Check if flow already exists
    let result: Vec<Demo> = diesel::insert_into(demo)
        .values(&new_demos)
        .get_results(&mut conn)
        .await
        .map_err(replace_with_500_page)?;

    Ok(Json(result))
}
