pub struct Endpoints;

impl Endpoints {
    pub const BASE_URL: &'static str = "https://pokeapi.co/api/v2";
    
    pub fn pokemon(name_or_id: &str) -> String {
        format!("{}/pokemon/{}", Self::BASE_URL, name_or_id)
    }
    
    pub fn move_(name_or_id: &str) -> String {
        format!("{}/move/{}", Self::BASE_URL, name_or_id)
    }
    
    pub fn item(name_or_id: &str) -> String {
        format!("{}/item/{}", Self::BASE_URL, name_or_id)
    }
}