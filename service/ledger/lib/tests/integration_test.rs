use env_logger;
use ledger_lib::{
    entity::{account, txn},
    error::LedgerError,
    ledger::Ledger,
};
use log::LevelFilter;
use migration_lib::{Migrator, MigratorTrait};
use sea_orm::{prelude::Decimal, ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use test_util_rust::testcontainers::mysql::MySql;
use testcontainers::clients;
use uuid::{Uuid, Version};

#[tokio::test]
async fn test_suite() -> Result<(), LedgerError> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .is_test(true)
        .init();

    let docker = clients::Cli::default();
    let node = docker.run(MySql::default());

    let connection_string = &format!(
        "mysql://root:password@127.0.0.1:{}/test",
        node.get_host_port_ipv4(3306)
    );
    let mut opt = ConnectOptions::new(connection_string.to_owned());
    opt.sqlx_logging(true).sqlx_logging_level(LevelFilter::Info);

    let db = Arc::new(Database::connect(opt).await.unwrap());

    let l: Arc<Ledger> = Arc::new(Ledger::new(db.clone(), Some(false)).await?);

    test_new_account(l.clone(), db.clone()).await?;
    test_new_deposit(l.clone(), db.clone()).await?;
    test_new_withdraw_success(l.clone(), db.clone()).await?;
    test_new_withdraw_error(l.clone(), db.clone()).await?;
    test_add_balance_success(l.clone(), db.clone()).await?;
    test_add_balance_no_account(l.clone(), db.clone()).await?;

    Ok(())
}

async fn test_new_account(l: Arc<Ledger>, db: Arc<DatabaseConnection>) -> Result<(), LedgerError> {
    Migrator::up(&*db, None).await?;

    let owner_id = Uuid::new_v4();
    let currency = "BRL";

    let acc: account::Model = (&*l).new_account(owner_id, currency.to_owned()).await?;

    assert_ne!(Uuid::nil(), acc.id);
    assert_eq!(Some(Version::Random), acc.id.get_version());
    assert_eq!(owner_id, acc.owner_id);
    assert_eq!(currency.to_owned(), acc.currency);
    assert_eq!(Decimal::new(0, 0), acc.balance);
    assert_eq!(None, acc.updated_at);

    Migrator::reset(&*db).await?;

    Ok(())
}

async fn test_new_deposit(l: Arc<Ledger>, db: Arc<DatabaseConnection>) -> Result<(), LedgerError> {
    Migrator::up(&*db, None).await?;

    let owner_id = Uuid::new_v4();
    let currency = "BRL";
    let amount = Decimal::new(100, 0);

    let acc: account::Model = (&*l).new_account(owner_id, currency.to_owned()).await?;

    let transaction: txn::Model = (&*l).new_deposit(acc.id, amount).await?;
    assert_ne!(Uuid::nil(), transaction.id);
    assert_eq!(Some(Version::Random), transaction.id.get_version());
    assert_eq!(acc.id, transaction.account_id);
    assert_eq!(txn::TxnType::Deposit, transaction.txn_type);
    assert_eq!(amount, transaction.amount);
    assert_eq!(None, transaction.updated_at);

    Migrator::reset(&*db).await?;

    Ok(())
}

async fn test_new_withdraw_success(
    l: Arc<Ledger>,
    db: Arc<DatabaseConnection>,
) -> Result<(), LedgerError> {
    Migrator::up(&*db, None).await?;

    let owner_id = Uuid::new_v4();
    let currency = "BRL";
    let amount = Decimal::new(100, 0);

    let acc: account::Model = (&*l).new_account(owner_id, currency.to_owned()).await?;
    (&*l).new_deposit(acc.id, amount).await?;

    let transaction: txn::Model = l.new_withdraw(acc.id, amount).await?;
    assert_ne!(Uuid::nil(), transaction.id);
    assert_eq!(Some(Version::Random), transaction.id.get_version());
    assert_eq!(acc.id, transaction.account_id);
    assert_eq!(txn::TxnType::Withdraw, transaction.txn_type);
    assert_eq!(amount, transaction.amount);
    assert_eq!(None, transaction.updated_at);

    Migrator::reset(&*db).await?;

    Ok(())
}

async fn test_new_withdraw_error(
    l: Arc<Ledger>,
    db: Arc<DatabaseConnection>,
) -> Result<(), LedgerError> {
    Migrator::up(&*db, None).await?;

    let owner_id = Uuid::new_v4();
    let currency = "BRL";
    let amount = Decimal::new(100, 0);

    let acc: account::Model = (&*l).new_account(owner_id, currency.to_owned()).await?;

    assert_eq!(
        (&*l).new_withdraw(acc.id, amount).await,
        Err(LedgerError::InsufficientBalanceError)
    );

    Migrator::reset(&*db).await?;

    Ok(())
}

async fn test_add_balance_success(
    l: Arc<Ledger>,
    db: Arc<DatabaseConnection>,
) -> Result<(), LedgerError> {
    Migrator::up(&*db, None).await?;

    let owner_id = Uuid::new_v4();
    let currency = "BRL";
    let amount = Decimal::new(10, 0);

    let acc: account::Model = (&*l).new_account(owner_id, currency.to_owned()).await?;

    let result: bool = (&*l)
        .add_balance_by_owner(acc.id, acc.owner_id, amount)
        .await?;

    assert!(result);

    Migrator::reset(&*db).await?;

    Ok(())
}

async fn test_add_balance_no_account(
    l: Arc<Ledger>,
    db: Arc<DatabaseConnection>,
) -> Result<(), LedgerError> {
    Migrator::up(&*db, None).await?;

    let result: bool = (&*l)
        .add_balance_by_owner(Uuid::new_v4(), Uuid::new_v4(), Decimal::new(10, 0))
        .await?;

    assert!(!result);

    Migrator::reset(&*db).await?;

    Ok(())
}
