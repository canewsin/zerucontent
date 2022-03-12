use super::{File, Include, UserContents};

use json_filter_sorted::sort::sort_json;

use crate::util::is_default;
use crate::zeruformatter;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::default::Default;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Content {
    pub address: String,

    #[serde(skip_serializing_if = "is_default")]
    pub address_index: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub domain: String,
    #[serde(skip_serializing_if = "is_default")]
    pub title: String,
    //TODO! Skiping default value while serialising may cause sign verify failures
    // #[serde(skip_serializing_if = "is_default")]
    pub description: String,
    #[serde(skip_serializing_if = "is_default")]
    pub favicon: String,

    pub files: BTreeMap<String, File>,
    #[serde(skip_serializing_if = "is_default")]
    pub files_optional: BTreeMap<String, File>,

    #[serde(skip_serializing_if = "is_default")]
    pub cloneable: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub cloned_from: String,
    #[serde(skip_serializing_if = "is_default")]
    pub clone_root: String,

    #[serde(rename = "background-color")]
    #[serde(skip_serializing_if = "is_default")]
    pub background_color: String,
    #[serde(skip_serializing_if = "is_default")]
    pub viewport: String,
    #[serde(skip_serializing_if = "is_default")]
    pub translate: Vec<String>,

    #[serde(skip_serializing_if = "is_default")]
    pub user_contents: UserContents,

    pub ignore: String, //May break old zeronet sites
    #[serde(skip_serializing_if = "is_default")]
    pub inner_path: String,
    pub modified: usize, //TODO! This need to be f64 for older content.json format
    #[serde(skip_serializing_if = "is_default")]
    pub postmessage_nonce_security: bool,

    #[serde(skip_serializing_if = "is_default")]
    sign: Vec<f64>, // DEPRECATED
    #[serde(skip_serializing_if = "is_default")]
    pub signers_sign: String,
    #[serde(skip_serializing_if = "is_default")]
    pub signs: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "is_default")]
    pub signs_required: usize,

    #[serde(skip_serializing_if = "is_default")]
    pub includes: BTreeMap<String, Include>,
    #[serde(skip_serializing_if = "is_default")]
    pub merged_type: String,
    #[serde(skip_serializing_if = "is_default")]
    pub optional: String,

    #[serde(skip_serializing_if = "is_default")]
    pub settings: BTreeMap<String, serde_json::Value>,

    #[serde(flatten)]
    other: BTreeMap<String, Value>,
    pub zeronet_version: String,
}

pub fn dump<T: Serialize>(value: T) -> Result<String, serde_json::error::Error> {
    zeruformatter::to_string_zero(
        &sort_json(json!(value))
            .unwrap()
            .as_object()
            .map(|x| x.to_owned())
            .unwrap(),
    )
}

impl Content {
    pub fn create(address: String, address_index: u32) -> Content {
        Content {
            title: address.to_owned(),
            address,
            address_index,
            modified: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
            inner_path: "content.json".to_owned(),
            postmessage_nonce_security: true,
            ..Default::default()
        }
    }

    pub fn from_buf(buf: serde_bytes::ByteBuf) -> Result<Content, ()> {
        let content = match serde_json::from_slice(&buf) {
            Ok(c) => c,
            Err(_) => return Err(()),
        };
        Ok(content)
    }

    pub fn cleared(&self) -> Content {
        let mut new_content = self.clone();
        new_content.signs = BTreeMap::new();
        new_content.sign = vec![];
        new_content
    }

    pub fn dump(&self) -> Result<String, serde_json::error::Error> {
        zeruformatter::to_string_zero(
            &sort_json(json!(self.cleared()))
                .unwrap()
                .as_object()
                .map(|x| x.to_owned())
                .unwrap(),
        )
    }

    // TODO: verify should probably return more than just a bool
    pub fn verify(&self, key: String) -> bool {
        let content = self.cleared();
        let signature = match self.signs.get(&key) {
            Some(v) => v,
            None => return false,
        };

        let result =
            zeronet_cryptography::verify(content.dump().unwrap().as_bytes(), &key, &signature);
        result.is_ok()
    }

    pub fn sign(&self, privkey: String) -> String {
        let result = zeronet_cryptography::sign(self.dump().unwrap().as_bytes(), &privkey).unwrap();
        result
    }

    pub fn get_file(&self, inner_path: &str) -> Option<File> {
        if let Some(f) = self.files.get(inner_path) {
            return Some(f.clone());
        } else if let Some(f) = self.files_optional.get(inner_path) {
            return Some(f.clone());
        }
        None
    }
}
