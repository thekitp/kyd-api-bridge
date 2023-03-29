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
    pub fn with_sensor_data(&mut self, sensor_data: KYDSensorsData) -> &mut Self {
        let KYDSensorsData {
            ts,
            cnt_in,
            cnt_out,
            sensor_co2,
            sensor_pressure,
            sensor_temperature,
        } = sensor_data;

        self.time = ts;
        self.data.analytics_result.cnt_in = cnt_in;
        self.data.analytics_result.cnt_out = cnt_out;
        self.data.analytics_result.sensor_co2 = sensor_co2;
        self.data.analytics_result.sensor_pressure = sensor_pressure;
        self.data.analytics_result.sensor_temperature = sensor_temperature;

        self
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

#[derive(Deserialize, Default)]
pub struct KYDSensorsData {
    ts: String,
    cnt_in: usize,
    cnt_out: usize,
    sensor_co2: usize,
    sensor_pressure: usize,
    sensor_temperature: usize,
}

impl KYDSensorsData {
    pub async fn fetch() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new();
        let _request = client.request(Method::GET, "https://services.aidery.io/data-log/id").bearer_auth("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYzZGI1OGZmNzM5MTk4NDg3NWFlZTZhZCIsInJvbGUiOjUwLCJpYXQiOjE2NzUzMTk1NTF9.U2pyt_pAC1BkxA36L_CayZ3NXxvpFMn_ZbGqvP3rr1I");
        Ok(Self {
            ts: "2023-03-29T11:14:32.488Z".into(),
            cnt_in: 99,
            cnt_out: 99,
            sensor_co2: 1000,
            sensor_pressure: 1000,
            sensor_temperature: 1000,
        })
    }
}
