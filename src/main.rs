mod database;
mod models;
mod routes;
mod rss;

use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Conectar ao banco
    let pool = database::init_db().await?;

    // Configurar API
    let api = routes::configurar_rotas(pool);

    // Disponibilizar frontend e API
    let app = Router::new()
        .merge(api)
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true));

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("NewsHub iniciado!");
    println!("Acesse: http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
