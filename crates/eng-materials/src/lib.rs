use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
    sync::OnceLock,
};

use eng_core::units::parse_equation_quantity_to_si;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MaterialRef {
    key: String,
}

#[derive(Debug, Clone)]
pub struct MaterialState {
    material_key: String,
    temperature_k: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct MaterialFile {
    key: String,
    name: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    source: String,
    properties: BTreeMap<String, PropertyDefFile>,
}

#[derive(Debug, Clone, Deserialize)]
struct PropertyDefFile {
    dimension: String,
    unit: String,
    data: String,
    #[serde(default = "default_interpolation")]
    interpolation: String,
}

#[derive(Debug, Clone)]
pub struct MaterialDef {
    pub key: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub source: String,
    pub properties: BTreeMap<String, PropertySeries>,
}

#[derive(Debug, Clone)]
pub struct PropertySeries {
    pub dimension: String,
    pub unit: String,
    pub interpolation: String,
    pub points: Vec<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct MaterialDocsEntry {
    pub key: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub source: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Error)]
pub enum MaterialError {
    #[error("material repository error: {0}")]
    Repository(String),
    #[error("unknown material '{0}'")]
    UnknownMaterial(String),
    #[error("unknown material property '{property}' for material '{material}'")]
    UnknownProperty { material: String, property: String },
    #[error(
        "temperature {temperature_k} K outside data range [{min_k}, {max_k}] for {material}.{property}"
    )]
    OutOfRange {
        material: String,
        property: String,
        temperature_k: f64,
        min_k: f64,
        max_k: f64,
    },
    #[error("unit parse failed for {dimension}: {message}")]
    UnitParse { dimension: String, message: String },
}

pub type Result<T> = std::result::Result<T, MaterialError>;

fn default_interpolation() -> String {
    "linear".to_string()
}

static MATERIAL_REPO: OnceLock<Result<BTreeMap<String, MaterialDef>>> = OnceLock::new();

fn default_data_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("materials")
}

pub fn repository() -> Result<&'static BTreeMap<String, MaterialDef>> {
    MATERIAL_REPO
        .get_or_init(|| load_repository(&default_data_root()))
        .as_ref()
        .map_err(Clone::clone)
}

pub fn catalog() -> Result<Vec<MaterialRef>> {
    let mut out = Vec::new();
    for key in repository()?.keys() {
        out.push(MaterialRef { key: key.clone() });
    }
    Ok(out)
}

pub fn get(key_or_alias: &str) -> Result<MaterialRef> {
    let query = key_or_alias.trim().to_ascii_lowercase();
    for (key, def) in repository()? {
        if key.eq_ignore_ascii_case(&query)
            || def.aliases.iter().any(|a| a.eq_ignore_ascii_case(&query))
        {
            return Ok(MaterialRef { key: key.clone() });
        }
    }
    Err(MaterialError::UnknownMaterial(key_or_alias.to_string()))
}

pub fn docs_entries() -> Result<Vec<MaterialDocsEntry>> {
    let mut out = Vec::new();
    for material in catalog()? {
        let def = material.definition()?;
        out.push(MaterialDocsEntry {
            key: def.key,
            name: def.name,
            aliases: def.aliases,
            description: def.description,
            source: def.source,
            properties: def.properties.keys().cloned().collect(),
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(out)
}

impl MaterialRef {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn definition(&self) -> Result<MaterialDef> {
        repository()?
            .get(&self.key)
            .cloned()
            .ok_or_else(|| MaterialError::UnknownMaterial(self.key.clone()))
    }

    pub fn temperature<T: IntoMaterialInput>(&self, temperature: T) -> Result<MaterialState> {
        let t_k = parse_input(temperature.into_material_input(), "temperature")?;
        Ok(MaterialState {
            material_key: self.key.clone(),
            temperature_k: t_k,
        })
    }
}

impl MaterialState {
    pub fn temperature_k(&self) -> f64 {
        self.temperature_k
    }

    pub fn material_key(&self) -> &str {
        &self.material_key
    }

    pub fn property(&self, property: &str) -> Result<f64> {
        let material = repository()?
            .get(&self.material_key)
            .ok_or_else(|| MaterialError::UnknownMaterial(self.material_key.clone()))?;
        let series =
            material
                .properties
                .get(property)
                .ok_or_else(|| MaterialError::UnknownProperty {
                    material: self.material_key.clone(),
                    property: property.to_string(),
                })?;
        interpolate_linear(
            &self.material_key,
            property,
            self.temperature_k,
            &series.points,
        )
    }
}

#[derive(Debug, Clone)]
pub enum MaterialInputValue {
    Si(f64),
    WithUnit(String),
}

pub trait IntoMaterialInput {
    fn into_material_input(self) -> MaterialInputValue;
}

impl IntoMaterialInput for f64 {
    fn into_material_input(self) -> MaterialInputValue {
        MaterialInputValue::Si(self)
    }
}

impl IntoMaterialInput for &str {
    fn into_material_input(self) -> MaterialInputValue {
        MaterialInputValue::WithUnit(self.to_string())
    }
}

impl IntoMaterialInput for String {
    fn into_material_input(self) -> MaterialInputValue {
        MaterialInputValue::WithUnit(self)
    }
}

fn parse_input(input: MaterialInputValue, dimension: &str) -> Result<f64> {
    match input {
        MaterialInputValue::Si(v) => Ok(v),
        MaterialInputValue::WithUnit(text) => parse_equation_quantity_to_si(dimension, &text)
            .map_err(|e| MaterialError::UnitParse {
                dimension: dimension.to_string(),
                message: e.to_string(),
            }),
    }
}

fn load_repository(root: &Path) -> Result<BTreeMap<String, MaterialDef>> {
    let mut out = BTreeMap::new();
    let entries = fs::read_dir(root).map_err(|e| {
        MaterialError::Repository(format!("failed to read {}: {e}", root.display()))
    })?;
    for entry in entries {
        let entry = entry.map_err(|e| MaterialError::Repository(e.to_string()))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let material_yaml = path.join("material.yaml");
        if !material_yaml.exists() {
            continue;
        }
        let text = fs::read_to_string(&material_yaml).map_err(|e| {
            MaterialError::Repository(format!("failed reading {}: {e}", material_yaml.display()))
        })?;
        let file: MaterialFile = serde_yaml::from_str(&text).map_err(|e| {
            MaterialError::Repository(format!("failed parsing {}: {e}", material_yaml.display()))
        })?;
        let mut properties = BTreeMap::new();
        for (prop_key, def) in &file.properties {
            let csv_path = path.join(&def.data);
            let points = load_points(&csv_path)?;
            properties.insert(
                prop_key.clone(),
                PropertySeries {
                    dimension: def.dimension.clone(),
                    unit: def.unit.clone(),
                    interpolation: def.interpolation.clone(),
                    points,
                },
            );
        }
        out.insert(
            file.key.clone(),
            MaterialDef {
                key: file.key,
                name: file.name,
                aliases: file.aliases,
                description: file.description,
                source: file.source,
                properties,
            },
        );
    }
    Ok(out)
}

fn load_points(path: &Path) -> Result<Vec<(f64, f64)>> {
    let text = fs::read_to_string(path).map_err(|e| {
        MaterialError::Repository(format!("failed reading {}: {e}", path.display()))
    })?;
    let mut points = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line_no == 0 && line.to_ascii_lowercase().contains("temperature_k") {
            continue;
        }
        let mut cols = line.split(',');
        let t = cols
            .next()
            .ok_or_else(|| {
                MaterialError::Repository(format!("{} malformed csv line", path.display()))
            })?
            .trim()
            .parse::<f64>()
            .map_err(|e| {
                MaterialError::Repository(format!("{} invalid temperature: {e}", path.display()))
            })?;
        let v = cols
            .next()
            .ok_or_else(|| {
                MaterialError::Repository(format!("{} malformed csv line", path.display()))
            })?
            .trim()
            .parse::<f64>()
            .map_err(|e| {
                MaterialError::Repository(format!("{} invalid value: {e}", path.display()))
            })?;
        points.push((t, v));
    }
    points.sort_by(|a, b| a.0.total_cmp(&b.0));
    Ok(points)
}

fn interpolate_linear(
    material: &str,
    property: &str,
    x: f64,
    points: &[(f64, f64)],
) -> Result<f64> {
    if points.is_empty() {
        return Err(MaterialError::Repository(format!(
            "{material}.{property} has no data points"
        )));
    }
    let (min_x, max_x) = (points[0].0, points[points.len() - 1].0);
    if x < min_x || x > max_x {
        return Err(MaterialError::OutOfRange {
            material: material.to_string(),
            property: property.to_string(),
            temperature_k: x,
            min_k: min_x,
            max_k: max_x,
        });
    }
    if (x - min_x).abs() < f64::EPSILON {
        return Ok(points[0].1);
    }
    if (x - max_x).abs() < f64::EPSILON {
        return Ok(points[points.len() - 1].1);
    }
    for window in points.windows(2) {
        let (x0, y0) = window[0];
        let (x1, y1) = window[1];
        if x >= x0 && x <= x1 {
            let f = (x - x0) / (x1 - x0);
            return Ok(y0 + f * (y1 - y0));
        }
    }
    Err(MaterialError::Repository(format!(
        "failed interpolation for {material}.{property}"
    )))
}

pub fn stainless_304() -> MaterialRef {
    get("stainless_304").expect("generated material must exist")
}

pub fn aluminum_6061_t6() -> MaterialRef {
    get("aluminum_6061_t6").expect("generated material must exist")
}

impl FromStr for MaterialRef {
    type Err = MaterialError;
    fn from_str(s: &str) -> Result<Self> {
        get(s)
    }
}
