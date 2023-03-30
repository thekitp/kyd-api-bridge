/*
[
  {
    "time": "2021-12-19T11:16:41.742791+00:00",
    "data": {
      "imgDim": {
        "width": 3840,
        "height": 2160
      },
      "fullImg": "/assets/vamstack/images/analytics/results/8-165-1639912584408000000-fullframe.jpg",
      "sourceId": 247,
      "sourceName": "PIT-Face-Demo-1",
      "analyticsId": 8,
      "assignmentId": 165,
      "analyticsResult": {
        "cntIn": 1,
        "cntOut": 1,
        "objsInfo": {
          "x1": 2054,
          "x2": 2428,
          "y1": 384,
          "y2": 1670,
          "label": "person",
          "roiName": "OUT",
          "trackId": 53,
          "croppedImg": "/assets/vamstack/images/analytics/results/8-165-247-1639912584408000000.jpeg",
          "probability": 0.8213423490524292
        }
      }
    },
    "origin_name": "VAMSTACK-ONPREMISED",
    "compute_name": "vam-people-counting-in-out"
  }
]
*/

use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
pub struct Analytics {
    time: String,
    data: AnalyticsData,
    origin_name: String,
    probability: f64,
}

impl Analytics {
    pub fn fill_device_data(&mut self, device_data: KYDDeviceDataLogEntry) {
        match device_data.data {
            KYDDeviceDataType::HumanEntranceCounter { mode: _, counter } => {
                self.data.analytics_result.cnt_in = counter.cnt_in;
                self.data.analytics_result.cnt_out = counter.cnt_out;
            }
            KYDDeviceDataType::PASCO2Sensor {
                co2,
                pressure,
                temp,
            } => {
                self.data.analytics_result.sensor_co2 = co2;
                self.data.analytics_result.sensor_pressure = pressure;
                self.data.analytics_result.sensor_temperature = temp;
            }
        };
        self.time = device_data.ts;
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct AnalyticsData {
    img_dim: ImageDimentions,
    full_img: String,
    source_id: usize,
    source_name: String,
    analytics_id: usize,
    assignment_id: usize,
    analytics_result: AnalyticsResult,
}

#[derive(Serialize, Default)]
struct ImageDimentions {
    width: usize,
    height: usize,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct AnalyticsResult {
    cnt_in: usize,
    cnt_out: usize,
    sensor_co2: usize,
    sensor_pressure: usize,
    sensor_temperature: usize,
    objs_info: ObjectsInfo,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct ObjectsInfo {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
    label: String,
    roi_name: String,
    track_id: usize,
    cropped_img: String,
    probability: f64,
}

#[derive(Deserialize)]
pub struct KYDApi {}

#[derive(Deserialize, Debug)]
pub struct KYDDeviceDataLog {
    success: bool,
    msg: String,
    pub data: Vec<KYDDeviceDataLogEntry>,
}

#[derive(Deserialize, Debug)]
pub struct KYDDeviceDataLogEntry {
    _id: String,
    id: String,
    ts: String,
    #[serde(rename = "type")]
    sensor_type: usize,
    data: KYDDeviceDataType,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum KYDDeviceDataType {
    HumanEntranceCounter {
        mode: usize,
        counter: Inout,
    },
    PASCO2Sensor {
        co2: usize,
        pressure: usize,
        temp: usize,
    },
}

#[derive(Deserialize, Debug)]
pub struct Inout {
    #[serde(rename = "in")]
    cnt_in: usize,
    #[serde(rename = "out")]
    cnt_out: usize,
}

impl KYDApi {
    pub async fn fetch(device_id: String) -> Result<KYDDeviceDataLog, Box<dyn std::error::Error>> {
        let client = Client::new();
        let request = client.request(Method::GET, format!("https://services.aidery.io/data-log/id?deviceId={device_id}&sort=-ts&limit=1")).bearer_auth("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYzZGI1OGZmNzM5MTk4NDg3NWFlZTZhZCIsInJvbGUiOjUwLCJpYXQiOjE2NzUzMTk1NTF9.U2pyt_pAC1BkxA36L_CayZ3NXxvpFMn_ZbGqvP3rr1I");
        let entries = request.send().await?.json::<KYDDeviceDataLog>().await?;

        Ok(entries)
    }
}
