use crate::config::configuration;
use crate::config::configuration::Configuration;

#[test]
fn when_configuration_return_all_required() {
    let cfg = Configuration::init_config();

    assert_eq!(cfg.db_host, "localhost");
    assert_eq!(cfg.db_user, "postgres");
    assert_eq!(cfg.db_password, "root");
    assert_eq!(cfg.db_name, "xklean_cleaning");
    assert_eq!(cfg.db_schema, "cls_staff");
    assert_eq!(cfg.service_host, "127.0.0.1");
    assert_eq!(cfg.service_port, "8093");
}

#[test]
fn when_configuration_get_connection_url_all_required() {
    let cfg = Configuration::init_config();

    let url = cfg.get_connection_url();

    assert_eq!(url, "postgres://postgres:root@localhost/xklean_cleaning?currentSchema=cls_staff");
}

#[test]
fn when_configuration_get_service_address_all_required() {
    let cfg = Configuration::init_config();

    let url = cfg.get_service_address();

    assert_eq!(url, format!("{}:{}", "127.0.0.1","8093"));
}