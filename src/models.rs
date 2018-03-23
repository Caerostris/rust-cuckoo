use restful;

#[derive(Deserialize, RestResource)]
#[get = "/api"]
pub struct MalwerkItems {
    pub items: Vec<MalwerkItem>,
}

#[derive(Deserialize)]
pub struct MalwerkItem {
    #[serde(rename = "Hashvalue")]
    pub hash_value: String,
    #[serde(rename = "Filename")]
    pub filename: String,
    #[serde(rename = "Positives")]
    pub positives: String,
    #[serde(rename = "Total")]
    pub total: String,
    #[serde(rename = "Detectiondate")]
    pub detection_date: String,
    #[serde(rename = "Zip file password")]
    pub zip_password: String,
    #[serde(rename = "Moreinformation")]
    pub more_information: String,
}

#[derive(Deserialize)]
pub struct CuckooTask {
    pub added_on: String,
    pub category: String,
    pub clock: String,
    pub completed_on: Option<String>,
    pub custom: Option<String>,
    pub duration: i32,
    pub enforce_timeout: bool,
    pub errors: Vec<String>,
    pub guest: CuckooGuest,
    pub id: i32,
    pub memory: bool,
    pub package: Option<String>,
    pub platform: Option<String>,
    pub priority: i32,
    pub route: Option<String>,
    pub sample: CuckooSample,
    pub sample_id: Option<i32>,
    pub started_on: Option<String>,
    pub status: String,
    pub submit_id: Option<i32>,
    pub tags: Vec<String>,
    pub target: String,
    pub timeout: i32,
}

#[derive(Deserialize)]
pub struct CuckooGuest {
    pub id: Option<u32>,
    pub label: Option<String>,
    pub manager: Option<String>,
    pub name: Option<String>,
    pub shutdown_on: Option<String>,
    pub started_on: Option<String>,
    pub status: Option<String>,
    pub task_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct CuckooSample {
    pub crc32: Option<String>,
    pub file_size: Option<u32>,
    pub file_type: Option<String>,
    pub id: Option<u32>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub sha512: Option<String>,
    pub ssdeep: Option<String>,
}

#[derive(Deserialize, RestResource)]
#[get = "/tasks/list"]
pub struct CuckooTasksResource {
    pub tasks: Vec<CuckooTask>,
}

#[derive(Deserialize, RestResource)]
#[get = "/tasks/view/{i32}"]
#[post = "/tasks/create/file"]
pub struct CuckooTaskResource {
    pub task: CuckooTask,
}

#[derive(Deserialize, RestResource)]
#[post = "/tasks/create/file"]
pub struct CuckooTaskCreateResource {
    pub task_id: i32,
}
