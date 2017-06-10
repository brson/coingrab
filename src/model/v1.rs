use chrono::{DateTime, UTC};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub fourchan: Vec<FourChanRun>,
}

#[derive(Serialize, Deserialize)]
pub struct FourChanRun {
    pub time: DateTime<UTC>,
    pub threads: Vec<FourChanThread>,
}

#[derive(Serialize, Deserialize)]
pub struct FourChanThread {
    pub sym: String,
    pub id: String,
    pub link: String,
}

impl Default for Data {
    fn default() -> Data {
        Data {
            fourchan: vec![],
        }
    }
}
