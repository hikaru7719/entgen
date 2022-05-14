use super::*;
use crate::error::EntgenError;
use log::LevelFilter;

extern crate log;

pub async fn run() {
    if let Err(err) = run_with_result().await {
        log_and_exit(err);
    }
    std::process::exit(0);
}

fn log_and_exit(err: EntgenError) {
    log::error!("command failed with err = {:?}", err);
    std::process::exit(1);
}

async fn run_with_result() -> Result<(), EntgenError> {
    env_logger::builder().filter_level(LevelFilter::Info);
    let cli = cli::Cli::parse_opt();
    let config = config::Config::new(cli.file)?;

    let pool = db::information_schema::connect(&config.postgres).await?;

    match generate(&pool, &config).await {
        Ok(_) => Ok(()),
        Err(err) => {
            if !pool.is_closed() {
                pool.close().await;
            }
            Err(err)
        }
    }
}

async fn generate(
    pool: &db::information_schema::Pool,
    config: &config::Config,
) -> Result<(), EntgenError> {
    let tables = db::information_schema::fetch_user_defined_tables(&pool).await?;

    for table_name in tables.iter() {
        let columns = db::information_schema::fetch_column_definition(&pool, table_name).await?;
        let pk = db::information_schema::fetch_column_primary_key(&pool, table_name).await?;
        let entity_template = template::entity::from_vec(table_name, &columns);
        let repository_template = template::repository::from_vec(table_name, &columns, &pk);
        let body = format!(
            "{}\n{}",
            entity_template.build()?,
            repository_template.build()?
        );
        template::writer::write(&config.output_dir, table_name, &body)?;
    }
    Ok(())
}
