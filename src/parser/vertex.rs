use super::ast::{
    contract_part_as_target, expression_as_target, source_unit_part_as_target, statement_as_target,
    Target,
};
use solang_parser::pt;
use trustfall::provider::Typename;

#[derive(PartialEq, Clone, Debug,Typename)]
pub enum Vertex {
    Statement(pt::Statement),
    Expression(pt::Expression),
    SourceUnit(pt::SourceUnit),
    SourceUnitPart(pt::SourceUnitPart),
    ContractPart(pt::ContractPart),
}

impl Into<Vertex> for pt::Statement {
    fn into(self) -> Vertex {
        Vertex::Statement(self)
    }
}

impl Into<Vertex> for Box<pt::Statement> {
    fn into(self) -> Vertex {
        Vertex::Statement(*self)
    }
}

impl Into<Vertex> for pt::Expression {
    fn into(self) -> Vertex {
        Vertex::Expression(self)
    }
}
impl Into<Vertex> for Box<pt::Expression> {
    fn into(self) -> Vertex {
        Vertex::Expression(*self)
    }
}

impl Into<Vertex> for pt::ContractPart {
    fn into(self) -> Vertex {
        Vertex::ContractPart(self)
    }
}

impl Into<Vertex> for pt::SourceUnitPart {
    fn into(self) -> Vertex {
        Vertex::SourceUnitPart(self)
    }
}

impl Into<Vertex> for pt::SourceUnit {
    fn into(self) -> Vertex {
        Vertex::SourceUnit(self)
    }
}

// TODO: expand typename for each and every enum
impl Typename for Vertex {
    fn typename(&self) -> &'static str {
        match self {
            Self::Expression(..) => "Expression",
            Self::Statement(..) => "Statement",
            Self::SourceUnit(..) => "Source_unit",
            Self::SourceUnitPart(..) => "Source_unit_part",
            Self::ContractPart(..) => "Contract_part",
        }
    }
}

impl Vertex {
    pub fn as_target(&self) -> Target {
        match self {
            Self::Expression(expression) => return expression_as_target(expression),
            Self::Statement(statement) => return statement_as_target(statement),
            Self::SourceUnit(_) => return Target::SourceUnit,
            Self::SourceUnitPart(source_unit_part) => {
                return source_unit_part_as_target(source_unit_part)
            }
            Self::ContractPart(contract_part) => return contract_part_as_target(contract_part),
        }
    }

    pub fn as_expression(self) -> Option<pt::Expression> {
        match self {
            Self::Expression(expression) => Some(expression),
            _ => None,
        }
    }

    pub fn as_statement(self) -> Option<pt::Statement> {
        match self {
            Self::Statement(statement) => Some(statement),
            _ => None,
        }
    }

    pub fn as_source_unit(self) -> Option<pt::SourceUnit> {
        match self {
            Self::SourceUnit(source_unit) => Some(source_unit),
            _ => None,
        }
    }

    pub fn as_source_unit_part(self) -> Option<pt::SourceUnitPart> {
        match self {
            Self::SourceUnitPart(source_unit_part) => Some(source_unit_part),
            _ => None,
        }
    }

    pub fn as_contract_part(self) -> Option<pt::ContractPart> {
        match self {
            Self::ContractPart(contract_part) => Some(contract_part),
            _ => None,
        }
    }

    pub fn is_source_unit_part(&self) -> bool {
        if let Self::SourceUnitPart(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_contract_part(&self) -> bool {
        if let Self::ContractPart(_) = self {
            true
        } else {
            false
        }
    }
}
