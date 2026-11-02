CREATE TABLE IF NOT EXISTS feeds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nome TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    categoria TEXT NOT NULL,
    ativo INTEGER DEFAULT 1,
    criado_em DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS noticias (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    titulo TEXT NOT NULL,
    descricao TEXT,
    link TEXT NOT NULL UNIQUE,
    imagem_url TEXT,
    categoria TEXT NOT NULL,
    fonte TEXT,
    publicada_em TEXT,
    criada_em DATETIME DEFAULT CURRENT_TIMESTAMP
);