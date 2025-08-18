use std::env;
use std::fs;
use std::path::Path;
use serde_json::Value;
use typify::{TypeSpace, TypeSpaceSettings};
use schemars::schema::Schema;

pub type BuildResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Generate schemas from TOML configuration
pub fn generate_schemas_from_config() -> BuildResult<()> {
    println!("cargo:rerun-if-changed=schemas.toml");
    println!("cargo:rerun-if-changed=schemas/");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR")?;
    let generated_dir = Path::new(&out_dir).join("generated");
    fs::create_dir_all(&generated_dir)?;

    let config_path = Path::new("schemas.toml");
    if !config_path.exists() {
        return create_empty_schemas_module(&generated_dir);
    }

    let config_content = fs::read_to_string(config_path)?;
    let config: toml::Value = toml::from_str(&config_content)?;

    let schemas_table = match config.get("schemas") {
        Some(toml::Value::Table(table)) => table,
        _ => return create_empty_schemas_module(&generated_dir),
    };

    let mut generated_any = false;

    for (_schema_name, schema_config) in schemas_table {
        if let toml::Value::Table(config_table) = schema_config {
            if let Some(toml::Value::Array(patterns)) = config_table.get("include_patterns") {
                for pattern in patterns {
                    if let toml::Value::String(pattern_str) = pattern {
                        if let Ok(paths) = glob::glob(pattern_str) {
                            for schema_path in paths.flatten() {
                                println!("Processing schema: {}", schema_path.display());
                                
                                let schema_content = fs::read_to_string(&schema_path)?;
                                let resolved_schema = resolve_schema_references(&schema_content, &schema_path)?;
                                
                                let raw_name = schema_path.file_stem().unwrap().to_str().unwrap();
                                let type_name = sanitize_type_name(raw_name);
                                let module_name = &type_name;
                                generate_rust_types(&resolved_schema, &type_name, module_name, &generated_dir)?;
                                generated_any = true;
                            }
                        }
                    }
                }
            }
        }
    }

    if generated_any {
        create_schemas_module(&generated_dir)?;
    } else {
        create_empty_schemas_module(&generated_dir)?;
    }

    Ok(())
}

#[allow(dead_code)]
fn get_type_name_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(sanitize_type_name)
        .unwrap_or_else(|| "UnknownType".to_string())
}

fn sanitize_type_name(name: &str) -> String {
    let sanitized = name.replace("-", "_").replace(" ", "_");
    match sanitized.as_str() {
        "macro" => "macro_def".to_string(),
        "type" => "type_def".to_string(),
        "mod" => "mod_def".to_string(),
        _ => sanitized,
    }
}

fn resolve_schema_references(schema_content: &str, schema_path: &Path) -> BuildResult<Value> {
    let mut schema_value: Value = serde_json::from_str(schema_content)?;
    
    // Remove external URLs from $id to avoid external fetching
    if let Value::Object(ref mut obj) = schema_value {
        if let Some(Value::String(id_str)) = obj.get("$id") {
            if id_str.starts_with("http") {
                obj.remove("$id");
            }
        }
    }
    
    // Resolve external file references
    let schema_root = schema_path.parent().ok_or("Invalid schema path")?;
    resolve_external_refs(&mut schema_value, schema_root)?;
    
    Ok(schema_value)
}

fn resolve_external_refs(value: &mut Value, schema_dir: &Path) -> BuildResult<()> {
    match value {
        Value::Object(obj) => {
            if let Some(Value::String(ref_str)) = obj.get("$ref") {
                if ref_str.starts_with("./") || ref_str.starts_with("../") {
                    let (file_part, pointer_part) = if let Some(hash_idx) = ref_str.find('#') {
                        (&ref_str[..hash_idx], Some(&ref_str[hash_idx + 1..]))
                    } else {
                        (ref_str.as_str(), None)
                    };
                    
                    let ref_path = schema_dir.join(file_part);
                    
                    if ref_path.exists() {
                        let file_content = fs::read_to_string(&ref_path)?;
                        let mut ref_value: Value = serde_json::from_str(&file_content)?;
                        
                        if let Value::Object(ref mut ref_obj) = ref_value {
                            if let Some(Value::String(id_str)) = ref_obj.get("$id") {
                                if id_str.starts_with("http") {
                                    ref_obj.remove("$id");
                                }
                            }
                        }
                        
                        let ref_schema_dir = ref_path.parent().ok_or("Invalid reference path")?;
                        resolve_external_refs(&mut ref_value, ref_schema_dir)?;
                        
                        if let Some(pointer) = pointer_part {
                            if let Some(pointed_value) = follow_json_pointer(&ref_value, pointer) {
                                *value = pointed_value.clone();
                            } else {
                                *value = ref_value;
                            }
                        } else {
                            *value = ref_value;
                        }
                        return Ok(());
                    }
                }
            }
            
            for (_, v) in obj.iter_mut() {
                resolve_external_refs(v, schema_dir)?;
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                resolve_external_refs(item, schema_dir)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn follow_json_pointer<'a>(value: &'a Value, pointer: &str) -> Option<&'a Value> {
    if pointer.is_empty() || pointer == "/" {
        return Some(value);
    }
    
    let parts: Vec<&str> = pointer.split('/').skip(1).collect();
    let mut current = value;
    
    for part in parts {
        match current {
            Value::Object(obj) => {
                current = obj.get(part)?;
            }
            Value::Array(arr) => {
                let index: usize = part.parse().ok()?;
                current = arr.get(index)?;
            }
            _ => return None,
        }
    }
    
    Some(current)
}

fn generate_rust_types(resolved_schema: &Value, type_name: &str, _module_name: &str, output_dir: &Path) -> BuildResult<()> {
    let mut settings = TypeSpaceSettings::default();
    settings.with_struct_builder(true);
    let mut type_space = TypeSpace::new(&settings);
    
    let schema: Schema = serde_json::from_value(resolved_schema.clone())?;
    type_space.add_type_with_name(&schema, Some(type_name.to_string()))?;
    
    let raw_code = format!("{}", type_space.to_stream());
    
    let mut code = String::new();
    code.push_str("// Auto-generated types from JSON schema\n");
    code.push_str("#[allow(unused_imports)] // Will be used by mappings\n");
    code.push_str("#[allow(clippy::clone_on_copy)] // Generated code patterns\n"); 
    code.push_str("#[allow(clippy::uninlined_format_args)] // Generated code patterns\n\n");
    
    // Add serde imports - unused import warnings will be resolved when mappings use them
    if raw_code.contains("Deserialize") || raw_code.contains("Serialize") {
        code.push_str("use serde::{Deserialize, Serialize};\n\n");
    }
    
    code.push_str(&raw_code);
    
    let formatted_code = format_rust_code(&code)?;
    let sanitized_filename = sanitize_type_name(type_name);
    let output_file = output_dir.join(format!("{sanitized_filename}.rs"));
    fs::write(&output_file, formatted_code)?;
    
    Ok(())
}

fn format_rust_code(code: &str) -> BuildResult<String> {
    match syn::parse_file(code) {
        Ok(parsed_file) => {
            let formatted = prettyplease::unparse(&parsed_file);
            Ok(formatted)
        }
        Err(_) => Ok(code.to_string()),
    }
}

fn create_empty_schemas_module(generated_dir: &Path) -> BuildResult<()> {
    let mod_file = generated_dir.join("mod.rs");
    let content = "// No schemas found - empty module\n";
    fs::write(&mod_file, content)?;
    Ok(())
}

fn create_schemas_module(generated_dir: &Path) -> BuildResult<()> {
    let mod_file_path = generated_dir.join("mod.rs");
    let mut mod_file_content = String::new();
    mod_file_content.push_str("// Auto-generated schema module\n");
    mod_file_content.push_str("// Generated types are accessed via qualified paths: crate::generated::module::Type\n\n");
    
    if let Ok(entries) = fs::read_dir(generated_dir) {
        let mut type_files = Vec::new();
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map(|s| s == "rs").unwrap_or(false) {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem != "mod" {
                        type_files.push(stem.to_string());
                    }
                }
            }
        }
        
        if !type_files.is_empty() {
            type_files.sort();
            
            // Collect unique sanitized module names to avoid duplicates
            let mut unique_modules = std::collections::HashSet::new();
            for type_file in &type_files {
                let sanitized_name = sanitize_type_name(type_file);
                unique_modules.insert(sanitized_name);
            }
            
            // Convert to sorted vector for consistent output
            let mut sorted_modules: Vec<_> = unique_modules.into_iter().collect();
            sorted_modules.sort();
            
            // Only declare modules - no re-exports to avoid naming conflicts
            for module_name in &sorted_modules {
                mod_file_content.push_str(&format!("pub mod {module_name};\n"));
            }
        }
    }
    
    fs::write(&mod_file_path, mod_file_content)?;
    
    // Generate documentation
    generate_schema_docs(generated_dir.parent().unwrap())?;
    
    Ok(())
}

fn generate_schema_docs(out_dir: &Path) -> BuildResult<()> {
    let docs_dir = out_dir.join("docs");
    fs::create_dir_all(&docs_dir)?;
    
    let generated_dir = out_dir.join("generated");
    let mut modules = Vec::new();
    let mut all_types = Vec::new();
    
    if generated_dir.exists() {
        if let Ok(entries) = fs::read_dir(&generated_dir) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map(|s| s == "rs").unwrap_or(false) {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if stem != "mod" {
                            modules.push(stem.to_string());
                            
                            if let Ok(content) = fs::read_to_string(&path) {
                                for line in content.lines() {
                                    if line.trim().starts_with("pub struct ") {
                                        if let Some(type_name) = extract_struct_name(line) {
                                            all_types.push((stem.to_string(), type_name));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    modules.sort();
    all_types.sort();
    
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("    <meta charset=\"utf-8\">\n");
    html.push_str("    <title>Generated Schema Documentation</title>\n");
    html.push_str("    <style>\n");
    html.push_str("        body { font-family: Arial, sans-serif; margin: 40px; }\n");
    html.push_str("        .module { margin: 20px 0; padding: 15px; border: 1px solid #ddd; }\n");
    html.push_str("        h2 { color: #0066cc; }\n");
    html.push_str("        code { background: #f5f5f5; padding: 2px 4px; }\n");
    html.push_str("        pre { background: #f5f5f5; padding: 10px; border-radius: 4px; }\n");
    html.push_str("        .type-list { margin: 10px 0; font-family: monospace; }\n");
    html.push_str("    </style>\n</head>\n<body>\n");
    html.push_str("    <h1>Generated Schema Documentation</h1>\n\n");
    
    if modules.is_empty() {
        html.push_str("    <p>No schemas found - empty module</p>\n");
    } else {
        for module in &modules {
            html.push_str(&format!("    <div class=\"module\">\n        <h2>{module}</h2>\n"));
            html.push_str(&format!("        <code>use crate::schemas::{module}::*;</code>\n"));
            
            let module_types: Vec<_> = all_types.iter()
                .filter(|(m, _)| m == module)
                .map(|(_, t)| t)
                .collect();
                
            if !module_types.is_empty() {
                html.push_str("        <div class=\"type-list\">\n            <strong>Types:</strong> ");
                let type_names: Vec<String> = module_types.iter().map(|s| s.to_string()).collect();
                html.push_str(&type_names.join(", "));
                html.push_str("\n        </div>\n");
            }
            
            html.push_str("    </div>\n\n");
        }
        
        html.push_str("    <h2>Usage Example</h2>\n    <pre><code>use crate::schemas::*;\n\n");
        if let Some((_, first_type)) = all_types.first() {
            html.push_str(&format!("// Example using generated type\nlet example = {first_type}::default();\n"));
        }
        html.push_str("</code></pre>\n");
    }
    
    html.push_str("</body>\n</html>");
    
    let index_file = docs_dir.join("index.html");
    fs::write(&index_file, html)?;
    
    Ok(())
}

fn extract_struct_name(line: &str) -> Option<String> {
    let line = line.trim();
    if let Some(start) = line.find("pub struct ") {
        let after_struct = &line[start + 11..];
        if let Some(end) = after_struct.find(|c: char| c.is_whitespace() || c == '<' || c == '{') {
            Some(after_struct[..end].to_string())
        } else {
            Some(after_struct.to_string())
        }
    } else {
        None
    }
}

