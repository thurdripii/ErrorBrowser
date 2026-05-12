use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Favorite {
    pub title: String,
    pub url: String,
}

pub fn save_favorite(fav: Favorite) -> std::io::Result<()> {
    let mut favorites = load_favorites();
    favorites.push(fav);
    let j = serde_json::to_string(&favorites)?;
    let mut file = File::create("assets/favorites.json")?;
    file.write_all(j.as_bytes())?;
    Ok(())
}

pub fn load_favorites() -> Vec<Favorite> {
    let data = std::fs::read_to_string("assets/favorites.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_default()
}
