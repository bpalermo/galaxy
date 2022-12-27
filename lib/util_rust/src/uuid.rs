use uuid::Uuid;

pub fn parse_uuid_from_string(id: String) -> Option<Uuid> {
    match Uuid::parse_str(id.as_str()) {
        Ok(parsed) => Some(parsed),
        Err(_) => None,
    }
}
