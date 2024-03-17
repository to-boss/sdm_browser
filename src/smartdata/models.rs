use std::{cmp::Ordering, collections::BTreeMap};

use dioxus::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};

const OFFICIAL_LIST_LINK: &str = "https://raw.githubusercontent.com\
/smart-data-models/data-models/master/specs/AllSubjects/official_list_data_models.json";

#[derive(Deserialize, Serialize, Debug, Props, PartialEq, Clone)]
pub struct ModelList {
    #[serde(rename = "updatedDate")]
    pub date: String,
    #[serde(rename = "officialList")]
    pub entries: Vec<DataModelRepo>,
}

impl ModelList {
    pub async fn fetch() -> Result<Self, reqwest::Error> {
        let model_list = reqwest::get(OFFICIAL_LIST_LINK)
            .await?
            .json::<ModelList>()
            .await?;

        Ok(model_list)
    }
}

#[derive(Deserialize, Serialize, Debug, Props, PartialEq, Clone)]
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
    let (_, name) = name.split_once('.').expect("contains 'datamodel.'");
    Ok(name.to_string())
}

pub struct Link<'a> {
    github_user_content: &'a str,
    master: &'a str,
    ending: &'a str,
}

impl<'a> Link<'a> {
    pub fn to_data_model_repo(&self, repo_name: &str, data_model: &str) -> String {
        format!(
            "{githubusercontent}{repo_name}{master}{data_model}{yaml}",
            githubusercontent = self.github_user_content,
            master = self.master,
            yaml = self.ending
        )
    }
}

pub const GITHUB_MODEL_YAML: Link = Link {
    github_user_content: "https://raw.githubusercontent.com/smart-data-models/dataModel.",
    master: "/master/",
    ending: "/model.yaml",
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Model {
    pub description: String,
    pub properties: BTreeMap<String, Property>,
    pub required: Vec<String>,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(rename = "x-derived-from")]
    pub derived_from: String,
    #[serde(rename = "x-disclaimer")]
    pub disclaimer: String,
    #[serde(rename = "x-license-url")]
    pub license_url: String,
    #[serde(rename = "x-model-schema")]
    pub model_schema: String,
    #[serde(rename = "x-model-tags")]
    pub model_tags: String,
    #[serde(rename = "x-version")]
    pub version: String,
}

impl Model {
    pub async fn fetch(url: &str) -> Result<Self, reqwest::Error> {
        let body = reqwest::get(url).await?.text().await?;

        let mut yaml: BTreeMap<String, Model> = serde_yaml::from_str(&body).unwrap();
        let (_, model) = yaml.pop_first().expect("we have a object layer");

        Ok(model)
    }

    pub fn into_sorted_properties(self) -> Vec<Property> {
        let mut properties = Vec::with_capacity(self.properties.len());
        for (key, mut val) in self.properties.into_iter() {
            if self.required.contains(&key) {
                val.required = true;
                val.marked = true;
            }
            val.name = key;
            properties.push(val)
        }

        // Sorting inspired by this answer on SO
        // https://stackoverflow.com/questions/46512227/sort-a-vector-with-a-comparator-which-changes-its-behavior-dynamically/46514082#46514082
        // First we give the importance order
        // We want marked fields to be at the top, then we sort marked fields after name.len()
        // Fields which are not marked are sorted after their name in alphabetical order
        enum Field {
            Marked,
            Name,
        }

        let orders = [Field::Marked, Field::Name];
        properties.sort_by(|a, b| {
            orders.iter().fold(Ordering::Equal, |acc, field| {
                acc.then_with(|| match field {
                    Field::Marked => {
                        if a.marked && b.marked {
                            a.name.len().cmp(&b.name.len())
                        } else {
                            b.marked.cmp(&a.marked)
                        }
                    }
                    Field::Name => a.name.cmp(&b.name),
                })
            })
        });

        properties
    }
}

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(default)]
pub struct Property {
    #[serde(rename = "anyOf")]
    pub any_of: Option<Vec<serde_yaml::Value>>,
    pub format: Option<String>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<serde_yaml::Value>>,
    #[serde(rename = "enum")]
    pub enums: Option<Vec<String>>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<String>,
    #[serde(rename = "x-ngsi")]
    pub xngsi: Option<XNgsi>,
    #[serde(skip_deserializing)]
    pub marked: bool,
    #[serde(skip_deserializing)]
    pub name: String,
    #[serde(skip_deserializing)]
    pub required: bool,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct XNgsi {
    pub model: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub units: Option<String>,
}
