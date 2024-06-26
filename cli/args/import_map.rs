// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::serde_json;
use deno_core::url::Url;
use deno_runtime::deno_permissions::PermissionsContainer;
use import_map::ImportMap;
use import_map::ImportMapDiagnostic;
use log::warn;

use super::ConfigFile;
use crate::file_fetcher::FileFetcher;

pub async fn resolve_import_map(
  specified_specifier: Option<&Url>,
  maybe_config_file: Option<&ConfigFile>,
  file_fetcher: &FileFetcher,
) -> Result<Option<ImportMap>, AnyError> {
  if let Some(specifier) = specified_specifier {
    resolve_import_map_from_specifier(specifier.clone(), file_fetcher)
      .await
      .with_context(|| format!("Unable to load '{}' import map", specifier))
      .map(Some)
  } else if let Some(config_file) = maybe_config_file {
    let maybe_url_and_value = config_file
      .to_import_map_value(|specifier| {
        let specifier = specifier.clone();
        async move {
          let file = file_fetcher
            .fetch(&specifier, &PermissionsContainer::allow_all())
            .await?
            .into_text_decoded()?;
          Ok(file.source.to_string())
        }
      })
      .await
      .with_context(|| {
        format!(
          "Unable to resolve import map in '{}'",
          config_file.specifier
        )
      })?;
    match maybe_url_and_value {
      Some((url, value)) => {
        import_map_from_value(url.into_owned(), value).map(Some)
      }
      None => Ok(None),
    }
  } else {
    Ok(None)
  }
}

async fn resolve_import_map_from_specifier(
  specifier: Url,
  file_fetcher: &FileFetcher,
) -> Result<ImportMap, AnyError> {
  let value: serde_json::Value = if specifier.scheme() == "data" {
    let data_url_text =
      deno_graph::source::RawDataUrl::parse(&specifier)?.decode()?;
    serde_json::from_str(&data_url_text)?
  } else {
    let file = file_fetcher
      .fetch(&specifier, &PermissionsContainer::allow_all())
      .await?
      .into_text_decoded()?;
    serde_json::from_str(&file.source)?
  };
  import_map_from_value(specifier, value)
}

pub fn import_map_from_value(
  specifier: Url,
  json_value: serde_json::Value,
) -> Result<ImportMap, AnyError> {
  debug_assert!(
    !specifier.as_str().contains("../"),
    "Import map specifier incorrectly contained ../: {}",
    specifier.as_str()
  );
  let result = import_map::parse_from_value(specifier, json_value)?;
  print_import_map_diagnostics(&result.diagnostics);
  Ok(result.import_map)
}

fn print_import_map_diagnostics(diagnostics: &[ImportMapDiagnostic]) {
  if !diagnostics.is_empty() {
    warn!(
      "Import map diagnostics:\n{}",
      diagnostics
        .iter()
        .map(|d| format!("  - {d}"))
        .collect::<Vec<_>>()
        .join("\n")
    );
  }
}

pub fn enhance_import_map_value_with_workspace_members(
  mut import_map_value: serde_json::Value,
  workspace_members: &[deno_config::WorkspaceMemberConfig],
) -> serde_json::Value {
  let mut imports =
    if let Some(imports) = import_map_value.get("imports").as_ref() {
      imports.as_object().unwrap().clone()
    } else {
      serde_json::Map::new()
    };

  for workspace_member in workspace_members {
    let name = &workspace_member.package_name;
    let version = &workspace_member.package_version;
    // Don't override existings, explicit imports
    if imports.contains_key(name) {
      continue;
    }

    imports.insert(
      name.to_string(),
      serde_json::Value::String(format!("jsr:{}@^{}", name, version)),
    );
  }

  import_map_value["imports"] = serde_json::Value::Object(imports);
  ::import_map::ext::expand_import_map_value(import_map_value)
}
