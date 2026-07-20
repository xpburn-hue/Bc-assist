use serde::Deserialize;

use super::bc::{BallisticCoefficient, DragCurve};
use super::fixtures::{ProjectileDataset, ProjectileFixture, VelocitySample};

#[derive(Debug)]
pub enum DatasetImportError {
    ParseError(String),
    InvalidDataset,
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

pub fn from_json(input: &str) -> Result<ProjectileDataset, DatasetImportError> {
    let file: DatasetFile = serde_json::from_str(input)
        .map_err(|error| DatasetImportError::ParseError(error.to_string()))?;

    build_dataset(file)
}

pub fn from_csv(input: &str) -> Result<ProjectileDataset, DatasetImportError> {
    let mut reader = csv::Reader::from_reader(input.as_bytes());
    let samples = reader
        .deserialize()
        .map(|result| {
            result
                .map(|sample: VelocitySampleFile| VelocitySample {
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
            .iter()
            .map(|s| VelocitySampleFile {
                distance_yards: s.distance_yards,
                velocity_fps: s.velocity_fps,
            })
            .collect(),
    })
}

fn build_dataset(file: DatasetFile) -> Result<ProjectileDataset, DatasetImportError> {
    let dataset = ProjectileDataset {
        fixture: ProjectileFixture {
            name: Box::leak(file.projectile.name.into_boxed_str()),
            mass_grains: file.projectile.mass_grains,
            muzzle_velocity_fps: file.projectile.muzzle_velocity_fps,
            bc: BallisticCoefficient {
                value: file.projectile.bc.value,
                curve: file.projectile.bc.curve,
            },
        },
        samples: Box::leak(
            file.samples
                .into_iter()
                .map(|sample| VelocitySample {
                    distance_yards: sample.distance_yards,
                    velocity_fps: sample.velocity_fps,
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
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
