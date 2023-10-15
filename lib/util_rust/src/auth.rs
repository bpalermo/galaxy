use crate::uuid::parse_uuid_from_string;

use tonic::Request;
use uuid::Uuid;

pub fn get_owner_id_from_subject_header<T>(
    request: &Request<T>,
    subject_header: &str,
) -> Option<Uuid> {
    match request
        .metadata()
        .get(subject_header.clone())
        .unwrap()
        .to_str()
    {
        Ok(h) => parse_uuid_from_string(h.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use ledger_v1_rust::galaxy::service::ledger::v1::MyNewAccountRequest;
    use tonic::Request;
    use uuid::Uuid;

    #[test]
    fn test_get_owner_id_from_subject_header_success() {
        let expected: Uuid = Uuid::new_v4();

        let mut r: Request<MyNewAccountRequest> = Request::new(MyNewAccountRequest {
            currency: "BRL".to_string(),
        });

        r.metadata_mut()
            .insert("x-jwt-subject", expected.to_string().parse().unwrap());

        let result = get_owner_id_from_subject_header(&r, "x-jwt-subject");
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_get_owner_id_from_subject_header_none() {
        let mut r: Request<MyNewAccountRequest> = Request::new(MyNewAccountRequest {
            currency: "BRL".to_string(),
        });

        r.metadata_mut()
            .insert("x-jwt-subject", "".to_string().parse().unwrap());

        let result = get_owner_id_from_subject_header(&r, "x-jwt-subject");
        assert_eq!(None, result)
    }
}
