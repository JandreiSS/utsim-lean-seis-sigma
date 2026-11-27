# Changelog

As mudanças deste repositório ficam registradas aqui. O histórico de antes da separação
(2026-11-26) está no CHANGELOG.md da família.

## [Não lançado]

### Adicionado
- O UTSim LSS lê o pedido `nova_oportunidade` do UTSim Ambiental (`--utsim-pedido`, contrato em `docs/troca_ambiental.md` do Ambiental): a janela "Nova oportunidade do UTSim Ambiental" mostra o resumo do cenário e pede a carteira; a ideia entra no banco de ideias com a origem, as alavancas, as diferenças de CO₂e, energia, água e custo para o Base, o investimento, a linha ambiental do mapa de fluxo de valor e os testes da redução, e o arquivo dos cenários anexado com o hash conferido. Outra ação, um resumo que falta ou um arquivo mudado são recusados com a mensagem na barra de baixo. O ganho ambiental não vira ganho financeiro (decisão A25).

### Alterado
- Repositório próprio, separado do monorepo da família UTSim; usa a família pela tag v0.1.0.
- Usa a família pela tag v0.1.1 (as fontes da família entram no `install` comum).

### Corrigido
- O botão "Refazer" da barra de baixo mostrava a frase "Os aprovadores pediram para refazer parte de {$fase}…", nas cinco línguas: duas mensagens usavam a mesma chave. O botão ganhou a chave própria.
- "Menos telas", no pé da barra lateral, mostrava um quadrado no lugar da seta; as duas setas ("▶ Mais telas", "▼ Menos telas") agora existem na fonte.
