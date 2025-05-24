use reqwest;
use serde_json::Value;
use url::Url;

// List of pages or namespaces to exclude
const EXCLUDED_PAGES: &[&str] = &[
    "Doi (identifier)",
    "ISBN (identifier)",
    "ISSN (identifier)",
    "PMID (identifier)",
    "OCLC (identifier)",
    "Bibcode (identifier)",
    "S2CID (identifier)",
    "Category:",
    "Template:",
    "Wikipedia:",
    "File:",
    "Portal:",
    "Help:",
    "Talk:",
    "Special:",
];

// Check if a page is valid (not in excluded list)
fn is_valid_page(page: &str) -> bool {
    !EXCLUDED_PAGES.iter().any(|&excluded| page.starts_with(excluded) || page == excluded)
}

// Fetch links from a Wikipedia article (provided function)
pub async fn get_links(title: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://en.wikipedia.org/w/api.php?action=parse&page={}&format=json&prop=links",
        title
    );

    let response_text = reqwest::get(&api_url)
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&response_text)?;

    let mut results = Vec::new();

    if let Some(links) = json["parse"]["links"].as_array() {
        for link in links {
            if let Some(ns) = link.get("ns").and_then(|ns| ns.as_u64()) {
                if ns != 0 {
                    continue;
                }
            }

            if let Some(link_title) = link.get("*").and_then(|v| v.as_str()) {
                if is_valid_page(link_title) {
                    results.push(link_title.to_string());
                }
            }
        }
    }

    Ok(results)
}

// Extract title from a Wikipedia URL (provided function)
pub fn extract_title_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = Url::parse(url)?;
    let segments: Vec<_> = parsed.path_segments().ok_or("Invalid path")?.collect();
    if let Some(title) = segments.last() {
        Ok(title.replace("_", " "))
    } else {
        Err("No title found".into())
    }
}