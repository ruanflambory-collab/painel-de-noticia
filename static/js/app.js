
const listaNoticias = document.getElementById("lista-noticias");
const campoPesquisa = document.getElementById("pesquisa");
const statusNoticias = document.getElementById("status");
const botaoAtualizar = document.getElementById("atualizar");

let noticias = [];
let categoriaSelecionada = "Todas";

// Buscar notícias da API Rust
async function carregarNoticias() {

    statusNoticias.textContent = "Carregando notícias...";
    listaNoticias.innerHTML = "";

    try {

        const resposta = await fetch("/api/news");

        if (!resposta.ok) {
            throw new Error("Erro ao consultar a API");
        }

        noticias = await resposta.json();

        mostrarNoticias();

    } catch (erro) {

        console.error(erro);

        statusNoticias.textContent =
            "Não foi possível carregar as notícias.";

    }
}

// Criar os cartões das notícias
function mostrarNoticias() {

    listaNoticias.innerHTML = "";

    const pesquisa = campoPesquisa.value
        .toLowerCase()
        .trim();

    const filtradas = noticias.filter(noticia => {

        const correspondePesquisa =
            noticia.titulo.toLowerCase().includes(pesquisa) ||
            (noticia.descricao || "").toLowerCase().includes(pesquisa);

        const correspondeCategoria =
            categoriaSelecionada === "Todas" ||
            noticia.categoria === categoriaSelecionada;

        return correspondePesquisa && correspondeCategoria;
    });

    statusNoticias.textContent =
        `${filtradas.length} notícia(s) encontrada(s).`;

    if (filtradas.length === 0) {

        statusNoticias.textContent =
            "Nenhuma notícia encontrada.";

        return;
    }

    filtradas.forEach(noticia => {

        const card = document.createElement("article");
        card.className = "card-noticia";

        const categoria = document.createElement("span");
        categoria.className = "categoria";
        categoria.textContent = noticia.categoria;

        const titulo = document.createElement("h3");
        titulo.textContent = noticia.titulo;

        const descricao = document.createElement("p");
        descricao.textContent =
            noticia.descricao || "Descrição não disponível.";

        const fonte = document.createElement("small");
        fonte.textContent =
            `Fonte: ${noticia.fonte || "Não informada"}`;

        const link = document.createElement("a");
        link.href = noticia.link;
        link.textContent = "Ler notícia completa";
        link.target = "_blank";
        link.rel = "noopener noreferrer";

        card.appendChild(categoria);
        card.appendChild(titulo);
        card.appendChild(descricao);
        card.appendChild(fonte);
        card.appendChild(link);

        listaNoticias.appendChild(card);
    });
}

// Pesquisar enquanto digita
campoPesquisa.addEventListener("input", mostrarNoticias);

// Filtrar por categoria
document.querySelectorAll(".filtro").forEach(botao => {

    botao.addEventListener("click", () => {

        document.querySelectorAll(".filtro").forEach(item => {
            item.classList.remove("ativo");
        });

        botao.classList.add("ativo");

        categoriaSelecionada = botao.dataset.categoria;

        mostrarNoticias();
    });
});

// Atualizar manualmente
botaoAtualizar.addEventListener("click", carregarNoticias);

// Iniciar carregamento
carregarNoticias();
