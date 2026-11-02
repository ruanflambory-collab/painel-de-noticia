# 📰 NewsHub — Agregador de Notícias

<p align="center">
  <strong>Centralize notícias de diferentes fontes em um único lugar.</strong>
</p>

<p align="center">
  Um projeto desenvolvido para praticar desenvolvimento Full Stack, integração com APIs, processamento de RSS/XML e persistência de dados utilizando Rust e SQLite.
</p>

---

## 🚀 Sobre o projeto

O **NewsHub** é uma aplicação web de agregação de notícias que permite cadastrar fontes RSS, buscar publicações automaticamente e armazenar as notícias em um banco de dados local.

A proposta é facilitar o acesso a informações de diferentes portais, organizando as publicações por categorias e fontes.

O projeto está sendo desenvolvido como parte da minha evolução nos estudos de **Análise e Desenvolvimento de Sistemas**, com foco em programação, desenvolvimento web e banco de dados.

## ✨ Funcionalidades

- [x] Cadastro de fontes de notícias RSS.
- [x] Listagem das fontes cadastradas.
- [x] Validação de campos obrigatórios.
- [x] Prevenção de fontes duplicadas.
- [x] Integração com servidores externos utilizando HTTP.
- [x] Interpretação de arquivos RSS/XML.
- [x] Importação de notícias para o banco de dados.
- [x] Prevenção de notícias duplicadas.
- [x] Consulta das notícias armazenadas.
- [ ] Melhorias na apresentação das notícias.
- [ ] Ampliação das fontes de notícias.
- [ ] Melhorias no painel administrativo.

> Projeto em desenvolvimento. As funcionalidades estão sendo testadas e aprimoradas continuamente.

## 🛠️ Tecnologias utilizadas

| Tecnologia | Utilização |
|---|---|
| 🦀 Rust | Linguagem principal do backend |
| ⚡ Axum | Criação das rotas HTTP |
| 🌐 HTML5 | Estrutura das páginas |
| 🎨 CSS3 | Estilização e layout |
| ⚙️ JavaScript | Interatividade do frontend |
| 🗄️ SQLite | Banco de dados |
| 🔄 SQLx | Comunicação com o banco |
| 📡 Reqwest | Requisições HTTP |
| 📰 feed-rs | Interpretação de RSS/XML |
| 🔌 Tokio | Execução assíncrona |
| 📦 Tower HTTP | Servir arquivos estáticos |

## 🏗️ Estrutura do projeto

```text
newshub/
│
├── database/
│   ├── schema.sql
│   └── newshub.db
│
├── src/
│   ├── main.rs
│   ├── database.rs
│   ├── models.rs
│   ├── routes.rs
│   └── rss.rs
│
├── static/
│   ├── index.html
│   ├── admin.html
│   │
│   ├── css/
│   │   └── style.css
│   │
│   └── js/
│       ├── app.js
│       └── admin.js
│
├── Cargo.toml
└── Cargo.lock
```

## ⚙️ Como executar o projeto

### 1. Pré-requisitos

Antes de começar, você precisa ter instalado:

- Rust
- Cargo
- Git
- SQLite

No Ubuntu ou WSL, instale as dependências:

```bash
sudo apt update

sudo apt install build-essential pkg-config libssl-dev sqlite3 -y
```

Instale o Rust, caso ainda não tenha:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Configure o ambiente:

```bash
source "$HOME/.cargo/env"
```

### 2. Clone o repositório

```bash
git clone https://github.com/SEU-USUARIO/newshub.git
```

Entre na pasta:

```bash
cd newshub
```

### 3. Execute o projeto

```bash
cargo run
```

O servidor será iniciado em:

```text
http://localhost:3000
```

Para acessar o painel administrativo:

```text
http://localhost:3000/admin.html
```

## 🔗 Rotas da API

| Método | Rota | Descrição |
|---|---|---|
| GET | `/api/feeds` | Lista as fontes cadastradas |
| POST | `/api/feeds` | Cadastra uma nova fonte |
| POST | `/api/feeds/{id}/sync` | Sincroniza notícias de uma fonte |
| GET | `/api/news` | Lista as notícias armazenadas |

## 🗄️ Banco de dados

O NewsHub utiliza SQLite para armazenar informações em duas tabelas principais:

**Tabela `feeds`:**
- Identificação da fonte.
- Nome do portal.
- URL do RSS.
- Categoria.
- Status de ativação.
- Data de cadastro.

**Tabela `noticias`:**
- Identificação da notícia.
- Título.
- Descrição.
- Link original.
- Imagem.
- Categoria.
- Fonte.
- Data de publicação.

## 📡 Como funciona a importação?

1. O usuário cadastra uma fonte RSS.
2. O sistema valida os dados.
3. O backend realiza uma requisição HTTP.
4. O conteúdo RSS/XML é recebido pelo servidor.
5. A biblioteca `feed-rs` interpreta as publicações.
6. As notícias são organizadas e armazenadas no SQLite.
7. O frontend consulta a API para apresentar os resultados.

**Observação:** para importar notícias, é necessário cadastrar o endereço correto do feed RSS ou Atom, e não apenas a página inicial do portal.

---

## 🎯 Objetivos de aprendizado

Este projeto está sendo utilizado para desenvolver conhecimentos práticos em:

- Programação com Rust.
- Desenvolvimento de APIs REST.
- Arquitetura de aplicações web.
- Operações CRUD.
- Banco de dados relacionais.
- Consultas SQL.
- Requisições HTTP.
- Processamento de XML.
- Programação assíncrona.
- Integração entre frontend e backend.

## 🔮 Melhorias futuras

- Sistema de pesquisa de notícias.
- Filtros por categorias.
- Paginação de resultados.
- Atualização automática das fontes.
- Gerenciamento completo das fontes RSS.
- Melhorias de responsividade.
- Publicação da aplicação em um servidor.

## 👨‍💻 Desenvolvedor

**Ruan Flambory**

Estudante de Análise e Desenvolvimento de Sistemas (ADS).

Interessado em desenvolvimento Front-end, Back-end, programação e tecnologia.

📌 Projeto desenvolvido para fins de aprendizado e evolução profissional.

---

<p align="center">
  <strong>NewsHub — Informação organizada em um só lugar. 📰</strong>
</p>
