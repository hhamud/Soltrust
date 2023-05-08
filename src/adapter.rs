use trustfall::{
    provider::{
        BasicAdapter, ContextIterator, ContextOutcomeIterator, EdgeParameters, VertexIterator,
    },
    FieldValue, Schema,
};

use crate::util::read_file;
use crate::vertex::Vertex;
use serde_json;
use solc_ast::ast::SourceUnit;
use std::{error::Error, fs::File, io::BufReader};

#[derive(Debug, Clone, Default)]
pub struct Soltrust {
    data: Vec<Vertex>,
}

pub fn read_json(path: &str) -> Result<SourceUnit, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    serde_json::from_reader(&mut reader).map_err(|e| e.into())
}

impl Soltrust {
    pub fn new(path: &str) -> Self {
        // parse solc ast json file
        let content: SourceUnit = read_json(path).unwrap();

        Self { data: Vec::new() }
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
            "Contracts" => todo!(),
            "Contract" => {
                let name = parameters["name"].as_str().unwrap();
                todo!()
                //self.data
                //Box::new(std::iter::once(Vertex::ContractPart())),
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
        todo!()
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

    fn resolve_typename(
        &mut self,
        contexts: ContextIterator<'static, Self::Vertex>,
        _type_name: &str,
    ) -> ContextOutcomeIterator<'static, Self::Vertex, FieldValue> {
        todo!()
    }
}
