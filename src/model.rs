use serde_derive::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PacsParams {
    pub thumbnail_path: Option<String>,
    pub study_uid: Option<String>,
    pub series_uid: Option<String>,
    pub object_uid: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PacsXnData {
    pub fname: String,
    pub lname: String,
    pub mname: String,
    pub sname: String,
    pub birth: Option<PrimitiveDateTime>,
    pub ext_id: String,
    pub gender: String,
    pub images: Vec<PacsImageData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PacsImageData {
    pub study_uid: String,
    pub series_uid: String,
    pub object_uid: String,
    pub series_datetime: Option<PrimitiveDateTime>,
    pub series_num: Option<u64>,
    pub file_path: String,
}
