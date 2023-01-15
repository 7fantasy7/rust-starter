use axum::extract::{Path, State};
use axum::http::StatusCode;
use sea_orm::*;

use crate::entities::prelude::{Organization, Project};
use crate::entities::{organization, project};
use crate::AppState;

pub(crate) async fn create(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    let conn = &state.conn;

    let organization = organization::ActiveModel {
        name: Set(name.clone()),
        ..Default::default()
    };
    let org_res = Organization::insert(organization).exec(conn).await.unwrap();

    let project = project::ActiveModel {
        name: Set(name.clone()),
        organization_id: Set(Some(org_res.last_insert_id)),
        ..Default::default()
    };
    Project::insert(project).exec(conn).await.unwrap();

    Ok(name)
}

pub(crate) async fn find(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    // TODO get from req instead of passing both req and appData?
    //     let pool = req.app_data::<web::Data<Pool<MySql>>>().unwrap();

    // TODO or implement global state with OneCell?
    // https://users.rust-lang.org/t/one-global-variable-for-mysql-connection/49063/5

    // real example of pool in static:
    // https://github.com/mozilla-services/merino/blob/10e0eaf50be3afe0d15eb09e19260778266e0a0d/merino-integration-tests/src/utils/redis.rs

    let conn = &state.conn;

    let org_by_id: Option<organization::Model> = Organization::find()
        .filter(organization::Column::Id.eq(id))
        .one(conn)
        .await
        .expect("Shit");

    // find related
    // let projects = model.find_related(Project).all(conn).await.expect("Shit");
    // let mut project_names: Vec<String> = projects.into_iter().map(|b| b.name).collect();

    if let Some(model) = org_by_id {
        Ok(model.name)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
