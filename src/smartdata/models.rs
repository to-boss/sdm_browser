use std::{cmp::Ordering, collections::BTreeMap};

use dioxus::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};

use crate::DataModelData;

const OFFICIAL_LIST_LINK: &str = "https://raw.githubusercontent.com\
/smart-data-models/data-models/master/specs/AllSubjects/official_list_data_models.json";

pub fn data_model_yaml(data_model: &str, repo: &str) -> String {
    format!("https://raw.githubusercontent.com/smart-data-models/dataModel.{data_model}/master/{repo}/model.yaml")
}

pub fn data_model_github(repo_name: &str, name: &str) -> String {
    format!("https://github.com/smart-data-models/dataModel.{repo_name}/tree/master/{name}",)
}

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

    pub fn get_filtered_entries(&self, filter: &str) -> Vec<DataModelRepo> {
        fn in_name_or_children(data_model_repo: &DataModelRepo, filter: &str) -> bool {
            let in_names = data_model_repo.name.contains(filter);
            // short curcuit
            if in_names {
                return true;
            }

            let in_children = data_model_repo
                .data_models
                .iter()
                .any(|n| n.contains(filter));

            in_children
        }

        self.entries
            .iter()
            .filter(|&dmr| in_name_or_children(dmr, filter))
            .cloned()
            .collect()
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

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ParsedModel {
    pub description: String,
    pub properties: Vec<Property>,
    pub required: Vec<String>,
    pub typ: String,
    pub derived_from: String,
    pub disclaimer: String,
    pub license_url: String,
    pub schema: String,
    pub tags: String,
    pub version: String,
    pub url: String,
}

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
    pub schema: String,
    #[serde(rename = "x-model-tags")]
    pub tags: String,
    #[serde(rename = "x-version")]
    pub version: String,
    #[serde(skip_deserializing)]
    pub url: String,
}

impl Model {
    pub async fn fetch(data_model_data: &DataModelData) -> Result<Self, reqwest::Error> {
        let DataModelData {
            repo_name,
            name,
            url,
        } = data_model_data;

        let body = reqwest::get(url).await?.text().await?;

        let mut yaml: BTreeMap<String, Model> = serde_yaml::from_str(&body).unwrap();
        let (_, mut model) = yaml.pop_first().expect("we have a object layer");

        model.url = data_model_github(&repo_name, &name);

        Ok(model)
    }

    pub async fn fetch_and_parse(
        data_model_data: &DataModelData,
    ) -> Result<ParsedModel, anyhow::Error> {
        let res = Model::fetch(data_model_data).await?.into_parsed();
        Ok(res)
    }

    pub fn into_parsed(self) -> ParsedModel {
        let mut properties = Vec::with_capacity(self.properties.len());
        for (key, mut val) in self.properties.into_iter() {
            if self.required.contains(&key) {
                val.required = true;
                val.checked = true;
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
                        if a.checked && b.checked {
                            a.name.len().cmp(&b.name.len())
                        } else {
                            b.checked.cmp(&a.checked)
                        }
                    }
                    Field::Name => a.name.cmp(&b.name),
                })
            })
        });

        ParsedModel {
            description: self.description,
            properties,
            required: self.required,
            typ: self.typ,
            derived_from: self.derived_from,
            disclaimer: self.disclaimer,
            license_url: self.license_url,
            schema: self.schema,
            tags: self.tags,
            version: self.version,
            url: self.url,
        }
    }

    /*
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
    */
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
    pub checked: bool,
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

pub enum GeoProperty {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
}

impl GeoProperty {
    pub fn str(&self) -> &'static str {
        match self {
            GeoProperty::Point => "Point",
            GeoProperty::LineString => "LineString",
            GeoProperty::Polygon => "Polygon",
            GeoProperty::MultiPoint => "MultiPoint",
            GeoProperty::MultiLineString => "MultiLineString",
            GeoProperty::MultiPolygon => "MultiPolygon",
        }
    }

    pub fn array() -> [GeoProperty; 6] {
        [
            GeoProperty::Point,
            GeoProperty::LineString,
            GeoProperty::Polygon,
            GeoProperty::MultiPoint,
            GeoProperty::MultiLineString,
            GeoProperty::MultiPolygon,
        ]
    }
}
