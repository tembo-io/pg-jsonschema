use boon::{CompileError, Compiler, Schemas};
use pgrx::prelude::*;
use serde_json::Value;

pgrx::pg_module_magic!();

// id_for returns the schema `$id` for $x, falling back on "schema.json" if $x
// has no `$id`.
macro_rules! id_for {
    ($x:expr) => {
        if let Value::String(s) = &$x["$id"] {
            s
        } else {
            "schema.json"
        }
    };
}

// Converts schemas from `pgrx::Array<_>` to `Vec<serde_json::Value>` and
// returns the result. Used by the variadic functions.
macro_rules! values_for {
    ($x:expr) => {
        $x.iter_deny_null().map(|x| x.0).collect::<Vec<_>>()
    };
}

// pg_jsonschema-compatible functions.
#[pg_extern(immutable, strict)]
fn json_matches_schema(schema: pgrx::Json, instance: pgrx::Json) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, instance.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

#[pg_extern(immutable, strict)]
fn jsonb_matches_schema(schema: pgrx::Json, instance: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, instance.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

// Schema validation functions.

// jsonschema_is_valid(schema::json)
// jsonschema_is_valid(schema::jsonb)
// jsonschema_is_valid(id::text, VARIADIC schema::json)
// jsonschema_is_valid(id::text, VARIADIC schema::jsonb)

/// json_schema_is_valid validates `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_is_valid(schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    match compiles(id_for!(&schemas[0]), &schemas) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_is_valid validates `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_is_valid(schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    match compiles(id_for!(&schemas[0]), &schemas) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// json_schema_id_is_valid validates the schema with the `$id` `id` from the
/// `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::Json>) -> bool {
    let schemas = values_for!(schemas);
    match compiles(id, &schemas) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_id_is_valid validates the schema with the `$id` `id` from the
/// `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::JsonB>) -> bool {
    let schemas = values_for!(schemas);
    match compiles(id, &schemas) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

// Document validation functions.

// jsonschema_validates(doc::json,  schema::json)
// jsonschema_validates(doc::jsonb, schema::jsonb)
// jsonschema_validates(doc::json,  schema::jsonb)
// jsonschema_validates(doc::jsonb, schema::json)

/// json_schema_validates_json validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_json(json: pgrx::Json, schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_validates_jsonb validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_jsonb(json: pgrx::JsonB, schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// json_schema_validates_jsonb validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_jsonb(json: pgrx::Json, schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_validates_json validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_json(json: pgrx::JsonB, schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

// jsonschema_validates(doc::json,  id::text, VARIADIC schema::json)
// jsonschema_validates(doc::jsonb, id::text, VARIADIC schema::jsonb)
// jsonschema_validates(doc::json,  id::text, VARIADIC schema::jsonb)
// jsonschema_validates(doc::jsonb, id::text, VARIADIC schema::json)

/// json_schema_id_validates_json validates `json` against the schema with the
/// `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_id_validates_json(
    json: pgrx::Json,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::Json>,
) -> bool {
    let schemas = values_for!(schemas);
    match validate(id, &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_id_validates_jsonb validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_id_validates_jsonb(
    json: pgrx::JsonB,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::JsonB>,
) -> bool {
    let schemas = values_for!(schemas);
    match validate(id, &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// json_schema_id_validates_jsonb validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_id_validates_jsonb(
    json: pgrx::Json,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::JsonB>,
) -> bool {
    let schemas = values_for!(schemas);
    match validate(id, &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_id_validates_json validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_id_validates_json(
    json: pgrx::JsonB,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::Json>,
) -> bool {
    let schemas = values_for!(schemas);
    match validate(id, &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// new_compiler creates and returns a new `boon::Compiler` loaded with
/// `schemas`. Each schema in `schemas` is named for its `$id` field or, if it
/// has none, `id` is used for the first schema, and `"{id}{i}"` for
/// subsequent schemas.
fn new_compiler(id: &str, schemas: &[Value]) -> Result<Compiler, CompileError> {
    let mut compiler = Compiler::new();

    for (i, s) in schemas.iter().enumerate() {
        let sid = if let Value::String(s) = &s["$id"] {
            s.to_string()
        } else if i == 0 {
            // Use id for the first item.
            id.to_string()
        } else {
            // Use loc{i} for others.
            format!("{id}{i}")
        };

        compiler.add_resource(&sid, s.to_owned())?;
    }

    Ok(compiler)
}

/// compiles compiles the schema named `id` in `schemas`, returning `true` on
/// success and `false` on failure.
fn compiles(id: &str, schemas: &[Value]) -> Result<bool, CompileError> {
    if let Ok(mut c) = new_compiler(id, schemas) {
        let mut schemas = Schemas::new();
        c.compile(id, &mut schemas)?;
        return Ok(true);
    }

    Ok(false)
}

/// validate validates `instance` against schema `id` in `schemas`.
pub fn validate(id: &str, schemas: &[Value], instance: Value) -> Result<bool, CompileError> {
    match new_compiler(id, schemas) {
        Err(e) => Err(e),
        Ok(mut c) => {
            let mut schemas = Schemas::new();
            match c.compile(id, &mut schemas) {
                Err(e) => Err(e),
                Ok(index) => {
                    if let Err(e) = schemas.validate(&instance, index) {
                        info!("{e}");
                        return Ok(false);
                    }
                    Ok(true)
                }
            }
        }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use pgrx::{Json, JsonB};
    use serde_json::json;
    use std::{env, error::Error, fs::File, path::Path};

    fn load_json(name: &str) -> Value {
        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let root = Path::new(&root_dir);
        serde_json::from_reader(File::open(root.join("schemas").join(name)).unwrap()).unwrap()
    }

    #[pg_test]
    fn test_compiles() -> Result<(), Box<dyn Error>> {
        let address = load_json("address.schema.json");
        let user = load_json("user-profile.schema.json");

        assert!(compiles(
            "https://example.com/address.schema.json",
            &[address.clone(), user.clone()]
        )
        .unwrap());
        assert!(compiles(
            "https://example.com/user-profile.schema.json",
            &[address.clone(), user.clone()]
        )
        .unwrap());
        assert!(compiles(
            "https://example.com/nonesuch.schema.json",
            &[address.clone(), user.clone()]
        )
        .is_err());

        Ok(())
    }

    #[pg_test]
    fn test_new_compiler() -> Result<(), Box<dyn Error>> {
        let address = load_json("address.schema.json");
        let user = load_json("user-profile.schema.json");
        let id = String::from("https://example.com/user-profile.schema.json");
        let c = new_compiler(&id, &[address.clone(), user.clone()]);
        assert!(c.is_ok());

        // Make sure it compiles user and address.
        let mut c = c.unwrap();
        let mut schemas = Schemas::new();
        c.compile(&id, &mut schemas)?;

        let mut schemas = Schemas::new();
        assert!(c
            .compile("https://example.com/address.schema.json", &mut schemas)
            .is_ok());

        // Try some without IDs
        let id = String::from("file:test.json");
        let c = new_compiler(&id, &[json!({"type": "object"}), json!({"type": "array"})]);
        assert!(c.is_ok());

        // It should have used the id.
        let mut c = c.unwrap();
        let mut schemas = Schemas::new();
        assert!(c.compile(&id, &mut schemas).is_ok());

        // And appended "1" to the second schema.
        let mut schemas = Schemas::new();
        assert!(c.compile("file:test.json1", &mut schemas).is_ok());

        // But no more.
        let mut schemas = Schemas::new();
        assert!(c.compile("file:test.json2", &mut schemas).is_err());

        Ok(())
    }

    #[pg_test]
    fn test_id_for() {
        assert_eq!(id_for!(json!({"$id": "foo"})), "foo");
        assert_eq!(id_for!(json!({"$id": "bar"})), "bar");
        assert_eq!(id_for!(json!({"$id": "yo", "id": "hi"})), "yo");
        assert_eq!(id_for!(json!({"id": "yo"})), "schema.json");
        assert_eq!(id_for!(json!(null)), "schema.json");
    }

    #[pg_test]
    fn test_jsonb_matches_schema() {
        assert!(crate::jsonb_matches_schema(
            Json(json!({"type": "object"})),
            JsonB(json!({"hi": "there"})),
        ));
    }

    #[pg_test]
    fn test_jsonb_matches_schema_sql() -> spi::Result<()> {
        // Valid.
        let query = format!(
            "SELECT jsonb_matches_schema('{}', '{}')",
            json!({"type": "object"}),
            json!({"x": "y"})
        );
        let result = Spi::get_one(&query)?;
        assert_eq!(result, Some(true));

        // Invalid json.
        let query = format!(
            "SELECT jsonb_matches_schema('{}', '{}')",
            json!({"type": "object"}),
            json!(["x", "y"]),
        );
        let result = Spi::get_one(&query)?;
        assert_eq!(result, Some(false));

        // Invalid schema.
        // let query = format!(
        //     "SELECT jsonb_matches_schema('{}', '{}')",
        //     json!({"type": "nonesuch"}),
        //     json!({"x": "y"})
        // );
        // assert!(Spi::run(&query).is_err());
        // let result = Spi::run(&query);
        // assert_eq!(result, Err(pgrx::spi::SpiError::InvalidPosition));

        // NULL instance.
        let query = format!("SELECT jsonb_matches_schema(NULL, '{}')", json!({"x": "y"}));
        let result: Option<bool> = Spi::get_one(&query)?;
        assert_eq!(result, None);

        // NULL schema.
        let query = format!(
            "SELECT jsonb_matches_schema('{}', NULL)",
            json!({"type": "object"})
        );
        let result: Option<bool> = Spi::get_one(&query)?;
        assert_eq!(result, None);

        Ok(())
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
