use super::*;

pub async fn run_with_result() {
    let cli = cli::Cli::parse_opt();
    let config = config::Config::new(cli.file).unwrap();

    let pool = db::information_schema::connect(config.postgres)
        .await
        .unwrap();

    let tables = db::information_schema::fetch_user_defined_tables(&pool)
        .await
        .unwrap();

    for table_name in tables.iter() {
        let columns = db::information_schema::fetch_column_definition(&pool, table_name)
            .await
            .unwrap();
        let template = template::from_vec(table_name, columns);
        template.build_and_write("test".to_string()).unwrap();
    }
}
