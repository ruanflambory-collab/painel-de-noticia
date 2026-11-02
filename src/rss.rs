
use sqlx::SqlitePool;

pub async fn importar_noticias(
    pool: &SqlitePool,
    url: &str,
    categoria: &str,
    fonte: &str,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {

    // Criar cliente HTTP
    let cliente = reqwest::Client::builder()
        .user_agent("NewsHub/1.0")
        .build()?;

    // Buscar o conteúdo do endereço RSS
    let resposta = cliente
        .get(url)
        .header(
            "Accept",
            "application/rss+xml, application/atom+xml, application/xml, text/xml, */*",
        )
        .send()
        .await?;

    println!("URL solicitada: {}", url);
    println!("Status HTTP: {}", resposta.status());
    println!("URL final: {}", resposta.url());

    // Verificar se a resposta foi bem-sucedida
    let resposta = resposta.error_for_status()?;

    // Receber o conteúdo
    let conteudo = resposta.bytes().await?;

    println!("Tamanho recebido: {} bytes", conteudo.len());

    // Verificar se o conteúdo está vazio
    if conteudo.is_empty() {
        return Err("O servidor retornou um conteúdo vazio.".into());
    }

    // Mostrar os primeiros bytes recebidos
    println!(
        "Primeiros bytes: {:?}",
        &conteudo[..conteudo.len().min(100)]
    );

    // Converter o conteúdo para texto
    let texto = String::from_utf8_lossy(&conteudo);

    println!("Conteúdo recebido pelo Rust:");
    println!("{}", &texto[..texto.len().min(500)]);

    // Remover possível BOM e espaços antes do XML
    let texto_limpo = texto.trim_start_matches('\u{feff}').trim();

    // Interpretar o RSS/XML
    let feed = feed_rs::parser::parse(texto_limpo.as_bytes())
        .map_err(|erro| {
            format!(
                "Erro ao interpretar RSS/XML: {}. Conteúdo recebido: {}",
                erro,
                &texto_limpo[..texto_limpo.len().min(200)]
            )
        })?;

    let mut quantidade = 0;

    // Percorrer as notícias encontradas
    for entrada in feed.entries {
        let titulo = entrada
            .title
            .map(|t| t.content)
            .unwrap_or_else(|| "Sem título".to_string());

        let descricao = entrada
            .summary
            .map(|s| s.content)
            .unwrap_or_default();

        let link = match entrada.links.first() {
            Some(link) => link.href.clone(),
            None => continue,
        };

        let publicada_em = entrada
            .published
            .or(entrada.updated)
            .map(|data| data.to_rfc3339());

        // Salvar notícia no banco de dados
        let resultado = sqlx::query(
            r#"
            INSERT INTO noticias
            (titulo, descricao, link, categoria, fonte, publicada_em)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(link) DO NOTHING
            "#,
        )
        .bind(&titulo)
        .bind(&descricao)
        .bind(&link)
        .bind(categoria)
        .bind(fonte)
        .bind(&publicada_em)
        .execute(pool)
        .await?;

        quantidade += resultado.rows_affected() as usize;
    }

    println!("Notícias novas importadas: {}", quantidade);

    Ok(quantidade)
}
