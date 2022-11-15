
use serde::{Serialize, Deserialize};
use strum::{Display, EnumDiscriminants, EnumIter, EnumString, IntoStaticStr};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DruidClusterConfig {
    pub deep_storage: DeepStorageSpec,
}
#[derive(Clone, Debug, Deserialize, Serialize, Display)]
#[serde(rename_all = "camelCase")]
pub enum DeepStorageSpec {
    #[serde(rename = "hdfs")]
    #[strum(serialize = "hdfs")]
    HDFS(HdfsDeepStorageSpec),
    #[strum(serialize = "s3")]
    S3(S3DeepStorageSpec),
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HdfsDeepStorageSpec {
    pub config_map_name: String,
    pub directory: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3DeepStorageSpec {
    pub base_key: Option<String>,
}


fn main() {
    let input = r#"
    deepStorage:
      hdfs:
        configMapName: druid-hdfs
        directory: /druid
    "#;
    let _druid_cluster_config: DruidClusterConfig =
        serde_yaml::from_str(input).expect("well...this sucks");

}
