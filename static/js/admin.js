
const formulario = document.getElementById("form-feed");
const mensagem = document.getElementById("mensagem-admin");
const container = document.getElementById("feeds-container");

const totalFeeds = document.getElementById("total-feeds");
const totalAtivos = document.getElementById("total-ativos");

const botaoRecarregar = document.getElementById("recarregar-feeds");

// Ler a resposta do servidor, seja JSON ou texto
async function lerResposta(resposta) {
    const texto = await resposta.text();

    let resultado;

    try {
        resultado = texto ? JSON.parse(texto) : {};
    } catch {
        resultado = { mensagem: texto };
    }

    if (!resposta.ok) {
        throw new Error(
            resultado.mensagem ||
            resultado.error ||
            texto ||
            `Erro HTTP ${resposta.status}`
        );
    }

    return resultado;
}

// Buscar feeds cadastrados
async function carregarFeeds() {
    container.textContent = "Carregando fontes...";

    try {
        const resposta = await fetch("/api/feeds");
        const feeds = await lerResposta(resposta);

        totalFeeds.textContent = feeds.length;

        totalAtivos.textContent =
            feeds.filter(feed => feed.ativo === 1).length;

        container.innerHTML = "";

        if (feeds.length === 0) {
            container.textContent = "Nenhuma fonte cadastrada.";
            return;
        }

        feeds.forEach(feed => {
            const card = document.createElement("article");
            card.className = "feed-card";

            const nome = document.createElement("h3");
            nome.textContent = feed.nome;

            const categoria = document.createElement("p");
            categoria.textContent = `Categoria: ${feed.categoria}`;

            const endereco = document.createElement("p");
            endereco.textContent = feed.url;

            const estado = document.createElement("span");

            estado.textContent =
                feed.ativo === 1 ? "Fonte ativa" : "Fonte desativada";

            estado.className =
                feed.ativo === 1 ? "feed-ativo" : "feed-inativo";

            const botao = document.createElement("button");
            botao.textContent = "Importar notícias";

            botao.addEventListener("click", () => {
                sincronizarFeed(feed.id, botao);
            });

            card.appendChild(nome);
            card.appendChild(categoria);
            card.appendChild(endereco);
            card.appendChild(estado);
            card.appendChild(botao);

            container.appendChild(card);
        });

    } catch (erro) {
        console.error("Erro ao carregar feeds:", erro);
        container.textContent = erro.message;
    }
}

// Cadastrar feed
formulario.addEventListener("submit", async (evento) => {
    evento.preventDefault();

    const dados = {
        nome: document.getElementById("nome").value.trim(),
        url: document.getElementById("url").value.trim(),
        categoria: document.getElementById("categoria").value
    };

    mensagem.textContent = "Cadastrando fonte...";

    try {
        const resposta = await fetch("/api/feeds", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(dados)
        });

        const resultado = await lerResposta(resposta);

        mensagem.textContent =
            resultado.mensagem || "Feed cadastrado com sucesso!";

        mensagem.className = "mensagem-sucesso";

        formulario.reset();

        await carregarFeeds();

    } catch (erro) {
        console.error("Erro ao cadastrar feed:", erro);

        mensagem.textContent = erro.message;
        mensagem.className = "mensagem-erro";
    }
});

// Importar notícias de um feed
async function sincronizarFeed(id, botao) {
    botao.disabled = true;
    botao.textContent = "Importando...";

    mensagem.textContent = "Buscando notícias da fonte...";

    try {
        const resposta = await fetch(`/api/feeds/${id}/sync`, {
            method: "POST"
        });

        const resultado = await lerResposta(resposta);

        mensagem.textContent =
            resultado.mensagem || "Importação concluída!";

        mensagem.className = "mensagem-sucesso";

    } catch (erro) {
        console.error("Erro ao importar notícias:", erro);

        mensagem.textContent = erro.message;
        mensagem.className = "mensagem-erro";

    } finally {
        botao.disabled = false;
        botao.textContent = "Importar notícias";
    }
}

// Atualizar listagem
botaoRecarregar.addEventListener("click", carregarFeeds);

// Iniciar painel
carregarFeeds();
