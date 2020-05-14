use crate::parse::{Ast, Ident, Type};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FunctionIdentifier {
    name: Ident,
    args: Vec<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Function {
    return_type: Option<Type>,
}

/// Extract all functions from the AST
fn extract_functions(ast: &Ast) -> HashMap<FunctionIdentifier, Function> {
    ast.functions
        .iter()
        .map(|f| {
            let name = f.name.clone();
            let args = f.args.iter().map(|(_, t)| t).cloned().collect();
            let key = FunctionIdentifier { name, args };
            let value = Function {
                return_type: f.return_type.clone(),
            };

            (key, value)
        })
        .collect()
}
