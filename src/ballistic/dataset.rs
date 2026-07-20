use serde::Deserialize;

use super::bc::{BallisticCoefficient, DragCurve};

#[derive(Debug)]
pub enum DatasetImportError {
    ParseError(String),
    InvalidDataset,
}

#[derive(Debug)]
pub struct ImportedProjectileDataset {
    pub name: String,
    pub mass_grains: f64,
    pub muzzle_velocity_fps: f64,
    pub bc: BallisticCoefficient,
    pub samples: Vec<ImportedVelocitySample>,
}

#[derive(Debug, Clone)]
pub struct ImportedVelocitySample {
    pub distance_yards: f64,
    pub velocity_fps: f64,
}

impl ImportedProjectileDataset {
    fn is_valid(&self) -> bool {
        self.mass_grains > 0.0
            && self.muzzle_velocity_fps > 0.0
            && self.bc.value > 0.0
            && !self.samples.is_empty()
            && self.samples.windows(2).all(|pair| {
                pair[1].distance_yards >= pair[0].distance_yards
                    && pair[1].velocity_fps <= pair[0].velocity_fps
            })
    }
}

#[derive(Debug, Deserialize)]
struct DatasetFile {
    projectile: ProjectileFile,
    samples: Vec<VelocitySampleFile>,
}

#[derive(Debug, Deserialize)]
struct ProjectileFile {
    name: String,
    mass_grains: f64,
    muzzle_velocity_fps: f64,
    bc: BcFile,
}

#[derive(Debug, Deserialize)]
struct BcFile {
    value: f64,
    curve: DragCurve,
}

#[derive(Debug, Deserialize)]
struct VelocitySampleFile {
    distance_yards: f64,
    velocity_fps: f64,
}

pub fn from_json(input: &str) -> Result<ImportedProjectileDataset, DatasetImportError> {
    let file: DatasetFile = serde_json::from_str(input)
        .map_err(|error| DatasetImportError::ParseError(error.to_string()))?;

    build_dataset(file)
}

pub fn from_csv(input: &str) -> Result<ImportedProjectileDataset, DatasetImportError> {
    let mut reader = csv::Reader::from_reader(input.as_bytes());
    let samples = reader
        .deserialize()
        .map(|result| {
            result
                .map(|sample: VelocitySampleFile| ImportedVelocitySample {
                    distance_yards: sample.distance_yards,
                    velocity_fps: sample.velocity_fps,
                })
                .map_err(|error| DatasetImportError::ParseError(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    build_dataset(DatasetFile {
        projectile: ProjectileFile {
            name: "Imported projectile".to_string(),
            mass_grains: 1.0,
            muzzle_velocity_fps: samples.first().map(|s| s.velocity_fps).unwrap_or(0.0),
            bc: BcFile {
                value: 1.0,
                curve: DragCurve::G1,
            },
        },
        samples: samples
            .into_iter()
            .map(|sample| VelocitySampleFile {
                distance_yards: sample.distance_yards,
                velocity_fps: sample.velocity_fps,
            })
            .collect(),
    })
}

fn build_dataset(file: DatasetFile) -> Result<ImportedProjectileDataset, DatasetImportError> {
    let dataset = ImportedProjectileDataset {
        name: file.projectile.name,
        mass_grains: file.projectile.mass_grains,
        muzzle_velocity_fps: file.projectile.muzzle_velocity_fps,
        bc: BallisticCoefficient {
            value: file.projectile.bc.value,
            curve: file.projectile.bc.curve,
        },
        samples: file
            .samples
            .into_iter()
            .map(|sample| ImportedVelocitySample {
                distance_yards: sample.distance_yards,
                velocity_fps: sample.velocity_fps,
            })
            .collect(),
    };

    if dataset.is_valid() {
        Ok(dataset)
    } else {
        Err(DatasetImportError::InvalidDataset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_json_dataset() {
        let data = r#"{"projectile":{"name":"test","mass_grains":175,"muzzle_velocity_fps":2600,"bc":{"value":0.5,"curve":"G7"}},"samples":[{"distance_yards":0,"velocity_fps":2600}]}"#;
        assert!(from_json(data).is_ok());
    }

    #[test]
    fn imports_csv_dataset() {
        let data = "distance_yards,velocity_fps\n0,2600\n100,2400\n";
        assert!(from_csv(data).is_ok());
    }
}
