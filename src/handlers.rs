use axum::{Json, extract::{Path, Query, State}, body::Bytes, http::{header, Response}};
use http_body_util::Full;

use crate::{error::AppError, model::{PacsXnData, PacsImageData, PacsParams}, ApiState};

pub async fn get_pacs_xn(Path(xn): Path<i32>, State(_state): State<ApiState>) -> Result<Json<PacsXnData>, AppError> {
    Ok(Json(PacsXnData {
        fname: String::from("ชื่อทดสอบ"),
        lname: String::from("นามสกุลทดสอบ"),
        mname: String::from("ชื่อกลางทดสอบ"),
        sname: String::from("คำนำหน้าชื่อทดสอบ"),
        birth: None,
        ext_id: xn.to_string(),
        gender: String::from("M"),
        images: vec![PacsImageData {
            study_uid: String::from("123"),
            series_uid: String::from("456"),
            object_uid: String::from("789"),
            series_datetime: None,
            series_num: Some(1001),
            file_path: String::from("path/to/file.jpeg"),
        }],
    }))
}

pub async fn get_pacs_thumbnail(Query(params): Query<PacsParams>, State(_state): State<ApiState>) -> Result<Response<Full<Bytes>>, AppError> {
    if let (Some(_thumbnail_path), Some(_study_uid)) = (&params.thumbnail_path, &params.study_uid) {
        let data = Bytes::new();
        let body = Full::new(data);
        Response::builder()
            .header(header::CONTENT_TYPE, "image/jpeg")
            .body(body)
            .map_err(AppError::from)
    } else {
        Err(AppError::bad_request())
    }
}

pub async fn get_pacs_image(Query(params): Query<PacsParams>, State(_state): State<ApiState>) -> Result<Response<Full<Bytes>>, AppError> {
    if let (Some(_study_uid), Some(_series_uid), Some(_object_uid)) = (&params.study_uid, &params.series_uid, &params.object_uid) {
        let data = Bytes::new();
        let body = Full::new(data);
        Response::builder()
            .header(header::CONTENT_TYPE, "image/jpeg")
            .body(body)
            .map_err(AppError::from)
    } else {
        Err(AppError::bad_request())
    }
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use axum_test::TestServer;
    use std::net::SocketAddr;
    use crate::{route, ApiState};

    #[allow(dead_code)]
    pub async fn new_test_app(state: ApiState) -> TestServer {
        let app = route::api_router(state);
        TestServer::builder()
            .save_cookies()
            .http_transport()
            .build(app.into_make_service_with_connect_info::<SocketAddr>())
            .unwrap()
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_pacs_xn() {
        let state = ApiState::new();
        let server = new_test_app(state).await;
        let _ = server.get("/xn/1234").expect_success();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_pacs_thumbnail() {
        let state = ApiState::new();
        let server = new_test_app(state).await;
        let _ = server.get("/thumbnail?thumbnail_path=1234&study_uid=5678").expect_success();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_pacs_image() {
        let state = ApiState::new();
        let server = new_test_app(state).await;
        let _ = server.get("/image?study_uid=123&series_uid=456&object_uid=789").expect_success();
    }
}