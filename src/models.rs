use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Feed {
    pub id: i64,
    pub nome: String,
    pub url: String,
    pub categoria: String,
    pub ativo: i64,
}

#[derive(Debug, Deserialize)]
pub struct FeedInput {
    pub nome: String,
    pub url: String,
    pub categoria: String,
}

#[derive(Debug, Serialize)]
pub struct Mensagem {
    pub mensagem: String,
}
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Noticia {
    pub id: i64,
    pub titulo: String,
    pub descricao: Option<String>,
    pub link: String,
    pub imagem_url: Option<String>,
    pub categoria: String,
    pub fonte: Option<String>,
    pub publicada_em: Option<String>,
}
