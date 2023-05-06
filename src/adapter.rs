use std::collections::HashSet;
use trustfall::{
    provider::{
        BasicAdapter, ContextIterator, ContextOutcomeIterator, EdgeParameters, VertexIterator,
    },
    FieldValue, Schema
};

use crate::parser::vertex::Vertex;
use crate::util::read_file;

#[derive(Debug, Clone, Default)]
pub struct Soltrust {
    data: HashSet<String>,
}

impl Soltrust {
    pub fn new() -> Self {
        Self {
            data: HashSet::new()
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
            "Contracts" => Box::new(std::iter::once()),
            "Contract" => {
                let name = parameters["name"].as_str().unwrap();
                Box::new(std::iter::once()),
            }
            _ => unreachable!("resolve starting vertixes {edge_name}")
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
