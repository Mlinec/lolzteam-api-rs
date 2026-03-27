// openapi -> rust codegen
//
// Usage: cargo run -p lolzteam-codegen -- schemas/forum.json schemas/market.json src

use heck::{ToSnakeCase, ToUpperCamelCase};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

// ── Schema representation ──

#[derive(Debug, Clone)]
struct SchemaField {
    name: String,
    rust_type: String,
    description: Option<String>,
}

#[derive(Debug, Clone)]
struct SchemaModel {
    name: String,
    fields: Vec<SchemaField>,
}

#[derive(Debug, Clone)]
struct Endpoint {
    operation_id: String,
    method: String,
    path: String,
    tag: String,
    summary: Option<String>,
    path_params: Vec<Param>,
    query_params: Vec<Param>,
    body_params: Vec<Param>,
    request_body: Option<RequestBodySpec>,
    response_type: String,
}

#[derive(Debug, Clone)]
struct Param {
    name: String,
    rust_name: String,
    rust_type: String,
    required: bool,
    description: Option<String>,
    is_vec: bool,
    style: Option<String>,
    explode: Option<bool>,
    format: Option<String>,
}

#[derive(Debug, Clone)]
struct RequestBodySpec {
    content_type: String,
    is_multipart: bool,
    is_form: bool,
}

// ── JSON helpers ──

fn resolve_ref<'a>(root: &'a Value, reference: &str) -> &'a Value {
    let path = reference.trim_start_matches("#/");
    let mut current = root;
    for segment in path.split('/') {
        current = &current[segment];
    }
    current
}

fn json_type_to_rust(schema: &Value, root: &Value, nullable: bool) -> String {
    if let Some(r) = schema.get("$ref").and_then(|v| v.as_str()) {
        let type_name = r.rsplit('/').next().unwrap_or("Value");

        let resolved = resolve_ref(root, r);
        let has_props = resolved
            .get("properties")
            .and_then(|p| p.as_object())
            .map(|m| !m.is_empty())
            .unwrap_or(false);

        let rust_name = if has_props {
            sanitize_type_name(type_name)
        } else {
            "serde_json::Value".to_string()
        };

        return if nullable {
            format!("Option<{}>", rust_name)
        } else {
            rust_name
        };
    }

    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
        if let Some(first) = all_of.first() {
            return json_type_to_rust(first, root, nullable);
        }
    }

    if let Some(any_of) = schema.get("anyOf").and_then(|v| v.as_array()) {
        let non_null: Vec<_> = any_of
            .iter()
            .filter(|v| v.get("type").and_then(|t| t.as_str()) != Some("null"))
            .collect();
        if non_null.len() == 1 {
            return json_type_to_rust(non_null[0], root, true);
        }
        if let Some(first) = non_null.first() {
            return json_type_to_rust(first, root, nullable);
        }
    }

    // Handle multiple types in "type" field (OpenAPI 3.1)
    if let Some(types) = schema.get("type").and_then(|v| v.as_array()) {
        let non_null: Vec<_> = types
            .iter()
            .filter_map(|v| v.as_str())
            .filter(|s| *s != "null")
            .collect();
        let is_nullable = types.iter().any(|v| v.as_str() == Some("null"));
        if let Some(first) = non_null.first() {
            let base = match *first {
                "integer" => "i64".to_string(),
                "number" => "f64".to_string(),
                "boolean" => "bool".to_string(),
                "string" => "String".to_string(),
                "array" => {
                    let items = schema.get("items").unwrap_or(&Value::Null);
                    let inner = json_type_to_rust(items, root, false);
                    format!("Vec<{}>", inner)
                }
                "object" => "serde_json::Value".to_string(),
                _ => "serde_json::Value".to_string(),
            };
            return if nullable || is_nullable {
                format!("Option<{}>", base)
            } else {
                base
            };
        }
    }

    let base = match schema.get("type").and_then(|v| v.as_str()) {
        Some("integer") => "i64".to_string(),
        Some("number") => "f64".to_string(),
        Some("boolean") => "bool".to_string(),
        Some("string") => "String".to_string(),
        Some("array") => {
            let items = schema.get("items").unwrap_or(&Value::Null);
            let inner = json_type_to_rust(items, root, false);
            format!("Vec<{}>", inner)
        }
        Some("object") => "serde_json::Value".to_string(),
        Some("null") => return "Option<serde_json::Value>".to_string(),
        _ => "serde_json::Value".to_string(),
    };

    if nullable {
        format!("Option<{}>", base)
    } else {
        base
    }
}

fn sanitize_type_name(name: &str) -> String {
    name.replace("Resp_", "")
        .replace("Model", "")
        .to_upper_camel_case()
}

fn sanitize_field_name(name: &str) -> String {
    // Handle field names starting with a digit
    let name = if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("f_{}", name)
    } else {
        name.to_string()
    };

    let s = name.to_snake_case();
    match s.as_str() {
        "type" => "r#type".to_string(),
        "ref" => "r#ref".to_string(),
        "match" => "r#match".to_string(),
        "mod" => "r#mod".to_string(),
        "self" | "self_" => "self_val".to_string(),
        "super" => "r#super".to_string(),
        "crate" => "r#crate".to_string(),
        _ => s,
    }
}

fn is_vec_type(rust_type: &str) -> bool {
    let inner = rust_type
        .strip_prefix("Option<")
        .and_then(|s| s.strip_suffix('>'))
        .unwrap_or(rust_type);
    inner.starts_with("Vec<")
}

fn param_string(schema: &Value, root: &Value) -> (String, Option<String>) {
    let ty = json_type_to_rust(schema, root, false);
    let format = schema
        .get("format")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    (ty, format)
}

fn request_body_spec(details: &Value) -> Option<RequestBodySpec> {
    let rb = details.get("requestBody")?;
    let content = rb.get("content")?.as_object()?;
    if content.contains_key("multipart/form-data") {
        Some(RequestBodySpec {
            content_type: "multipart/form-data".to_string(),
            is_multipart: true,
            is_form: true,
        })
    } else if content.contains_key("application/x-www-form-urlencoded") {
        Some(RequestBodySpec {
            content_type: "application/x-www-form-urlencoded".to_string(),
            is_multipart: false,
            is_form: true,
        })
    } else if content.contains_key("application/json") {
        Some(RequestBodySpec {
            content_type: "application/json".to_string(),
            is_multipart: false,
            is_form: false,
        })
    } else {
        content.keys().next().map(|ct| RequestBodySpec {
            content_type: ct.clone(),
            is_multipart: ct == "multipart/form-data",
            is_form: ct == "multipart/form-data" || ct == "application/x-www-form-urlencoded",
        })
    }
}

fn param_style(param: &Value) -> (Option<String>, Option<bool>) {
    (
        param
            .get("style")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        param.get("explode").and_then(|v| v.as_bool()),
    )
}

fn schema_is_binary(schema: &Value) -> bool {
    schema.get("format").and_then(|v| v.as_str()) == Some("binary")
}

fn rust_type_for_param(schema: &Value, root: &Value, required: bool) -> String {
    let base = if schema_is_binary(schema) {
        "crate::client::RequestBody".to_string()
    } else {
        json_type_to_rust(schema, root, false)
    };
    if required || base.starts_with("Option<") {
        base
    } else {
        format!("Option<{}>", base)
    }
}

fn request_body_arg(spec: &RequestBodySpec) -> String {
    if spec.is_multipart || spec.is_form {
        "crate::client::RequestBody::Form(body)".to_string()
    } else {
        "crate::client::RequestBody::Json(serde_json::Value::Object(body))".to_string()
    }
}

fn method_name_from_op_id(op_id: &str) -> String {
    op_id.replace('.', "_").to_snake_case()
}

// ── Schema extraction ──

fn extract_schemas(root: &Value) -> Vec<SchemaModel> {
    let mut models = Vec::new();
    let schemas = match root.get("components").and_then(|c| c.get("schemas")) {
        Some(s) => s,
        None => return models,
    };

    let schemas_map = match schemas.as_object() {
        Some(m) => m,
        None => return models,
    };

    for (name, schema) in schemas_map {
        let rust_name = sanitize_type_name(name);
        let props = match schema.get("properties").and_then(|p| p.as_object()) {
            Some(p) => p,
            None => continue,
        };
        if props.is_empty() {
            continue;
        }

        let required_set: BTreeSet<String> = schema
            .get("required")
            .and_then(|r| r.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut fields = Vec::new();
        for (field_name, field_schema) in props {
            let required = required_set.contains(field_name.as_str());
            let mut rust_type = json_type_to_rust(field_schema, root, !required);
            if !required && !rust_type.starts_with("Option<") {
                rust_type = format!("Option<{}>", rust_type);
            }

            let description = field_schema
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            fields.push(SchemaField {
                name: field_name.clone(),
                rust_type,
                description,
            });
        }

        models.push(SchemaModel {
            name: rust_name,
            fields,
        });
    }

    models
}

// ── Endpoint extraction ──

fn extract_endpoints(root: &Value, response_model_names: &BTreeSet<String>) -> Vec<Endpoint> {
    let mut endpoints = Vec::new();
    let paths = match root.get("paths").and_then(|p| p.as_object()) {
        Some(p) => p,
        None => return endpoints,
    };

    for (path, methods) in paths {
        let methods_map = match methods.as_object() {
            Some(m) => m,
            None => continue,
        };

        for (method, details) in methods_map {
            if !["get", "post", "put", "delete", "patch"].contains(&method.as_str()) {
                continue;
            }

            let op_id = match details.get("operationId").and_then(|v| v.as_str()) {
                Some(id) => id.to_string(),
                None => continue,
            };

            let tag = details
                .get("tags")
                .and_then(|t| t.as_array())
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .unwrap_or("Other")
                .to_string();

            let summary = details
                .get("summary")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // Parse parameters
            let mut path_params = Vec::new();
            let mut query_params = Vec::new();

            if let Some(params) = details.get("parameters").and_then(|p| p.as_array()) {
                for param in params {
                    let param = if let Some(r) = param.get("$ref").and_then(|v| v.as_str()) {
                        resolve_ref(root, r)
                    } else {
                        param
                    };

                    let pname = match param.get("name").and_then(|v| v.as_str()) {
                        Some(n) => n.to_string(),
                        None => continue,
                    };

                    let location = param.get("in").and_then(|v| v.as_str()).unwrap_or("query");
                    let required = param
                        .get("required")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(location == "path");

                    let schema = param.get("schema").unwrap_or(&Value::Null);
                    let (base_ty, format) = param_string(schema, root);
                    let rust_type = if required {
                        base_ty.clone()
                    } else if base_ty.starts_with("Option<") {
                        base_ty.clone()
                    } else {
                        format!("Option<{}>", base_ty)
                    };

                    let description = param
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    let is_vec = is_vec_type(&rust_type);
                    let (style, explode) = param_style(param);

                    let p = Param {
                        name: pname.clone(),
                        rust_name: sanitize_field_name(&pname),
                        rust_type,
                        required,
                        description,
                        is_vec,
                        style,
                        explode,
                        format,
                    };

                    if schema_is_binary(schema) {
                        query_params.push(p);
                        continue;
                    }

                    match location {
                        "path" => path_params.push(p),
                        _ => query_params.push(p),
                    }
                }
            }

            // Parse request body
            let mut body_params = Vec::new();

            let request_body = request_body_spec(details);
            if let Some(rb) = details.get("requestBody") {
                let content = rb.get("content").and_then(|c| c.as_object());
                if let Some(content_map) = content {
                    let ct_schema = content_map
                        .get("application/json")
                        .or_else(|| content_map.get("multipart/form-data"))
                        .or_else(|| content_map.get("application/x-www-form-urlencoded"))
                        .or_else(|| content_map.values().next());

                    if let Some(ct_schema) = ct_schema {
                        if let Some(schema) = ct_schema.get("schema") {
                            let schema =
                                if let Some(r) = schema.get("$ref").and_then(|v| v.as_str()) {
                                    resolve_ref(root, r)
                                } else {
                                    schema
                                };

                            let required_set: BTreeSet<String> = schema
                                .get("required")
                                .and_then(|r| r.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect()
                                })
                                .unwrap_or_default();

                            if let Some(props) =
                                schema.get("properties").and_then(|p| p.as_object())
                            {
                                for (pname, pschema) in props {
                                    let required = required_set.contains(pname.as_str());
                                    let (base_ty, format) = param_string(pschema, root);
                                    let rust_type = if required {
                                        base_ty.clone()
                                    } else if base_ty.starts_with("Option<") {
                                        base_ty.clone()
                                    } else {
                                        format!("Option<{}>", base_ty)
                                    };

                                    let description = pschema
                                        .get("description")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());

                                    let is_vec = is_vec_type(&rust_type);
                                    let (style, explode) = param_style(pschema);

                                    body_params.push(Param {
                                        name: pname.clone(),
                                        rust_name: sanitize_field_name(pname),
                                        rust_type,
                                        required,
                                        description,
                                        is_vec,
                                        style,
                                        explode,
                                        format,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            let response_type = extract_response_type(details, root, response_model_names);

            endpoints.push(Endpoint {
                operation_id: op_id,
                method: method.to_uppercase(),
                path: path.clone(),
                tag,
                summary,
                path_params,
                query_params,
                body_params,
                request_body,
                response_type,
            });
        }
    }

    endpoints.sort_by(|a, b| a.operation_id.cmp(&b.operation_id));
    endpoints
}

fn response_type_name_from_op_id(op_id: &str) -> String {
    let base = op_id.replace('.', "_").to_upper_camel_case();
    format!("{}Response", base)
}

fn get_response_schema<'a>(details: &'a Value) -> Option<&'a Value> {
    let responses = details.get("responses")?.as_object()?;
    let resp = responses
        .get("200")
        .or_else(|| responses.get("201"))
        .or_else(|| {
            responses
                .iter()
                .find(|(k, _)| k.starts_with('2'))
                .map(|(_, v)| v)
        })?;
    resp.get("content")
        .and_then(|c| c.get("application/json"))
        .and_then(|j| j.get("schema"))
}

/// Extract response models with inline schemas from all endpoints.
fn extract_response_models(root: &Value) -> Vec<(String, SchemaModel)> {
    let mut models = Vec::new();
    let paths = match root.get("paths").and_then(|p| p.as_object()) {
        Some(p) => p,
        None => return models,
    };

    for (_, methods) in paths {
        let methods_map = match methods.as_object() {
            Some(m) => m,
            None => continue,
        };

        for (method, details) in methods_map {
            if !["get", "post", "put", "delete", "patch"].contains(&method.as_str()) {
                continue;
            }

            let op_id = match details.get("operationId").and_then(|v| v.as_str()) {
                Some(id) => id.to_string(),
                None => continue,
            };

            let schema = match get_response_schema(details) {
                Some(s) => s,
                None => continue,
            };

            // skip $ref responses — handled by component schemas
            if schema.get("$ref").is_some() {
                continue;
            }

            // only inline objects with properties
            let props = match schema.get("properties").and_then(|p| p.as_object()) {
                Some(p) if !p.is_empty() => p,
                _ => continue,
            };

            let struct_name = response_type_name_from_op_id(&op_id);

            let required_set: BTreeSet<String> = schema
                .get("required")
                .and_then(|r| r.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let mut fields = Vec::new();
            for (field_name, field_schema) in props {
                let required = required_set.contains(field_name.as_str());
                let mut rust_type = json_type_to_rust(field_schema, root, !required);
                if !required && !rust_type.starts_with("Option<") {
                    rust_type = format!("Option<{}>", rust_type);
                }

                let description = field_schema
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                fields.push(SchemaField {
                    name: field_name.clone(),
                    rust_type,
                    description,
                });
            }

            models.push((
                op_id,
                SchemaModel {
                    name: struct_name,
                    fields,
                },
            ));
        }
    }

    models
}

fn extract_response_type(
    details: &Value,
    root: &Value,
    response_model_names: &BTreeSet<String>,
) -> String {
    let op_id = match details.get("operationId").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => return "serde_json::Value".to_string(),
    };

    let schema = match get_response_schema(details) {
        Some(s) => s,
        None => return "serde_json::Value".to_string(),
    };

    // $ref → component type
    if schema.get("$ref").is_some() {
        return json_type_to_rust(schema, root, false);
    }

    // generated response model
    let candidate = response_type_name_from_op_id(&op_id);
    if response_model_names.contains(&candidate) {
        return candidate;
    }

    // fallback
    json_type_to_rust(schema, root, false)
}

// ── Code generation ──

fn generate_models(models: &[SchemaModel], response_models: &[SchemaModel]) -> String {
    let mut out = String::new();

    out.push_str(
        "//! Auto-generated response models from the LOLZTEAM OpenAPI specification.\n\
         //!\n\
         //! DO NOT EDIT — regenerate with `cargo run -p lolzteam-codegen`.\n\n\
         #![allow(unused, clippy::all)]\n\n\
         use serde::{Deserialize, Deserializer, Serialize};\n\n\
         /// Deserialize a field that may be `null` or have a mismatched type.\n\
         /// Falls back to `T::default()` on any type mismatch (e.g. `false` for i64, `null` for String).\n\
         fn null_default<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>\n\
         where\n\
         \x20   D: Deserializer<'de>,\n\
         \x20   T: Default + serde::de::DeserializeOwned,\n\
         {\n\
         \x20   let v = serde_json::Value::deserialize(deserializer)?;\n\
         \x20   Ok(serde_json::from_value(v).unwrap_or_default())\n\
         }\n\n\
         /// Deserialize a Vec field that may be null, an array, or an object (takes values).\n\
         /// The LOLZTEAM API sometimes returns `{}` or `{\"key\": val}` instead of `[]`.\n\
         fn null_or_vec<'de, D, T>(deserializer: D) -> std::result::Result<Vec<T>, D::Error>\n\
         where\n\
         \x20   D: Deserializer<'de>,\n\
         \x20   T: serde::de::DeserializeOwned + Default,\n\
         {\n\
         \x20   use serde_json::Value;\n\
         \x20   let v = Value::deserialize(deserializer)?;\n\
         \x20   match v {\n\
         \x20       Value::Array(arr) => {\n\
         \x20           let mut out = Vec::with_capacity(arr.len());\n\
         \x20           for item in arr {\n\
         \x20               out.push(serde_json::from_value(item).unwrap_or_default());\n\
         \x20           }\n\
         \x20           Ok(out)\n\
         \x20       }\n\
         \x20       Value::Object(map) => {\n\
         \x20           let mut out = Vec::with_capacity(map.len());\n\
         \x20           for (_key, item) in map {\n\
         \x20               out.push(serde_json::from_value(item).unwrap_or_default());\n\
         \x20           }\n\
         \x20           Ok(out)\n\
         \x20       }\n\
         \x20       Value::Null => Ok(Vec::new()),\n\
         \x20       _ => Ok(Vec::new()),\n\
         \x20   }\n\
         }\n\n",
    );

    // component schema models
    for model in models {
        emit_model_struct(&mut out, model);
    }

    // ItemList.items custom deserializer
    out.push_str(
        "// API sometimes returns items as array, sometimes as object {id: item}\n\
         fn deserialize_items<'de, D>(deserializer: D) -> std::result::Result<Vec<ItemFromList>, D::Error>\n\
         where\n\
         \x20   D: serde::Deserializer<'de>,\n\
         {\n\
         \x20   use serde::de;\n\
         \x20   use serde_json::Value;\n\n\
         \x20   let v = Value::deserialize(deserializer)?;\n\
         \x20   match v {\n\
         \x20       Value::Array(arr) => {\n\
         \x20           let mut out = Vec::with_capacity(arr.len());\n\
         \x20           for item in arr {\n\
         \x20               out.push(serde_json::from_value(item).unwrap_or_default());\n\
         \x20           }\n\
         \x20           Ok(out)\n\
         \x20       }\n\
         \x20       Value::Object(map) => {\n\
         \x20           let mut out = Vec::with_capacity(map.len());\n\
         \x20           for (_key, item) in map {\n\
         \x20               out.push(serde_json::from_value(item).unwrap_or_default());\n\
         \x20           }\n\
         \x20           Ok(out)\n\
         \x20       }\n\
         \x20       Value::Null => Ok(Vec::new()),\n\
         \x20       _ => Err(de::Error::custom(\"expected array or object for items\")),\n\
         \x20   }\n\
         }\n\n",
    );

    // Emit response wrapper models
    if !response_models.is_empty() {
        out.push_str("// ── Response wrappers ──\n\n");
        for model in response_models {
            emit_model_struct(&mut out, model);
        }
    }

    out
}

fn emit_model_struct(out: &mut String, model: &SchemaModel) {
    out.push_str(&format!(
        "/// {} model from the LOLZTEAM API.\n",
        model.name
    ));
    out.push_str(
        "#[derive(Debug, Clone, Serialize, Deserialize, Default)]\n\
         #[serde(default)]\n",
    );
    out.push_str(&format!("pub struct {} {{\n", model.name));

    for field in &model.fields {
        if let Some(desc) = &field.description {
            for line in desc.lines() {
                out.push_str(&format!("    /// {}\n", line));
            }
        }
        let rust_name = sanitize_field_name(&field.name);
        let raw_rust_name = rust_name.strip_prefix("r#").unwrap_or(&rust_name);
        if raw_rust_name != field.name {
            out.push_str(&format!("    #[serde(rename = \"{}\")]\n", field.name));
        }
        // Special handling: ItemList.items needs custom deserializer
        if model.name == "ItemList" && field.name == "items" {
            out.push_str("    #[serde(deserialize_with = \"deserialize_items\", default)]\n");
        } else if !field.rust_type.starts_with("Option<") {
            // Vec fields: API may return object instead of array
            if field.rust_type.starts_with("Vec<") {
                out.push_str("    #[serde(deserialize_with = \"null_or_vec\", default)]\n");
            } else {
                // API may return explicit null for non-Option fields — treat as default
                out.push_str("    #[serde(deserialize_with = \"null_default\", default)]\n");
            }
        }
        out.push_str(&format!("    pub {}: {},\n", rust_name, field.rust_type));
    }

    out.push_str("}\n\n");
}

fn generate_param_types(endpoints: &[Endpoint], prefix: &str) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "//! Auto-generated types for {} API.\n\
         //! DO NOT EDIT — regenerate with `cargo run -p lolzteam-codegen`.\n\n\
         #![allow(unused, clippy::all)]\n\n\
         use serde::{{Deserialize, Serialize}};\n\
         use crate::models::*;\n\n",
        prefix.to_upper_camel_case()
    ));

    for ep in endpoints {
        let all_params: Vec<&Param> = ep
            .query_params
            .iter()
            .chain(ep.body_params.iter())
            .collect();

        if all_params.len() > 3 {
            let struct_name = format!(
                "{}{}Params",
                prefix.to_upper_camel_case(),
                method_name_from_op_id(&ep.operation_id).to_upper_camel_case()
            );

            out.push_str(&format!(
                "/// Parameters for `{}` ({} {}).\n",
                ep.operation_id, ep.method, ep.path
            ));
            out.push_str(
                "#[derive(Debug, Clone, Serialize, Deserialize, Default)]\n\
                 #[serde(default)]\n",
            );
            out.push_str(&format!("pub struct {} {{\n", struct_name));

            for p in &all_params {
                if let Some(desc) = &p.description {
                    for line in desc.lines() {
                        out.push_str(&format!("    /// {}\n", line));
                    }
                }
                let raw_name = p.rust_name.strip_prefix("r#").unwrap_or(&p.rust_name);
                if raw_name != p.name {
                    out.push_str(&format!("    #[serde(rename = \"{}\")]\n", p.name));
                }
                if !p.required {
                    out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                }
                out.push_str(&format!("    pub {}: {},\n", p.rust_name, p.rust_type));
            }

            out.push_str("}\n\n");
        }
    }

    out
}

fn generate_api_methods(endpoints: &[Endpoint], prefix: &str) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "//! Auto-generated API methods for LOLZTEAM {} API.\n\
         //!\n\
         //! DO NOT EDIT — regenerate with `cargo run -p lolzteam-codegen`.\n\n\
         #![allow(unused, clippy::all)]\n\n\
         use crate::client::ApiClient;\n\
         use crate::error::Result;\n\
         use crate::models::*;\n\
         use crate::{}::types::*;\n\n",
        prefix.to_upper_camel_case(),
        prefix,
    ));

    // Group by tag
    let mut by_tag: BTreeMap<String, Vec<&Endpoint>> = BTreeMap::new();
    for ep in endpoints {
        by_tag.entry(ep.tag.clone()).or_default().push(ep);
    }

    out.push_str(&format!(
        "/// {} API methods.\n",
        prefix.to_upper_camel_case()
    ));
    out.push_str(&format!(
        "impl crate::{}::{}Api {{\n",
        prefix,
        prefix.to_upper_camel_case()
    ));

    for (tag, eps) in &by_tag {
        out.push_str(&format!("\n    // ── {} ──\n\n", tag));

        for ep in eps {
            generate_method(&mut out, ep, prefix);
        }
    }

    out.push_str("}\n");
    out
}

fn generate_method(out: &mut String, ep: &Endpoint, prefix: &str) {
    let fn_name = method_name_from_op_id(&ep.operation_id);
    let all_optional_params: Vec<&Param> = ep
        .query_params
        .iter()
        .chain(ep.body_params.iter())
        .collect();

    let use_params_struct = all_optional_params.len() > 3;
    let params_struct_name = format!(
        "{}{}Params",
        prefix.to_upper_camel_case(),
        fn_name.to_upper_camel_case()
    );

    // doc comment
    if let Some(summary) = &ep.summary {
        out.push_str(&format!("    /// {}\n", summary));
    }
    out.push_str(&format!("    /// `{} {}`\n", ep.method, ep.path));

    // Function signature
    out.push_str(&format!("    pub async fn {}(\n        &self,\n", fn_name));

    // Path params
    for p in &ep.path_params {
        let ty = simplify_path_param_type(&p.rust_type);
        out.push_str(&format!("        {}: {},\n", p.rust_name, ty));
    }

    if use_params_struct {
        out.push_str(&format!("        params: {},\n", params_struct_name));
    } else {
        let mut required: Vec<&Param> = Vec::new();
        let mut optional: Vec<&Param> = Vec::new();
        for p in &all_optional_params {
            if p.required {
                required.push(p);
            } else {
                optional.push(p);
            }
        }
        for p in required.iter().chain(optional.iter()) {
            out.push_str(&format!("        {}: {},\n", p.rust_name, p.rust_type));
        }
    }

    out.push_str(&format!("    ) -> Result<{}> {{\n", ep.response_type));

    // url
    for p in &ep.path_params {
        if p.rust_name.starts_with("r#") {
            let bare = p.rust_name.strip_prefix("r#").unwrap();
            out.push_str(&format!(
                "        let _path_{bare} = {raw};\n",
                bare = bare,
                raw = p.rust_name,
            ));
        }
    }
    let url_expr = if !ep.path_params.is_empty() {
        let mut fmt_str = ep.path.clone();
        for p in &ep.path_params {
            let placeholder = format!("{{{}}}", p.name);
            if p.rust_name.starts_with("r#") {
                let bare = p.rust_name.strip_prefix("r#").unwrap();
                let alias = format!("_path_{}", bare);
                fmt_str = fmt_str.replace(&placeholder, &format!("{{{}}}", alias));
            } else {
                let rust_placeholder = format!("{{{}}}", p.rust_name);
                fmt_str = fmt_str.replace(&placeholder, &rust_placeholder);
            }
        }
        format!("&format!(\"{}\")", fmt_str)
    } else {
        format!("\"{}\"", ep.path)
    };

    // query
    if !ep.query_params.is_empty() {
        out.push_str("        let mut query: Vec<(&str, String)> = Vec::new();\n");
        for p in &ep.query_params {
            let accessor = if use_params_struct {
                format!("params.{}", p.rust_name)
            } else {
                p.rust_name.clone()
            };
            let explode = p.explode.unwrap_or(true);
            if p.required {
                if p.is_vec {
                    if explode {
                        out.push_str(&format!(
                            "        for item in &{} {{ query.push((\"{}\", item.to_string())); }}\n",
                            accessor, p.name
                        ));
                    } else {
                        out.push_str(&format!(
                            "        query.push((\"{}\", {}.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(\",\")));\n",
                            p.name, accessor
                        ));
                    }
                } else {
                    out.push_str(&format!(
                        "        query.push((\"{}\", {}.to_string()));\n",
                        p.name, accessor
                    ));
                }
            } else if p.is_vec {
                if explode {
                    out.push_str(&format!(
                        "        if let Some(v) = &{} {{ for item in v {{ query.push((\"{}\", item.to_string())); }} }}\n",
                        accessor, p.name
                    ));
                } else {
                    out.push_str(&format!(
                        "        if let Some(v) = &{} {{ query.push((\"{}\", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(\",\"))); }}\n",
                        accessor, p.name
                    ));
                }
            } else {
                out.push_str(&format!(
                    "        if let Some(v) = &{} {{ query.push((\"{}\", v.to_string())); }}\n",
                    accessor, p.name
                ));
            }
        }
    }

    // body
    let has_body =
        !ep.body_params.is_empty() && matches!(ep.method.as_str(), "POST" | "PUT" | "PATCH");

    if has_body {
        if let Some(spec) = &ep.request_body {
            if spec.is_multipart || spec.is_form {
                out.push_str("        let mut body = Vec::<(String, String)>::new();\n");
                for p in &ep.body_params {
                    let accessor = if use_params_struct {
                        format!("params.{}", p.rust_name)
                    } else {
                        p.rust_name.clone()
                    };
                    if p.required {
                        out.push_str(&format!(
                            "        body.push((\"{}\".into(), {}.to_string()));\n",
                            p.name, accessor
                        ));
                    } else {
                        out.push_str(&format!(
                            "        if let Some(v) = &{} {{ body.push((\"{}\".into(), v.to_string())); }}\n",
                            accessor, p.name
                        ));
                    }
                }
            } else {
                out.push_str("        let mut body = serde_json::Map::new();\n");
                for p in &ep.body_params {
                    let accessor = if use_params_struct {
                        format!("params.{}", p.rust_name)
                    } else {
                        p.rust_name.clone()
                    };
                    if p.required {
                        out.push_str(&format!(
                            "        body.insert(\"{}\".into(), serde_json::to_value(&{}).unwrap_or_default());\n",
                            p.name, accessor
                        ));
                    } else {
                        out.push_str(&format!(
                            "        if let Some(v) = &{} {{ body.insert(\"{}\".into(), serde_json::to_value(v).unwrap_or_default()); }}\n",
                            accessor, p.name
                        ));
                    }
                }
            }
        }
    }

    // request
    let method_lower = ep.method.to_lowercase();
    out.push_str(&format!(
        "        self.client.request(\n\
         \x20           \"{}\",\n\
         \x20           {},\n",
        method_lower, url_expr
    ));

    if !ep.query_params.is_empty() {
        out.push_str("            Some(&query),\n");
    } else {
        out.push_str("            None::<&[(&str, String)]>,\n");
    }

    if has_body {
        if let Some(spec) = &ep.request_body {
            if spec.is_multipart || spec.is_form {
                out.push_str("            Some(crate::client::RequestBody::Form(body)),\n");
            } else {
                out.push_str("            Some(crate::client::RequestBody::Json(serde_json::Value::Object(body))),\n");
            }
        } else {
            out.push_str("            None::<crate::client::RequestBody>,\n");
        }
    } else {
        out.push_str("            None::<crate::client::RequestBody>,\n");
    }

    out.push_str("        ).await\n");
    out.push_str("    }\n\n");
}

/// Path params → simple types (i64 or &str).
fn simplify_path_param_type(t: &str) -> &str {
    match t {
        "i64" | "String" | "bool" | "f64" => t,
        _ if t.starts_with("Option<") => t,
        _ => "i64", // most path params are IDs
    }
}

// ── Main ──

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <forum.json> <market.json> <output_dir>", args[0]);
        std::process::exit(1);
    }

    let forum_path = &args[1];
    let market_path = &args[2];
    let output_dir = &args[3];

    let forum_raw = fs::read_to_string(forum_path).expect("Failed to read forum.json");
    let forum: Value = serde_json::from_str(&forum_raw).expect("Failed to parse forum.json");

    let market_raw = fs::read_to_string(market_path).expect("Failed to read market.json");
    let market: Value = serde_json::from_str(&market_raw).expect("Failed to parse market.json");

    // Extract component schema models
    let mut all_models = extract_schemas(&forum);
    let market_models = extract_schemas(&market);

    let mut model_names: BTreeSet<String> = all_models.iter().map(|m| m.name.clone()).collect();
    for m in market_models {
        if !model_names.contains(&m.name) {
            model_names.insert(m.name.clone());
            all_models.push(m);
        }
    }

    // Extract response wrapper models from inline response schemas
    let mut all_response_models_raw = extract_response_models(&forum);
    all_response_models_raw.extend(extract_response_models(&market));

    // Deduplicate response models by name
    let mut response_model_names: BTreeSet<String> = BTreeSet::new();
    let mut all_response_models: Vec<SchemaModel> = Vec::new();
    for (_op_id, model) in all_response_models_raw {
        if !response_model_names.contains(&model.name) && !model_names.contains(&model.name) {
            response_model_names.insert(model.name.clone());
            all_response_models.push(model);
        }
    }

    let forum_endpoints = extract_endpoints(&forum, &response_model_names);
    let market_endpoints = extract_endpoints(&market, &response_model_names);

    let out = std::path::Path::new(output_dir);
    fs::create_dir_all(out.join("forum")).unwrap();
    fs::create_dir_all(out.join("market")).unwrap();

    // models.rs
    let models_code = generate_models(&all_models, &all_response_models);
    fs::write(out.join("models.rs"), &models_code).unwrap();
    eprintln!(
        "  ✓ models.rs ({} component + {} response = {} types)",
        all_models.len(),
        all_response_models.len(),
        all_models.len() + all_response_models.len()
    );

    // forum
    let forum_types = generate_param_types(&forum_endpoints, "forum");
    fs::write(out.join("forum/types.rs"), &forum_types).unwrap();
    eprintln!("  ✓ forum/types.rs");

    let forum_methods = generate_api_methods(&forum_endpoints, "forum");
    fs::write(out.join("forum/methods.rs"), &forum_methods).unwrap();
    eprintln!("  ✓ forum/methods.rs ({} endpoints)", forum_endpoints.len());

    // market
    let market_types = generate_param_types(&market_endpoints, "market");
    fs::write(out.join("market/types.rs"), &market_types).unwrap();
    eprintln!("  ✓ market/types.rs");

    let market_methods = generate_api_methods(&market_endpoints, "market");
    fs::write(out.join("market/methods.rs"), &market_methods).unwrap();
    eprintln!(
        "  ✓ market/methods.rs ({} endpoints)",
        market_endpoints.len()
    );

    // mod.rs files
    let forum_mod = "pub mod methods;\npub mod types;\n\n\
         use crate::client::ApiClient;\n\n\
         /// Forum API wrapper.\n\
         pub struct ForumApi {\n\
         \x20   pub(crate) client: ApiClient,\n\
         }\n\n\
         impl ForumApi {\n\
         \x20   pub fn new(client: ApiClient) -> Self {\n\
         \x20       Self { client }\n\
         \x20   }\n\
         }\n";
    fs::write(out.join("forum/mod.rs"), forum_mod).unwrap();

    let market_mod = "pub mod methods;\npub mod types;\n\n\
         use crate::client::ApiClient;\n\n\
         /// Market API wrapper.\n\
         pub struct MarketApi {\n\
         \x20   pub(crate) client: ApiClient,\n\
         }\n\n\
         impl MarketApi {\n\
         \x20   pub fn new(client: ApiClient) -> Self {\n\
         \x20       Self { client }\n\
         \x20   }\n\
         }\n";
    fs::write(out.join("market/mod.rs"), market_mod).unwrap();

    eprintln!("\n✅ Code generation complete!");
}
