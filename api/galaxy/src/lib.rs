/// Package galaxy.service
pub mod service {
    pub mod ledger {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/galaxy.service.ledger.v1.rs"));
        }
    }
    pub mod mailer {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/galaxy.service.mailer.v1.rs"));
        }
    }
}

/// Package galaxy.type
pub mod r#type {
    include!(concat!(env!("OUT_DIR"), "/galaxy.r#type.rs"));

    pub mod account {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/galaxy.r#type.account.v1.rs"));
        }
    }
}
