use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use sqlx::SqlitePool;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Configuração do banco SQLite
    let options = SqliteConnectOptions::new()
        .filename("database/newshub.db")
        .create_if_missing(true);

    // Criar conexão com o banco
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    println!("Conectado ao SQLite!");

    // Ler o arquivo SQL
    let schema = include_str!("../database/schema.sql");

    // Executar os comandos de criação das tabelas
    for comando in schema.split(';') {
        let comando = comando.trim();

        if !comando.is_empty() {
            sqlx::query(comando).execute(&pool).await?;
        }
    }

    println!("Tabelas criadas com sucesso!");

    Ok(pool)
}
