use crate::grammar::SymbolAttribute;
use crate::utils::str_vec::StrVec;
use anyhow::{bail, Result};

use super::symbol_table::{
    Instance, ScopeId, ScopedNameId, Symbol, SymbolId, SymbolKind, SymbolTable, Type, TypeEntrails,
};

pub(crate) trait SymbolFacade<'a> {
    fn name(&self) -> String;
    fn kind(&self) -> &'a SymbolKind;
    fn to_rust(&self) -> String;
    fn format(&self, scope_depth: usize) -> String;
    fn my_id(&self) -> SymbolId;
    fn name_id(&self) -> ScopedNameId;
}

pub(crate) trait InstanceFacade<'a>: SymbolFacade<'a> {
    fn type_id(&self) -> SymbolId;
    fn description(&self) -> &str;
    fn sem(&self) -> SymbolAttribute;
    fn used(&self) -> bool;
    fn reference(&self) -> &'static str;
}

pub(crate) trait TypeFacade<'a>: SymbolFacade<'a> {
    fn inner_name(&self) -> String;
    fn member_scope(&self) -> ScopeId;
    fn entrails(&self) -> &TypeEntrails;
    fn is_container(&self) -> bool;
    fn generate_range_calculation(&self) -> Result<String>;
    fn members(&self) -> &[SymbolId];
    fn lifetime(&self) -> String;
}

pub(crate) struct SymbolItem<'a> {
    symbol: &'a Symbol,
    symbol_table: &'a SymbolTable,
}

impl<'a> SymbolItem<'a> {
    pub(crate) fn new(symbol: &'a Symbol, symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol,
            symbol_table,
        }
    }
}

impl<'a> SymbolFacade<'a> for SymbolItem<'a> {
    fn name(&self) -> String {
        self.symbol_table.name(self.symbol.name_id).to_string()
    }

    fn kind(&self) -> &'a SymbolKind {
        &self.symbol.kind
    }

    fn to_rust(&self) -> String {
        self.symbol.to_rust(self.symbol_table)
    }

    fn format(&self, scope_depth: usize) -> String {
        self.symbol.format(self.symbol_table, scope_depth)
    }

    fn my_id(&self) -> SymbolId {
        self.symbol.my_id
    }

    fn name_id(&self) -> ScopedNameId {
        self.symbol.name_id
    }
}

pub(crate) struct InstanceItem<'a> {
    symbol_item: SymbolItem<'a>,
    instance: &'a Instance,
}

impl<'a> InstanceItem<'a> {
    pub(crate) fn new(symbol_item: SymbolItem<'a>, instance: &'a Instance) -> Self {
        Self {
            symbol_item,
            instance,
        }
    }
}

impl<'a> SymbolFacade<'a> for InstanceItem<'a> {
    fn name(&self) -> String {
        self.symbol_item.name()
    }

    fn kind(&self) -> &'a SymbolKind {
        self.symbol_item.kind()
    }

    fn to_rust(&self) -> String {
        self.symbol_item.to_rust()
    }

    fn format(&self, scope_depth: usize) -> String {
        self.symbol_item.format(scope_depth)
    }

    fn my_id(&self) -> SymbolId {
        self.symbol_item.my_id()
    }

    fn name_id(&self) -> ScopedNameId {
        self.symbol_item.name_id()
    }
}

impl<'a> InstanceFacade<'a> for InstanceItem<'a> {
    fn type_id(&self) -> SymbolId {
        self.instance.type_id
    }

    fn description(&self) -> &str {
        &self.instance.description
    }

    fn sem(&self) -> SymbolAttribute {
        self.instance.sem
    }

    fn used(&self) -> bool {
        self.instance.entrails.used
    }

    fn reference(&self) -> &'static str {
        match self.instance.entrails.ref_spec {
            super::symbol_table::ReferenceType::None => &"",
            super::symbol_table::ReferenceType::Ref => &"&",
        }
    }
}

pub(crate) struct TypeItem<'a> {
    symbol_item: SymbolItem<'a>,
    my_type: &'a Type,
}

impl<'a> TypeItem<'a> {
    pub(crate) fn new(symbol_item: SymbolItem<'a>, my_type: &'a Type) -> Self {
        Self {
            symbol_item,
            my_type,
        }
    }
}

impl<'a> SymbolFacade<'a> for TypeItem<'a> {
    fn name(&self) -> String {
        if self.symbol_item.name_id().is_unnamed() {
            self.my_type
                .entrails
                .format(self.symbol_item.my_id(), self.symbol_item.symbol_table)
        } else {
            self.symbol_item.name()
        }
    }

    fn kind(&self) -> &'a SymbolKind {
        self.symbol_item.kind()
    }

    fn to_rust(&self) -> String {
        self.symbol_item.to_rust()
    }

    fn format(&self, scope_depth: usize) -> String {
        self.symbol_item.format(scope_depth)
    }

    fn my_id(&self) -> SymbolId {
        self.symbol_item.my_id()
    }

    fn name_id(&self) -> ScopedNameId {
        self.symbol_item.name_id()
    }
}

impl<'a> TypeFacade<'a> for TypeItem<'a> {
    fn inner_name(&self) -> String {
        self.my_type
            .inner_name(self.symbol_item.symbol_table, self.symbol_item.symbol)
    }

    fn member_scope(&self) -> ScopeId {
        self.my_type.member_scope
    }

    fn entrails(&self) -> &TypeEntrails {
        &self.my_type.entrails
    }

    fn is_container(&self) -> bool {
        self.my_type.is_container()
    }

    fn generate_range_calculation(&self) -> Result<String> {
        let symbol_table = self.symbol_item.symbol_table;
        match self.entrails() {
            TypeEntrails::Struct => {
                let relevant_symbols = symbol_table
                    .scope(self.member_scope())
                    .symbols
                    .iter()
                    .filter(|s| {
                        let member = symbol_table.symbol_as_instance(**s);
                        member.sem() != SymbolAttribute::Clipped
                    })
                    .cloned()
                    .collect::<Vec<SymbolId>>();

                if relevant_symbols.is_empty() {
                    Ok("        Span::default()".to_string())
                } else {
                    Ok(format!(
                        "{}",
                        relevant_symbols.iter().enumerate().fold(
                            StrVec::new(8),
                            |mut acc, (i, m)| {
                                let addition = if i > 0 { "+ " } else { "" };
                                let member = symbol_table.symbol_as_instance(*m);
                                let member_type = symbol_table.symbol_as_type(member.type_id());
                                match member_type.entrails() {
                                    TypeEntrails::Vec(_) => {
                                        acc.push(format!(
                                            "{}self.{}.first().map_or(Span::default(), |f| f.span())",
                                            addition,
                                            member.name()
                                        ));
                                        acc.push(format!(
                                            "+ self.{}.last().map_or(Span::default(), |l| l.span())",
                                            member.name()
                                        ));
                                    }
                                    TypeEntrails::Option(_) => acc.push(format!(
                                        "{}self.{}.as_ref().map_or(Span::default(), |o| o.span())",
                                        addition,
                                        member.name()
                                    )),
                                    _ => acc.push(format!(
                                        "{}self.{}.span()",
                                        addition,
                                        member.name()
                                    )),
                                }
                                acc
                            }
                        )
                    ))
                }
            }
            TypeEntrails::Enum => {
                let mut enum_data = EnumRangeCalcBuilder::default().build().unwrap();
                enum_data.enum_variants = self
                    .symbol_item
                    .symbol_table
                    .scope(self.member_scope())
                    .symbols
                    .iter()
                    .fold(StrVec::new(8), |mut acc, v| {
                        let v = self.symbol_item.symbol_table.symbol_as_type(*v);
                        if let TypeEntrails::EnumVariant(a) = v.entrails() {
                            let enum_variant_type = self.symbol_item.symbol_table.symbol_as_type(*a);
                            match enum_variant_type.entrails() {
                                TypeEntrails::Vec(_) => {
                                    acc.push(format!(
                                        "{}::{}(v) => v.first().map_or(Span::default(), |f| f.span())",
                                        self.name(),
                                        v.name()
                                    ));
                                    acc.push(
                                        "+ v.last().map_or(Span::default(), |l| l.span()),".to_string(),
                                    );
                                }
                                TypeEntrails::Option(_) => acc.push(format!(
                                    "{}::{}(o) => o.as_ref().map_or(Span::default(), |o| o.span()),",
                                    self.name(),
                                    v.name()
                                )),
                                _ => {
                                    // Expr::CommentExpr(v) => v.span(),
                                    acc.push(format!("{}::{}(v) => v.span(),", self.name(), v.name()))
                                }
                            }
                        } else {
                            panic!("Expecting enum variant here!")
                        }
                        acc
                    });
                Ok(format!("{}", enum_data))
            }
            _ => bail!("Unexpected type for range calculation!"),
        }
    }

    fn members(&self) -> &[SymbolId] {
        &self
            .symbol_item
            .symbol_table
            .scope(self.member_scope())
            .symbols
    }

    fn lifetime(&self) -> String {
        self.symbol_item
            .symbol_table
            .lifetime(self.symbol_item.my_id())
    }
}

#[derive(Builder, Debug, Default)]
pub(crate) struct EnumRangeCalc {
    #[builder(default)]
    pub enum_variants: StrVec,
}

impl std::fmt::Display for EnumRangeCalc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let EnumRangeCalc { enum_variants } = self;
        f.write_fmt(ume::ume! {
            match self {
                #enum_variants
            }
        })
    }
}
