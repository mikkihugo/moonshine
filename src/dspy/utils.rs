pub fn get_iter_from_value(
  value: &serde_json::Value,
) -> impl Iterator<Item = (String, serde_json::Value)> {
  value
    .as_object()
    .map(|obj| {
      obj
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect::<Vec<_>>()
    })
    .unwrap_or_default()
    .into_iter()
}
