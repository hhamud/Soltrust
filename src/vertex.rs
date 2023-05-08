use solc_ast::ast;
use trustfall::provider::TrustfallEnumVertex;

#[derive(PartialEq, Clone, Debug, TrustfallEnumVertex)]
pub enum Vertex {
    Statement(ast::Statement),
    Expression(ast::Expression),
    Function(ast::FunctionDefinition),
    SourceUnit(ast::SourceUnitNode),
    ContractPart(ast::ContractDefinitionNode),
}

impl Into<Vertex> for ast::Statement {
    fn into(self) -> Vertex {
        Vertex::Statement(self)
    }
}

impl Into<Vertex> for Box<ast::Statement> {
    fn into(self) -> Vertex {
        Vertex::Statement(*self)
    }
}

impl Into<Vertex> for ast::Expression {
    fn into(self) -> Vertex {
        Vertex::Expression(self)
    }
}
impl Into<Vertex> for Box<ast::Expression> {
    fn into(self) -> Vertex {
        Vertex::Expression(*self)
    }
}

impl Into<Vertex> for ast::ContractDefinitionNode {
    fn into(self) -> Vertex {
        Vertex::ContractPart(self)
    }
}

impl Into<Vertex> for Box<ast::SourceUnitNode> {
    fn into(self) -> Vertex {
        Vertex::SourceUnit(*self)
    }
}

impl Into<Vertex> for ast::SourceUnitNode {
    fn into(self) -> Vertex {
        Vertex::SourceUnit(self)
    }
}
