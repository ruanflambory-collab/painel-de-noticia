
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use sqlx::SqlitePool;

use crate::models::{Feed, FeedInput, Mensagem, Noticia};
use crate::rss::importar_noticias;

// Consultar todos os feeds
pub async fn listar_feeds(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Feed>>, (StatusCode, String)> {
    let feeds = sqlx::query_as::<_, Feed>(
        "SELECT id, nome, url, categoria, ativo FROM feeds ORDER BY id DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Erro ao listar feeds: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao consultar os feeds.".to_string(),
        )
    })?;

    Ok(Json(feeds))
}

// Cadastrar um novo feed
pub async fn cadastrar_feed(
    State(pool): State<SqlitePool>,
    Json(dados): Json<FeedInput>,
) -> Result<Json<Mensagem>, (StatusCode, String)> {

    if dados.nome.trim().is_empty()
        || dados.url.trim().is_empty()
        || dados.categoria.trim().is_empty()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Preencha todos os campos.".to_string(),
        ));
    }

    let url = dados.url.trim();

    if !url.starts_with("https://") && !url.starts_with("http://") {
        return Err((
            StatusCode::BAD_REQUEST,
            "Informe uma URL HTTP ou HTTPS válida.".to_string(),
        ));
    }

    // Verificar se a URL já existe
    let existe = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM feeds WHERE url = ?",
    )
    .bind(url)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Erro ao verificar URL: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao consultar o banco de dados.".to_string(),
        )
    })?;

    if existe > 0 {
        return Err((
            StatusCode::CONFLICT,
            "Esta fonte RSS já está cadastrada.".to_string(),
        ));
    }

    sqlx::query(
        "INSERT INTO feeds (nome, url, categoria) VALUES (?, ?, ?)",
    )
    .bind(dados.nome.trim())
    .bind(url)
    .bind(dados.categoria.trim())
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Erro ao cadastrar feed: {}", e);
        (
            StatusCode::CONFLICT,
            "Não foi possível cadastrar a fonte. Verifique se a URL já existe.".to_string(),
        )
    })?;

    Ok(Json(Mensagem {
        mensagem: "Feed cadastrado com sucesso!".to_string(),
    }))
}

// Atualizar as notícias de um feed
pub async fn sincronizar_feed(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Mensagem>, (StatusCode, String)> {

    let feed = sqlx::query_as::<_, (String, String, String)>(
        "SELECT nome, url, categoria FROM feeds WHERE id = ? AND ativo = 1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("Erro ao buscar feed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao consultar o feed.".to_string(),
        )
    })?;

    let (nome, url, categoria) = feed.ok_or((
        StatusCode::NOT_FOUND,
        "Feed não encontrado ou desativado.".to_string(),
    ))?;

    let quantidade = importar_noticias(&pool, &url, &categoria, &nome)
        .await
        .map_err(|e| {
            eprintln!("Erro ao importar notícias: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                format!("Não foi possível importar o feed: {}", e),
            )
        })?;

    Ok(Json(Mensagem {
        mensagem: format!("{} notícia(s) nova(s) importada(s)!", quantidade),
    }))
}

// Listar notícias
pub async fn listar_noticias(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Noticia>>, (StatusCode, String)> {

    let noticias = sqlx::query_as::<_, Noticia>(
        r#"
        SELECT
            id,
            titulo,
            descricao,
            link,
            imagem_url,
            categoria,
            fonte,
            publicada_em
        FROM noticias
        ORDER BY publicada_em DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Erro ao listar notícias: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao consultar as notícias.".to_string(),
        )
    })?;

    Ok(Json(noticias))
}

// Organizar as rotas da API
pub fn configurar_rotas(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/feeds", get(listar_feeds).post(cadastrar_feed))
        .route("/api/feeds/{id}/sync", post(sincronizar_feed))
        .route("/api/news", get(listar_noticias))
        .with_state(pool)
}
