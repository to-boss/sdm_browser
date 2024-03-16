use serde::{Deserialize, Deserializer, Serialize};

const OFFICIAL_LIST_LINK: &str = "https://raw.githubusercontent.com\
/smart-data-models/data-models/master/specs/AllSubjects/official_list_data_models.json";

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ModelList {
    #[serde(rename = "updatedDate")]
    pub date: String,
    #[serde(rename = "officialList")]
    pub entries: Vec<DataModelRepo>,
}

impl ModelList {
    pub async fn fetch() -> Result<Self, reqwest::Error> {
        let model_list = reqwest::get(OFFICIAL_LIST_LINK)
            .await
            .unwrap()
            .json::<ModelList>()
            .await
            .unwrap();

        Ok(model_list)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct DataModelRepo {
    #[serde(rename = "repoName")]
    #[serde(deserialize_with = "remove_part")]
    pub name: String,
    #[serde(rename = "repoLink")]
    pub link: String,
    #[serde(rename = "dataModels")]
    pub data_models: Vec<String>,
    pub domains: Vec<String>,
}

fn remove_part<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let name: &str = Deserialize::deserialize(deserializer)?;
    let (_, name) = name.split_once('.').expect("contains datamodel.");
    Ok(name.to_string())
}

pub struct Link<'a> {
    part1: &'a str,
    part2: &'a str,
    part3: &'a str,
}

impl<'a> Link<'a> {
    pub fn to_data_model_repo(&self, repo_name: &String, data_model: &String) -> String {
        format!(
            "{githubusercontent}{repo_name}{master}{data_model}{schema}",
            githubusercontent = self.part1,
            master = self.part2,
            schema = self.part3
        )
    }
}

// https://raw.githubusercontent.com/smart-data-models/dataModel.Parking
// /master/
// ParkingSpot
// /schema.json
pub const GITHUB_LINK: Link = Link {
    part1: "https://raw.githubusercontent.com/smart-data-models/dataModel.",
    part2: "/master/",
    part3: "/schema.json",
};
