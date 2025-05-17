use syn::{FnArg, Ident, Pat, Signature, Type};

// Helper function to extract fields from function signature
pub(crate) fn extract_fields(sig: &Signature) -> Vec<(Ident, Type)> {
    sig.inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let ident = pat_ident.ident.clone();
                    let ty = (*pat_type.ty).clone();
                    Some((ident, ty))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

// Helper trait to convert strings to PascalCase
pub(crate) trait ToPascalCase {
    fn to_pascal_case(&self) -> String;
}

impl ToPascalCase for &str {
    fn to_pascal_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in self.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}

impl ToPascalCase for String {
    fn to_pascal_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in self.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}
