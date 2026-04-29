use std::collections::HashMap;

const DEFAULT_WEIGHTS: &[(&str, f64)] = &[
    ("popularity_log", 1.15),
    ("hit_log", 0.95),
    ("popularity_sqrt", 0.25),
    ("hit_log_sqrt", 0.15),
];

pub fn compute_rank_signal(
    popularity: f64,
    hit_log: f64,
    overrides: Option<HashMap<String, f64>>,
) -> f64 {
    let mut weights: HashMap<String, f64> = HashMap::new();

    for (key, value) in DEFAULT_WEIGHTS {
        weights.insert(key.to_string(), *value);
    }

    if let Some(overrides) = overrides {
        for (key, value) in overrides {
            if weights.contains_key(&key) {
                weights.insert(key, value);
            }
        }
    }

    (popularity + 1.0).ln() * weights["popularity_log"]
        + (hit_log + 1.0).ln() * weights["hit_log"]
        + popularity.sqrt() * weights["popularity_sqrt"]
        + hit_log.sqrt() * weights["hit_log_sqrt"]
}

pub fn attach_rank_sort(params: &mut HashMap<String, String>, field: &str, direction: &str) {
    let dir = match direction.to_lowercase().as_str() {
        "asc" => "asc",
        _ => "desc",
    };

    let sort_instruction = format!("{}:{}", field, dir);

    if params.is_empty() {
        params.insert("sort_by".to_string(), sort_instruction);
        return;
    }

    match params.get_mut("sort_by") {
        Some(current) if !current.is_empty() => {
            current.push(',');
            current.push_str(&sort_instruction);
        }
        _ => {
            params.insert("sort_by".to_string(), sort_instruction);
        }
    }
}
