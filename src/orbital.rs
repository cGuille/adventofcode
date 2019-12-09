use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
pub struct UniversalOrbitMap {
    // Index are objects orbiting around their value
    relationships: HashMap<String, String>,
}

impl UniversalOrbitMap {
    pub fn path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        let mut result = Vec::new();
        let mut next = self.relationships.get(from);

        loop {
            if next.is_none() {
                return None;
            }

            let next_value = next.unwrap();
            result.push(next_value.to_string());

            if next_value == to {
                break;
            }

            next = self.relationships.get(next_value);
        }

        Some(result)
    }

    pub fn objects(&self) -> Vec<String> {
        self.relationships.keys().map(|k| k.to_string()).collect()
    }
}

impl From<File> for UniversalOrbitMap {
    fn from(file: File) -> Self {
        UniversalOrbitMap {
            relationships: BufReader::new(file)
                .lines()
                .map(|line| {
                    let rel: OrbitalRelationship = line.unwrap().parse().unwrap();
                    (rel.satellite, rel.reference)
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct OrbitalRelationship {
    reference: String,
    satellite: String,
}

impl FromStr for OrbitalRelationship {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split(")").collect();

        if parts.len() != 2 {
            return Err(format!("Invalid orbital relationship: {}", input));
        }

        Ok(OrbitalRelationship {
            reference: parts[0].to_string(),
            satellite: parts[1].to_string(),
        })
    }
}
