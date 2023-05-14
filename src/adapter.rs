use trustfall::{
    provider::{
        field_property, resolve_property_with, BasicAdapter, ContextIterator,
        ContextOutcomeIterator, EdgeParameters, VertexIterator,
    },
    FieldValue, Schema,
};

use crate::util::read_file;
use crate::vertex::Vertex;
use serde_json;
use solc_ast::ast::{NodeType, SourceUnit};
use std::{error::Error, fs::File, io::BufReader};

#[derive(Debug, Clone, Default)]
pub struct Soltrust {
    data: Vec<SourceUnit>,
}

pub fn read_json(path: &str) -> Result<SourceUnit, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    serde_json::from_reader(&mut reader).map_err(|e| e.into())
}

impl Soltrust {
    pub fn new(path: &str) -> Self {
        let content: SourceUnit = read_json(path).unwrap();
        Self {
            data: vec![content],
        }
    }

    pub fn schema(&self) -> Schema {
        Schema::parse(read_file("./soltrust.graphql")).expect("valid schema")
    }
}

impl BasicAdapter<'static> for Soltrust {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &mut self,
        edge_name: &str,
        parameters: &EdgeParameters,
    ) -> VertexIterator<'static, Self::Vertex> {
        // get data points
        match edge_name.as_ref() {
            "Contracts" => Box::new(std::iter::once(self.data)),
            "Contract" => {
                let name = parameters["name"].as_str().unwrap();
                if let Some(found_contract) = self.data.iter().find(|contract| {
                    contract
                        .exported_symbols
                        .as_ref()
                        .expect("did not find {name} in the contracts folder")
                        == name
                }) {
                    Box::new(std::iter::once(found_contract))
                }
            }
            _ => unreachable!("resolve starting vertixes {edge_name}"),
        }
    }

    fn resolve_property(
        &mut self,
        contexts: ContextIterator<'static, Self::Vertex>,
        type_name: &str,
        property_name: &str,
    ) -> ContextOutcomeIterator<'static, Self::Vertex, FieldValue> {
        match type_name {
            "Pragna" => match property_name {
                "literals" => resolve_property_with(contexts, field_property!(literals)),
                "src" => resolve_property_with(contexts, field_property!(src)),
            },
            "Contract" => match property_name {
                "name" => resolve_property_with(contexts, field_property!(name)),
            },
            "Import" => match property_name {
                "file" => resolve_property_with(contexts, field_property!(file)),
                "unit_alias" => resolve_property_with(contexts, field_property!(unit_alias)),
            },
            "FunctionDefinition" => match property_name {
                "name" => resolve_property_with(contexts, field_property!(name)),
            },
            "Parameter" => match property_name {
                "name" => resolve_property_with(contexts, field_property!(name)),
                "type" => resolve_property_with(contexts, field_property!(_type)),
            },
            "Visibility" => match property_name {
                "public" => resolve_property_with(contexts, field_property!(public)),
                "private" => resolve_property_with(contexts, field_property!(private)),
                "internal" => resolve_property_with(contexts, field_property!(internal)),
                "external" => resolve_property_with(contexts, field_property!(external)),
            },
            "Mutability" => match property_name {
                "view" => resolve_property_with(contexts, field_property!(view)),
                "pure" => resolve_property_with(contexts, field_property!(pure)),
                "payable" => resolve_property_with(contexts, field_property!(payable)),
                "nonpayable" => resolve_property_with(contexts, field_property!(nonpayable)),
            },
        }
    }

    fn resolve_neighbors(
        &mut self,
        contexts: ContextIterator<'static, Self::Vertex>,
        type_name: &str,
        edge_name: &str,
        parameters: &EdgeParameters,
    ) -> ContextOutcomeIterator<'static, Self::Vertex, VertexIterator<'static, Self::Vertex>> {
        todo!()
    }

    fn resolve_coercion(
        &mut self,
        contexts: ContextIterator<'static, Self::Vertex>,
        type_name: &str,
        coerce_to_type: &str,
    ) -> ContextOutcomeIterator<'static, Self::Vertex, bool> {
        todo!()
    }
}
