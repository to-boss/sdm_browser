use std::collections::HashMap;

use anyhow::{bail, Context};

use crate::{
    smartdata::models::{Model, ParsedModel},
    ModelData,
};

#[derive(Default, Debug, Clone)]
pub struct ModelCache {
    inner: HashMap<String, ParsedModel>,
}

impl ModelCache {
    pub fn new() -> Self {
        ModelCache {
            inner: HashMap::new(),
        }
    }

    pub fn flip_checked(&mut self, key: &str, index: usize) {
        if let Some(parsed_model) = self.inner.get_mut(key) {
            if let Some(property) = parsed_model.properties.get_mut(index) {
                println!("flipped {} from {}", property.name, property.checked);
                property.checked = !property.checked;
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&ParsedModel> {
        self.inner.get(key)
    }

    pub async fn get_or_fetch_and_insert(
        &mut self,
        data_model_data: &ModelData,
    ) -> Result<ParsedModel, anyhow::Error> {
        let res = self.inner.get(&data_model_data.name);
        if res.is_some() {
            return res.cloned().context("can't happen");
        }

        let res = Model::fetch_and_parse(data_model_data).await;
        if let Ok(fetched_model) = &res {
            let parsed_model = fetched_model.to_owned();
            self.inner
                .insert(data_model_data.name.to_owned(), parsed_model.clone());
            return Ok(parsed_model);
        }

        res
    }
}
