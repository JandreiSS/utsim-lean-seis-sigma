# Plano: gerenciador de projetos Lean Seis Sigma da família UTSim

Plano técnico, sem implementação, de um aplicativo próprio da família UTSim para gerenciar
projetos de melhoria Lean Seis Sigma, distribuído pelo mesmo launcher do simulador e integrado a
ele. Objetivo do dono: "gerenciar os projetos e ter todo o fluxo natural de um projeto Lean Six
Sigma à mão". Nome provisório (decisão N1, 2026-11-03): **UTSim Seis Sigma**, produto
`lean-seis-sigma` no launcher; o dono pode trocar depois. No texto ele aparece como **app LSS** e
as crates com o prefixo `utsim-lss`.

Cada seção traz o que o app faz, onde mora o cálculo, como é testado, estimativa em horas
Opus 5.5 e o modelo de IA sugerido. Os requisitos têm prefixo próprio (`LSS-RF-nn`,
`LSS-RNF-nn`) e ficam só aqui até o dono aprovar; não entram em `docs/requisitos.md`.

Documentos relacionados, que este plano não repete:

- `docs/plano_analises_e_dinamica.md`, seção 2: estatística P1 a P3 do simulador (`utsim-stats`,
  ANOVA e Tukey, correlação, planejamento de experimentos, metamodelo) e seção 1 (PERT/CPM);
- `docs/plano_gemeo_digital_e_painel.md`: gêmeo digital (`utsim-twin`, `utsim-connect`),
  painel web (`utsim-dash`, `utsim-kpi`) e estatística E1 a E11, em especial E4 (CEP) e E6
  (capacidade);
- `docs/backlog_v02.md`: levas, critério de escolha do modelo e o launcher;
- `docs/roadmap.md`: o launcher (códigos de acesso, canais, Drive);
- `docs/arquitetura.md` e `docs/versionamento.md`: camadas, formato versionado e migrações.

Não existe ainda `docs/plano_launcher.md`; a seção 4.8 lista o que o app LSS exige dele.

---

## 0. Ponto de partida e princípios

### 0.1 O que já existe e será reaproveitado

| Peça | Onde | Uso no app LSS |
|---|---|---|
| Intervalos de confiança em qualquer nível, quantil t geral, funções especiais (`ln_gamma`, `beta_inc`, `gamma_p`, `norm_cdf`, `norm_ppf`, `t_cdf`, `t_ppf`, `brent`) | `crates/utsim-stats` (`lib.rs`, `special.rs`) | base de todos os testes de hipótese, capacidade, CEP e MSA |
| Aquecimento, precisão, médias em lotes, Little, autocorrelação lag 1 | `crates/utsim-stats` (`warmup.rs`, `precision.rs`, `little.rs`) | fase Medir com dados simulados e do gêmeo |
| Leitura de CSV/TXT, descritiva, independência, Mann-Kendall, atípicos, ajuste de distribuições, Anderson-Darling e K-S com bootstrap, AIC | `crates/utsim-fit` (`data.rs`, `diagnostics.rs`, `gof.rs`, `rank.rs`) | capacidade não normal (método dos percentis), diagnóstico dos dados de coleta |
| Estatísticas por objeto, gargalo, gargalo no turno, gargalo móvel, séries de conteúdo, histogramas, diferença pareada | `crates/utsim-model/src/analysis.rs` e `analysis/advanced.rs` | fases Medir, Analisar e Melhorar sobre o modelo |
| Cenários com números aleatórios comuns (`Experiment`, `run_scenarios`) | `crates/utsim-model/src/experiment.rs` | "testar no simulador antes de mexer na fábrica" |
| Otimização com comparação pareada | `crates/utsim-opt` | fase Melhorar |
| Rede de atividades, CPM, PERT, Monte Carlo, caminho crítico observado | `crates/utsim-pert` (`network`, `cpm`, `pert`, `monte_carlo`, `observed`) | cronograma do projeto (rede montada à mão com `Activity::new`) e fase Analisar (caminho crítico do processo) |
| Fluxograma do processo com os símbolos da engenharia de métodos, carta de processo em HTML e CSV, Mermaid | `crates/utsim-diagram` (ainda no branch `worktree-agent-a2b367131e1621448`, commits `0e4094e` e `c3395c3`, não integrado a `feat/fase-1`) | mapa de processo, carta de processo e base do SIPOC e do VSM |
| Planta CAD (DXF, calibração, obstáculos) | `crates/utsim-cad` (branch `worktree-agent-a899a9b2fb3c80b57`, em integração) | fundo do diagrama de espaguete |
| Relatório HTML autocontido com SVG em linha; CSV | `crates/utsim-model/src/html.rs`, `csv.rs` | exportações e gráficos |
| Página web em JavaScript puro, sem etapa de build | `web/` (`app.js`, `worker.js`) | estilo da interface do app LSS |
| Painel web servido pelo programa, gráficos SVG próprios, PDF pela impressão do navegador, `.xlsx` atrás de feature (decisões D1, D2, D8 e D9 aprovadas em 2026-11-02) | `docs/plano_gemeo_digital_e_painel.md`, seção 2 | mesma tecnologia de interface e o mesmo módulo de gráficos |

### 0.2 Princípios

- **Cálculo no Rust, apresentação fina.** A regra de `plano_analises_e_dinamica.md`, seção 0,
  vale aqui: toda conta testável mora numa crate pura, com testes contra resultados de
  referência. A interface só desenha números que chegam prontos em JSON.
- **Uma implementação estatística para a família.** CEP, capacidade, testes e DOE são escritos
  uma vez, nas crates comuns, e usados pelo simulador, pelo painel do gêmeo e pelo app LSS. Os
  itens E4 (CEP) e E6 (capacidade) do plano do gêmeo e os itens 9, 10, 13 e 14 do plano de
  análises passam a ser implementados nas mesmas crates descritas na seção 3.6, e não em
  duplicata.
- **Análise reprodutível.** Cada análise guardada no projeto é uma receita: conjunto de dados
  (com hash), função, parâmetros e versão da crate. O resultado é recalculado de forma
  determinística e mostrado com essa identificação; o relatório de tollgate cita a receita.
- **Origem do dado sempre visível.** Todo conjunto de dados carrega `origem: Real | Gêmeo |
  Simulado`. Gráficos, índices e relatórios mostram a origem; um tollgate não aceita linha de
  base de capacidade só com dado simulado sem uma ressalva registrada.
- **Funciona sem o simulador.** O app LSS é útil para quem não tem o UTSim instalado; os recursos
  de integração aparecem desabilitados, com a explicação, quando o simulador não está presente.
- **Custo zero de infraestrutura.** Nenhum servidor, nuvem ou banco pago. Trabalho em equipe por
  pasta compartilhada no Google Drive do usuário (seção 4.5), como a distribuição.
- **Cada número com sua incerteza.** Índices de capacidade com intervalo, testes com tamanho de
  efeito e poder, savings com o estado de validação.

---

## 1. Fluxo do projeto

### 1.1 Carteira: entrada, seleção e priorização

**Banco de ideias.** Qualquer pessoa registra uma oportunidade (título, processo, problema
observado, dado disponível, estimativa grosseira de ganho). Cada ideia passa por estados
`ideia → triagem → aprovada → em execução → encerrada | cancelada | arquivada`, num quadro da
carteira.

**Escolha da abordagem.** Na triagem, um roteiro de perguntas sugere a abordagem, e o champion
decide:

| Situação | Abordagem sugerida |
|---|---|
| Causa e solução conhecidas, risco baixo | ação direta (cartão no kanban, sem projeto) |
| Problema local, solução provável com a equipe do processo em poucos dias | kaizen rápido |
| Problema delimitado, uma página basta para conduzir e comunicar | A3 |
| Ciclo curto e repetitivo de melhoria contínua | PDCA |
| Processo existente, causa desconhecida, variação ou desempenho medidos | DMAIC |
| Processo ou produto novo, ou o atual não tem como atingir a meta | DMADV (DFSS) |

**Priorização.** Três instrumentos, todos configuráveis pela empresa:

1. **Matriz impacto × esforço** (2 × 2: ganho rápido, projeto estratégico, tarefa menor,
   evitar), com impacto e esforço numa escala de 1 a 5 ou derivados dos critérios abaixo.
2. **Pontuação ponderada.** Critérios com peso (ganho financeiro, satisfação do cliente,
   segurança, alinhamento estratégico, disponibilidade de dados, duração, risco); nota de 1 a
   5 ou 1-3-9; total normalizado e classificação.
3. **Alinhamento estratégico.** Matriz objetivos estratégicos × projetos com intensidade 0, 1,
   3 ou 9 (forma de desdobramento usual, ver Pyzdek, *The Six Sigma Handbook* [conferir
   capítulo]); a soma ponderada pelos pesos dos objetivos entra como critério do item 2.

A carteira mostra também a **capacidade das pessoas**: belts disponíveis, projetos por belt e
horas comprometidas, com aviso quando alguém passa do limite configurado.

### 1.2 Papéis

| Papel | Responsabilidade no app | Aprova |
|---|---|---|
| Patrocinador | dono do resultado de negócio; libera recursos | termo de abertura, tollgates, encerramento |
| Champion (ou dono da carteira) | seleciona e prioriza, remove barreiras, acompanha a carteira | priorização, tollgates |
| Dono do processo | recebe o processo melhorado e o plano de controle | tollgate do Controlar, transferência |
| Master black belt | mentor; revisa a estatística | parecer técnico nos tollgates (opcional) |
| Black belt / green belt | conduz o projeto | submete entregáveis |
| Yellow belt e equipe | executam ações, coletam dados, participam das reuniões | nada |
| Controladoria | valida os ganhos | savings previstos e realizados |

Os papéis são por projeto (uma pessoa pode ser green belt num e equipe noutro). O app gera a
matriz RACI das entregas a partir dos papéis e permite editá-la. Sem servidor não há
autenticação forte; a seção 4.5 trata da identificação e de seus limites. A ISO 18404 descreve
competências por nível de belt [conferir edição vigente]; o app só registra o nível declarado.

### 1.3 Termo de abertura

Campos: título; processo e área; declaração do problema (o quê, onde, quando, quanto, sem causa
nem solução); Y do projeto e CTQs ligados à árvore VOC/CTQ (seção 2.1); linha de base, meta e
prazo (meta SMART, com unidade e fonte do dado); escopo dentro e fora; caso de negócio e ganho
estimado (hard e soft, seção 1.10); equipe e papéis; cronograma macro com as datas dos
tollgates; riscos iniciais; premissas e restrições. O termo é versionado: mudança de meta ou
escopo depois de aprovado gera nova versão e precisa de nova aprovação do patrocinador.

### 1.4 DMAIC: fases, entregáveis e tollgates

| Fase | Entregáveis | Ferramentas principais (seção 2) |
|---|---|---|
| Definir | termo aprovado; VOC e árvore CTQ; SIPOC; mapa de processo de alto nível; equipe e RACI; cronograma; riscos; ganho estimado | VOC/CTQ, SIPOC, fluxograma, Pareto, matriz de partes interessadas |
| Medir | definição operacional do Y; plano de coleta; MSA aprovado; linha de base (estabilidade e capacidade); mapa detalhado e VSM atual; meta confirmada | plano de coleta, MSA, descritiva, cartas de controle, capacidade, VSM, carta de processo |
| Analisar | causas potenciais (Ishikawa, C&E); causas raiz verificadas com dado (testes, regressão); gargalo e desperdícios quantificados; FMEA do processo atual | Ishikawa, 5 porquês, matriz C&E, testes de hipótese, regressão, gargalo, PERT observado, espaguete |
| Melhorar | soluções geradas e selecionadas; DOE quando cabível; cenários no simulador; piloto com comparação estatística antes × depois; FMEA revisado; VSM futuro; plano de implantação | DOE, cenários UTSim, testes de hipótese, poka-yoke, 5S, SMED, trabalho padronizado |
| Controlar | plano de controle e de resposta; cartas de controle em operação; capacidade final; procedimentos atualizados; ganhos validados; transferência ao dono do processo; lições aprendidas | cartas de controle, capacidade, plano de controle, painel e gêmeo |

**Tollgate.** Ao fim de cada fase:

1. **Checklist.** Itens da fase, cada um com tipo `manual` (o belt marca e anexa evidência) ou
   `verificado` (o app confere sozinho: "existe MSA com %GRR ≤ 30 % da variação total ou
   justificativa registrada", "capacidade de linha de base calculada sobre dado com origem
   Real", "toda causa raiz tem ao menos uma análise ligada", "toda ação do Melhorar está
   concluída ou transferida"). Itens obrigatórios e opcionais, editáveis no modelo da empresa
   (seção 4.7).
2. **Relatório de tollgate** gerado (seção 4.6), com as receitas das análises.
3. **Reunião de revisão** com ata (seção 1.8).
4. **Decisão** de cada aprovador: `aprovar`, `aprovar com ressalvas` (ressalvas viram ações),
   `refazer` (volta à fase), `suspender` ou `cancelar`. A regra de aprovação (todos, maioria,
   patrocinador obrigatório) é configurável.
5. **Congelamento.** A aprovação grava o hash do estado dos entregáveis da fase. Editar um
   entregável aprovado depois é possível, mas marca o tollgate como "alterado depois da
   aprovação" com a diferença, até nova aprovação.

ISO 13053-1 e 13053-2 descrevem a metodologia DMAIC e as ferramentas por fase [conferir
edição; a revisão de 2011 é a conhecida]; o checklist padrão segue essa estrutura.

### 1.5 Outras abordagens

- **DMADV (DFSS).** Definir, Medir (VOC, CTQs, casa da qualidade QFD), Analisar (geração de
  conceitos, matriz de Pugh, análise funcional), Desenhar (projeto detalhado, tolerâncias por
  Monte Carlo, DOE, FMEA de projeto; o processo novo modelado e testado no UTSim antes de ser
  construído), Verificar (piloto, capacidade, plano de controle, transferência). Os tollgates
  funcionam como no DMAIC.
- **Kaizen rápido.** Pré-evento (escopo, dados, equipe, logística), evento de 3 a 5 dias com uma
  pauta por dia (entender, medir, analisar, melhorar, padronizar e apresentar), acompanhamento
  em 30, 60 e 90 dias com a lista de pendências. Um tollgate de abertura e um de encerramento.
- **A3.** Uma página com sete blocos (contexto, situação atual, meta, análise de causa,
  contramedidas, plano, acompanhamento), editada no próprio formato da folha e exportada em A3
  paisagem (Sobek e Smalley, *Understanding A3 Thinking* [conferir]). O A3 pode ser também o
  resumo de um DMAIC: os blocos se preenchem a partir dos entregáveis.
- **PDCA.** Quatro etapas leves, sem tollgate formal; cada ciclo registra hipótese, ação,
  verificação e padronização; ciclos encadeados no mesmo projeto.

As abordagens são **modelos de fluxo** (seção 4.7): a empresa pode criar o seu (por exemplo um
8D) sem código, como lista de fases, entregáveis, checklists e aprovadores.

### 1.6 Cronograma: Gantt e PERT

- Tarefas com duração determinística ou em três pontos (otimista, mais provável, pessimista),
  dependências fim-início com defasagem, marcos nos tollgates, responsável e percentual
  concluído; linha de base congelada na aprovação do termo para comparar previsto × real.
- **Cálculo pelo `utsim-pert`**: a rede é montada à mão com `Activity::new` e `precedences`;
  `cpm::schedule` dá datas mais cedo e mais tarde, folgas e caminho crítico; `pert::analyze` dá
  a probabilidade de cumprir a data do tollgate; `monte_carlo::run` dá a distribuição da data
  final e o índice de criticidade. A conversão entre duração em dias úteis e datas de
  calendário (fins de semana, feriados, calendário da empresa) é nova e fica em
  `utsim-lss-core` (o `utsim-pert` trabalha em unidades de tempo, sem calendário).
- Gantt com barras de linha de base e reais, caminho crítico destacado e dependências; o
  diagrama de rede vem de `report::diagram`.
- Observação: `utsim-pert` depende de `utsim-model`, `utsim-objects` e `utsim-scene` por causa da
  derivação a partir do modelo. Como o app LSS já depende de `utsim-model` para a integração, não
  há custo novo; se um dia o app precisar compilar sem o simulador, separa-se a derivação atrás
  de uma feature `model` do `utsim-pert` (tarefa pequena, Sonnet).

### 1.7 Riscos e quadro de ações

- **Riscos**: descrição, causa, efeito, probabilidade e impacto (1 a 5), nível na matriz,
  resposta (evitar, mitigar, transferir, aceitar), dono, prazo, estado; as respostas viram
  ações. Estrutura de acordo com a ISO 31000 [conferir edição].
- **Quadro kanban de ações**: cartões no formato 5W2H (o quê, por quê, onde, quando, quem, como,
  quanto), com origem (FMEA, ata, tollgate, risco, plano de resposta, kaizen), colunas
  configuráveis (a fazer, fazendo, bloqueado, verificar, feito), limite de cartões em andamento
  por coluna, raias por fase ou por pessoa, prazos vencidos destacados. Um cartão pode exigir
  verificação de eficácia (data e critério) antes de ir para "feito".
- Visão "minhas ações" de todos os projetos da carteira, para cada pessoa.

### 1.8 Reuniões e atas

Pauta, participantes (presentes e ausentes), anotações, decisões e ações. Uma ação escrita na
ata vira cartão no kanban com um clique, ligada à ata. Atas de tollgate trazem as decisões dos
aprovadores. Reuniões recorrentes (semanal da equipe, mensal da carteira) geram a pauta com as
ações vencidas e os indicadores do projeto.

### 1.9 Encerramento, lições aprendidas e base de conhecimento

Encerramento com: resultado do Y contra a meta (com intervalo), ganhos validados, plano de
controle transferido e aceito pelo dono do processo, lições aprendidas (o que funcionou, o que
não funcionou, o que repetir) e etiquetas (processo, área, tipo de problema, causa raiz,
solução, ferramentas usadas).

**Base de conhecimento**: busca por texto e por etiqueta em todos os projetos encerrados da
carteira, com acesso aos A3, aos FMEA e às soluções; "projetos parecidos" na abertura de um
novo projeto (mesmo processo ou mesmas etiquetas). O índice de busca é um cache local,
reconstruível, nunca sincronizado (seção 4.4).

### 1.10 Controle de ganhos

- **Categorias**: hard savings (redução de custo com efeito no resultado, aumento de receita) e
  soft savings (custo evitado, capacidade liberada, ganho de tempo sem redução de quadro,
  melhoria de segurança ou satisfação), com a definição da empresa no modelo (seção 4.7).
- **Estados**: `estimado` (belt, no termo) → `previsto` (validado pela controladoria no
  Analisar ou Melhorar) → `realizado` (lançamentos mensais após a implantação) → `validado`
  (controladoria confirma o realizado). Cada valor carrega a memória de cálculo (fórmula,
  premissas, fonte do dado) e quem validou e quando.
- **Acompanhamento** mensal por 12 meses (padrão configurável) depois do Controlar, com
  previsto × realizado; o realizado pode vir do indicador do plano de controle (seção 3.5)
  vezes o valor unitário definido pela controladoria.
- **Ganho estimado por simulação** (seção 3.4): aparece como `estimado` com a origem
  "simulação" e o intervalo dos cenários; nunca passa a `validado` sem dado real.
- A carteira soma ganhos por estado, área, ano e belt.

---

## 2. Ferramentas por fase

Cada ferramenta tem o que calcula, onde mora e como é testada. "Ref." indica o resultado de
referência; "[conferir]" marca o que precisa ser confirmado no texto original antes de virar
teste. Tolerância dos testes numéricos: com valores certificados (NIST StRD), o erro relativo
logarítmico (LRE) deve ser ≥ 9 dígitos nos conjuntos de dificuldade baixa e média e ≥ 6 nos de
dificuldade alta, critério usual na literatura de precisão numérica [conferir]; com exemplos de
livro, a precisão impressa no livro.

### 2.1 Definir

| Ferramenta | O que faz | Teste |
|---|---|---|
| VOC e árvore CTQ | registro das falas do cliente (fonte, segmento), agrupamento em necessidades, desdobramento em direcionadores e CTQs mensuráveis (definição operacional, unidade, LIE/LSE, meta); Kano opcional (obrigatório, desempenho, atrativo) | estrutura: cada CTQ com unidade e ao menos um limite; árvore sem ciclos; golden do JSON |
| SIPOC | fornecedores, entradas, 5 a 7 etapas, saídas, clientes; manual ou gerado do modelo UTSim (seção 3.1) | golden sobre os exemplos de `examples/` |
| Mapa de processo de alto nível | fluxograma com os símbolos da engenharia de métodos (operação, transporte, inspeção, espera, armazenagem) | reaproveita os testes de `utsim-diagram` |
| Pareto | ordenação, percentual e acumulado, linha de 80 %, categoria "outros" abaixo de um limiar; também ponderado por custo | exemplos com acumulado conhecido; propriedade: acumulado termina em 100 % |
| Matriz de partes interessadas | influência × interesse, estratégia de comunicação | nenhum cálculo |

### 2.2 Medir

*Feito em `utsim-quality` (comando `utsim qualidade`, manual "Qualidade"): MSA por ANOVA e por médias e amplitudes, viés e linearidade, atributos com Cohen e Fleiss; capacidade normal, Box-Cox e percentis; DPMO e nível sigma; Anderson-Darling; cartas com fase I e II, Western Electric e Nelson. Faltam: estudo do tipo 1, transformação de Johnson e gráfico de probabilidade normal. Os exemplos do manual de MSA não foram embutidos (material protegido): os testes usam somas de quadrados calculadas à mão e estudos simulados com componentes conhecidos.*

| Ferramenta | O que calcula | Teste |
|---|---|---|
| Plano de coleta de dados | Y e X, definição operacional, tipo (contínuo, atributo, contagem), unidade, estratificação, amostragem (tamanho, frequência, subgrupo), quem, onde, formulário de coleta gerado em planilha | tamanho de amostra pelas fórmulas de poder (seção 2.3) contra exemplos de Montgomery, *Design and Analysis of Experiments* [conferir capítulo] |
| Estatística descritiva | n, média, mediana, desvio, variância, CV, assimetria e curtose (estimadores ajustados), quartis (definição declarada no relatório), IC da média, da mediana (por ordem) e do desvio (qui-quadrado) | NIST StRD, univariados (Lew, Lottery, Mavro, Michelso, NumAcc1 a NumAcc4): média, desvio e autocorrelação lag 1 certificados |
| Gráficos | histograma (regra de Freedman-Diaconis ou Sturges, declarada), boxplot (bigodes 1,5 IQR, atípicos), série temporal com mediana (gráfico de sequência), dispersão com reta e banda, gráfico de probabilidade normal, Pareto, gráfico de valores individuais por grupo | dados de saída dos gráficos (posições, contagens por classe) testados; desenho em SVG com golden |
| Teste de corridas no gráfico de sequência | agrupamento, mistura, tendência e oscilação | já existe teste de corridas em `utsim-fit::diagnostics`; reaproveitar |
| MSA, Gage R&R por ANOVA | ANOVA de dois fatores cruzados (peça, avaliador, interação) com repetições; componentes de variância (repetibilidade, reprodutibilidade, interação, peça a peça); %contribuição, %VE da variação total, %tolerância (6σ ou 5,15σ, declarado), número de categorias distintas `ndc = 1,41 · σ_peça / σ_GRR`; interação retirada quando p > 0,25 (convenção AIAG [conferir]); componentes negativos truncados em zero com aviso | exemplo de 10 peças × 3 avaliadores × 3 repetições do manual AIAG MSA, 4ª ed. [conferir números e edição]; NIST StRD ANOVA para a parte de somas de quadrados; propriedade: dados sem efeito de avaliador dão reprodutibilidade ≈ 0 |
| MSA, médias e amplitudes | repetibilidade por `R̄ / d2*`, reprodutibilidade pela amplitude das médias dos avaliadores com correção pela repetibilidade, peça a peça pela amplitude das médias das peças; constantes `d2*` calculadas por integração da distribuição da amplitude, com a tabela publicada como teste | mesmo exemplo AIAG [conferir]; consistência com o método ANOVA (diferença pequena e explicada no relatório) |
| MSA, estudo do tipo 1, vício e linearidade | vício contra referência com teste t, regressão do vício contra a referência (linearidade) | exemplos do manual AIAG [conferir] |
| MSA por atributos | concordância dentro do avaliador, entre avaliadores e com o padrão (percentual com IC exato de Clopper-Pearson); kappa de Cohen (2 avaliações) e de Fleiss (várias), com erro padrão e teste; kappa por categoria | Cohen (1960) e Fleiss (1971), exemplos dos artigos [conferir tabelas]; casos limite: concordância perfeita dá κ = 1 |
| Capacidade, dado normal | σ dentro (`R̄/d2`, `S̄/c4` ou `MR̄/d2`, declarado) e σ global (desvio amostral); Cp, Cpk, Cpu, Cpl, Pp, Ppk, Cpm; IC de Cpk pela aproximação de Bissell; PPM e DPMO observados e esperados; Z benchmark; nível sigma | exemplos de Montgomery, *Introduction to Statistical Quality Control*, capítulo de capacidade [conferir edição e capítulo]; propriedades: Cpk = min(Cpu, Cpl), Cpk = Cp com média centrada |
| Capacidade, dado não normal | (a) método dos percentis com a distribuição ajustada pelo `utsim-fit` (`(LSE − P50)/(P99,865 − P50)` e equivalente inferior), como proposto em E6 do plano do gêmeo; (b) transformação Box-Cox (λ por máxima verossimilhança de perfil, IC de λ, arredondamento opcional para valores usuais) e Johnson (famílias SB, SL e SU, escolha pela regra de Slifker e Shapiro (1980) [conferir]) aplicadas aos dados e aos limites; o relatório mostra normalidade antes e depois | Box-Cox: exemplo do NIST/SEMATECH e-Handbook [conferir seção] e amostras lognormais simuladas (λ ≈ 0); Johnson: amostras SB e SU simuladas, dado transformado aprovado no A-D em ≈ 1 − α das sementes; percentis: igual ao clássico em dado normal |
| DPMO e nível sigma | `DPMO = defeitos / (unidades × oportunidades) × 10⁶`; DPU; rendimento de primeira passagem e rendimento encadeado (RTY); `nível sigma = Φ⁻¹(1 − DPMO/10⁶) + 1,5`, com o deslocamento de 1,5 explícito e desligável | valores clássicos: 3,4 DPMO ↔ 6,0; 233 ↔ 5,0; 6.210 ↔ 4,0; 66.807 ↔ 3,0 |
| Normalidade | Anderson-Darling com parâmetros estimados (estatística modificada `A² (1 + 0,75/n + 2,25/n²)` e p-valor pelas fórmulas de D'Agostino e Stephens (1986) [conferir]); gráfico de probabilidade; Ryan-Joiner ou Shapiro-Wilk como opção (P3) | valores críticos de Stephens (1974) [conferir]; taxa de erro tipo I ≈ α em amostras normais simuladas, no padrão de `utsim-fit/tests/recovery.rs` |
| Cartas de controle | I-MR, X̄-R, X̄-S, p, np, c, u (limites variáveis com n variável), com fase I (estimar limites, retirar pontos com causa atribuível registrada) e fase II (limites congelados); constantes A2, A3, B3, B4, D3, D4, d2, c4 calculadas e conferidas com a tabela; regras de Western Electric (1956) e de Nelson (1984) selecionáveis uma a uma; EWMA e CUSUM tabular (os mesmos de E4 do plano do gêmeo) | exemplos de Montgomery, *Introduction to Statistical Quality Control* (X̄-R dos anéis de pistão, p e u) [conferir]; séries sintéticas com cada padrão plantado; ARL sob controle de ≈ 370 para a regra 1 a 3σ por simulação |
| VSM atual | caixas de processo com C/T, C/O, disponibilidade, operadores e turnos; triângulos de estoque; escada do lead time (espera × agregação de valor); takt; manual ou gerado do modelo (seção 3.1) | escada: soma das esperas pela lei de Little igual ao lead time do modelo em linha determinística e em rede de Jackson (resultado analítico conhecido); golden do SVG |
| Carta de processo | passos com símbolo, tempo, distância e recursos; totais por símbolo | reaproveita `utsim-diagram::carta` |

### 2.3 Analisar

| Ferramenta | O que calcula | Teste |
|---|---|---|
| Ishikawa | causas por categoria (6M ou personalizadas), subcausas, marcação de causa provável e verificada, ligação de cada causa à análise que a confirmou ou descartou | estrutura e exportação; nenhum cálculo |
| 5 porquês | cadeia de porquês por efeito, com evidência em cada nível e ramificação | idem |
| Matriz causa-efeito | entradas (X) × saídas (Y) com peso do cliente e correlação 0-1-3-9; pontuação ponderada e Pareto dos X | exemplo calculado à mão; propriedade de ordenação |
| FMEA de processo | modo de falha, efeito, causa, controles de prevenção e detecção; S, O, D de 1 a 10 com escalas da empresa; RPN = S × O × D; ação prioritária (AP) alta, média ou baixa por tabela S × O × D; ações recomendadas viram cartões; S, O, D revistos depois da ação | RPN trivial; AP: a tabela cobre as 1000 combinações e a consulta é testada em toda a grade. A tabela oficial do manual AIAG e VDA (2019) é material protegido por direito autoral: o app traz a lógica, uma tabela padrão própria e a opção de cadastrar a tabela que a empresa tem direito de usar [conferir] |
| Testes t | uma amostra, duas amostras (Welch como padrão, combinada como opção, graus de liberdade de Satterthwaite), pareado; IC da diferença, tamanho de efeito (d de Cohen), poder | exemplos de livro com t, gl e p conhecidos [conferir fonte]; propriedade: pareado igual a uma amostra sobre as diferenças; p-valores contra `t_cdf` |
| Variâncias | F para duas, Bartlett e Levene (Brown-Forsythe, mediana) para várias | NIST/SEMATECH e-Handbook, exemplos de Bartlett e Levene [conferir] |
| ANOVA | um fator (com Welch para variâncias desiguais), dois fatores com interação, comparações de Tukey HSD, Bonferroni, Dunnett contra controle (P3); resíduos | NIST StRD ANOVA (SiRstv, SmLs01 a SmLs09, AtmWtAg), valores certificados; amplitude estudentizada contra tabela publicada (já previsto no item 9 do plano de análises) |
| Qui-quadrado | aderência, independência em tabela r × c, resíduos ajustados; Fisher exato em 2 × 2 (enumeração hipergeométrica) | exemplos de Agresti, *Categorical Data Analysis* [conferir]; Fisher por enumeração |
| Proporções | uma e duas proporções, exato (binomial, Clopper-Pearson via `beta_inc`) e aproximação normal, Wilson (já em `utsim-pert::math`, a consolidar) | casos com resposta conhecida; propriedade: Clopper-Pearson contém Wilson em n pequeno |
| Não paramétricos | sinal, Wilcoxon de postos sinalizados, Mann-Whitney (exato para n pequeno, normal com correção de empates), Kruskal-Wallis, mediana de Mood, Friedman | exatos contra enumeração completa em n pequeno; exemplos de Hollander e Wolfe, *Nonparametric Statistical Methods* [conferir] |
| Poder e tamanho de amostra | t de uma e duas amostras, pareado, proporções, ANOVA de um fator (t não central e F não central por série ou integração) | tabelas de Cohen, *Statistical Power Analysis for the Behavioral Sciences* [conferir]; simulação: poder estimado por Monte Carlo dentro de ±1 p.p. |
| Correlação | Pearson com IC por Fisher z, Spearman, matriz com cores (item 10 do plano de análises) | NIST StRD (Norris, correlação implícita); propriedade de invariância a transformações monótonas no Spearman |
| Regressão | simples, múltipla e polinomial por decomposição QR; R², R² ajustado, IC e IP, teste dos coeficientes, VIF, resíduos (normalidade, contra ajustado, ordem), pontos influentes (alavanca, Cook); seleção passo a passo e melhores subconjuntos (P3); logística binária por IRLS (P3) | NIST StRD regressão linear: Norris, Pontius, Longley, Filip (alta dificuldade), Wampler1 a Wampler5; logística contra exemplo de Hosmer e Lemeshow [conferir] |
| Gargalo, gargalo móvel, PERT observado, espaguete | seção 3.3 | testes das crates de origem |

### 2.4 Melhorar

| Ferramenta | O que calcula | Teste |
|---|---|---|
| DOE, fatorial completo | 2^k e fatorial geral; pontos centrais; blocos; aleatorização com semente registrada; planilha de coleta | matriz de planejamento: ortogonalidade e balanço testados por propriedade |
| DOE, fracionado | 2^(k−p) com geradores, estrutura de confusão (aliases) e resolução; Plackett-Burman | estrutura de confusão contra as tabelas de Box, Hunter e Hunter, *Statistics for Experimenters* [conferir]; propriedade: a palavra definidora gera o mesmo conjunto de aliases que a enumeração |
| DOE, superfície de resposta | composto central (rotacionável, de face, inscrito) e Box-Behnken; modelo quadrático; ponto estacionário e análise canônica; gráficos de contorno; otimização de várias respostas por desejabilidade de Derringer e Suich (1980) | exemplos de Montgomery, *Design and Analysis of Experiments* (fatorial 2^4 da taxa de filtração e sua meia fração; superfície de resposta) [conferir números dos exemplos]; Myers, Montgomery e Anderson-Cook, *Response Surface Methodology* [conferir] |
| DOE, análise | efeitos e coeficientes; ANOVA quando há réplicas ou pontos centrais; experimentos sem réplica pelo pseudo erro padrão de Lenth (1989); gráfico normal e de Pareto dos efeitos; efeitos principais e interações; curvatura pelos pontos centrais | Lenth (1989), exemplo do artigo [conferir]; Montgomery (mesmos exemplos) |
| Seleção de soluções | matriz de critérios ponderados, matriz de Pugh, esforço × impacto | cálculo trivial |
| Comparação antes × depois | testes da seção 2.3 sobre dado real do piloto; carta de controle com limites recalculados no ponto de mudança | idem 2.3 |
| Cenários e DOE no simulador | seção 3.4 | idem 3.4 |
| Poka-yoke | registro do dispositivo (tipo: contato, valor fixo, sequência; função: controle ou alerta), modo de falha do FMEA que ele trata, verificação periódica | ligação com o FMEA: a ação reduz O ou D, revisão registrada |
| 5S | auditoria por senso com nota (lista de verificação da empresa), gráfico radar e histórico de auditorias; fotos antes e depois como anexos | soma e média das notas |
| SMED | lista das atividades do setup com tempo, classificação interna e externa, conversão e redução; tempo de setup antes e depois; ligação ao parâmetro de setup do modelo UTSim | soma por classe; caso exemplo à mão |
| Trabalho padronizado | folha de trabalho padrão (sequência, tempos manual, automático e de deslocamento), tabela de combinação do trabalho padrão com o takt, estoque padrão em processo; balanceamento da linha (gráfico de barras dos operadores contra o takt) | totais e ocupação contra cálculo à mão; balanceamento: número mínimo de operadores = ⌈Σ conteúdo / takt⌉ |

### 2.5 Controlar

| Ferramenta | O que faz | Teste |
|---|---|---|
| Plano de controle | por característica: processo, característica (produto ou processo), especificação, método de medição (ligado ao MSA), tamanho e frequência da amostra, método de controle (carta, checklist, poka-yoke), responsável, plano de resposta; estrutura usual do manual AIAG APQP e Plano de Controle [conferir edição] | estrutura; cada característica crítica do FMEA (S alto) tem linha no plano (verificação do tollgate) |
| Plano de resposta | para cada sinal (regra de carta violada, limite de especificação, alarme do gêmeo): ação imediata, contenção, quem acionar, prazo; o sinal gera cartão no kanban | sinal sintético gera o cartão esperado |
| Cartas em operação | as mesmas da seção 2.2 em fase II, sobre dado real ou do gêmeo (seção 3.5) | idem 2.2 |
| Capacidade final e comparação com a linha de base | seção 2.2; diferença com IC | idem |
| Transferência | aceite formal do dono do processo; documentos atualizados como anexos | estado do aceite |

---

## 3. Integração com o UTSim

O diferencial do app LSS é usar o modelo de simulação como instrumento do projeto: mapear o
processo a partir do modelo, medir sobre o gêmeo, encontrar a causa no modelo e testar a
solução no modelo antes de mexer na fábrica.

### 3.1 Importar o modelo e gerar os mapas

O projeto guarda uma **ligação ao modelo**: caminho relativo do `.ron` (copiado para a pasta do
projeto ou referenciado), hash do conteúdo e versão do formato. Se o arquivo muda, o app avisa
quais mapas e análises dependem da versão antiga.

- **Fluxograma e carta de processo**: `utsim_diagram::flowchart(model, options)` e
  `carta::build`, com os números de uma rodada (`stats_from_run`). Depende da integração do
  branch de `utsim-diagram`.
- **SIPOC** (módulo novo `utsim-diagram::sipoc`): fornecedores e entradas a partir das `Source`
  (nome, rótulos dos itens, distribuição de chegada) e dos fluxos de fluido; processo como 5 a 7
  macroetapas, agrupando o caminho principal da carta de processo por grupos do modelo ou por
  corte nas filas; saídas e clientes a partir das `Sink` (incluindo refugo, quando marcado). O
  belt edita e o SIPOC fica como entregável próprio, com a origem "gerado do modelo, editado".
- **VSM atual** (módulo novo `utsim-diagram::vsm`): por processador, C/T = média do tempo de
  processo, C/O = setup, disponibilidade a partir de MTBF/MTTR e turnos, operadores das equipes;
  estoques = conteúdo médio das filas na rodada; escada do lead time com esperas pela lei de
  Little (`utsim-stats::little`) e agregação de valor pelos tempos de processo; takt calculado a
  partir da demanda informada. Fluxos de informação e de material (empurrado, puxado,
  supermercado) desenhados à mão sobre o VSM gerado.
- **VSM futuro**: o mesmo gerador aplicado ao modelo do cenário escolhido na fase Melhorar,
  lado a lado com o atual e com as diferenças do lead time e do WIP.
- **Espaguete**: deslocamentos dos operadores e transportadores a partir dos eventos `Travel` e
  `Transit` do rastro (o mesmo cálculo do mapa de calor, item 11 do plano de análises, em
  `utsim-scene`), desenhados sobre a planta CAD (`utsim-cad`) quando houver; distância total e
  por recurso.

### 3.2 Medir: dados reais, do gêmeo ou simulados

- **Real**: CSV e planilha importados (seção 4.4), com o diagnóstico de `utsim-fit::diagnostics`
  (independência, estacionariedade, atípicos, arredondamento) antes de qualquer índice.
- **Gêmeo**: leitura das gravações de observações do `utsim-twin` (formato longo
  `instante;tag;valor;qualidade`, seção 1.2 do plano do gêmeo) ou da API JSON do painel em
  `127.0.0.1`. O dado entra como conjunto com origem `Gêmeo`, com a janela e a qualidade.
- **Simulado**: KPIs por replicação de um modelo validado (`per_replication`,
  `metric_values`), com origem `Simulado`; útil para a linha de base quando ainda não há coleta
  e para responder "quanto o processo varia por natureza". Sempre rotulado.
- **Ajuste de distribuições do dado coletado**: o belt manda o resultado do `utsim-fit` para o
  modelo (a linha `Dist` do `.ron`), fechando o ciclo coleta → modelo.

### 3.3 Analisar: causas no modelo

- **Gargalo** (`analysis::bottleneck`, `bottleneck_in_shift`) e **gargalo móvel**
  (`shifting_bottleneck`), com a fração do tempo como gargalo único e compartilhado.
- **Caminho crítico observado** (`utsim_pert::observed::run`) de uma parte do processo, com os
  tempos de espera em filas e transportes: responde "o que atrasa este trecho".
- **Lei de Little** e **séries de WIP** para separar espera de processamento no lead time.
- **Análise de sensibilidade** (Monte Carlo do PERT e, depois, metamodelo do item 14 do plano
  de análises): quais parâmetros do modelo mais mexem no Y. O resultado entra na matriz C&E e
  no Ishikawa como evidência ("causa verificada no modelo"), distinguida de "causa verificada
  com dado real".

### 3.4 Melhorar: testar antes de mexer na fábrica

- **Cenários**: o belt descreve a solução como alterações de parâmetros (`set` do
  `Experiment`) ou como um modelo alternativo; o app roda as replicações com números aleatórios
  comuns e compara com a linha de base por diferença pareada (`paired_difference_at`), com IC e
  decisão "significativo e relevante" (diferença mínima de interesse informada pelo belt).
  Vários cenários: ANOVA e Tukey ou comparações pareadas com Bonferroni.
- **DOE no simulador**: fatores = parâmetros do modelo com níveis; o gerador da seção 2.4
  produz os cenários (o mesmo que o item 13 do plano de análises prevê para
  `utsim experiment`); respostas = KPIs; réplicas = replicações com sementes registradas; a
  análise é a mesma do DOE físico, com a observação de que o erro é o da variabilidade simulada.
  Superfície de resposta para ajustar, por exemplo, número de operadores e tamanho de lote.
- **Otimização** (`utsim-opt`) quando o espaço de soluções é grande, com a solução final
  comparada pareadamente com a linha de base.
- **Pacote de evidência**: cada estudo grava um arquivo `estudo.json` com hash do modelo,
  cenários, sementes, número de replicações, versão do UTSim e resultados; o relatório do
  tollgate cita o pacote, e qualquer pessoa reproduz o número.
- **Ganho estimado**: diferença de vazão, lead time, WIP ou custo (quando o modelo tiver custos)
  vezes o valor unitário da controladoria, com o IC, entra como saving `estimado` de origem
  "simulação".
- **Piloto real** depois do simulador: comparação antes × depois com os testes da seção 2.3; e
  comparação do resultado real com o previsto pelo modelo (E7 do plano do gêmeo), que valida o
  modelo para os próximos projetos.

### 3.5 Controlar: painel e CEP sobre o gêmeo

- O plano de controle aponta, por característica, para um KPI do painel (`KpiSet` de
  `utsim-kpi`) ou uma tag do gêmeo.
- O app lê periodicamente (quando aberto) a API JSON do painel em `127.0.0.1` ou as gravações em
  arquivo, e mantém as cartas de fase II do projeto; os limites podem vir do dado real da fase
  Controlar ou do modelo validado (limites a partir do modelo, E4 do plano do gêmeo).
- Sinal de carta ou alerta do painel gera o cartão do plano de resposta, com o trecho da série.
- Os indicadores do plano de controle alimentam o realizado dos ganhos (seção 1.10).

### 3.6 Formato de troca e crates comuns

**Biblioteca, não processo externo.** Os dois apps são Rust; o app LSS chama as crates do
simulador diretamente (ler `.ron`, rodar replicações e cenários em fios de trabalho, derivar
diagramas). Arquivos só onde há fronteira de pessoa ou de tempo:

| Troca | Formato | Direção |
|---|---|---|
| Modelo | `.ron` do UTSim (formato versionado), referenciado com hash | UTSim → LSS |
| Pacote de evidência | `estudo.json` (seção 3.4) com versão de esquema | ambos |
| Dados de coleta e do gêmeo | CSV (formato longo para séries), JSONL das gravações | gêmeo → LSS |
| KPIs ao vivo | API JSON do painel (`utsim-dash`) em `127.0.0.1` | painel → LSS |
| Abrir no simulador | linha de comando do UTSim com o caminho do modelo e, opcional, o cenário; via launcher, que sabe onde cada app está instalado | LSS → UTSim |
| Enviar para o projeto | o UTSim grava um `estudo.json` na pasta `dados/` de um projeto escolhido | UTSim → LSS |

**Crates comuns** (puras, sem interface, compilam para wasm):

| Crate | Conteúdo | Usada por |
|---|---|---|
| `utsim-stats` (existe; cresce) | base: funções especiais, quantis, descritiva, testes de hipótese (t, F, variâncias, ANOVA, Tukey, qui-quadrado, Fisher, proporções, não paramétricos), normalidade A-D, poder, correlação, regressão (QR), Box-Cox e Johnson; consolida `norm_cdf`, `quantile_sorted`, `correlation` e `wilson`, hoje duplicados em `utsim-pert::math` | simulador, painel, LSS |
| `utsim-quality` (**criada**; falta Johnson e o estudo do tipo 1) | cartas de controle e regras, EWMA, CUSUM, ARL por simulação, capacidade (normal, percentis, transformada), DPMO e nível sigma, MSA (ANOVA, médias e amplitudes, atributos, kappa) | painel (E4, E6), LSS |
| `utsim-doe` (nova; ou módulo `doe` de `utsim-stats`, decisão C1) | geração de planejamentos (fatorial, fracionado com aliases, Plackett-Burman, CCD, Box-Behnken), análise de efeitos, Lenth, superfície de resposta, desejabilidade | `utsim experiment`, LSS |
| `utsim-diagram` (no branch; cresce) | fluxograma e carta (existem), SIPOC, VSM, espaguete | simulador, LSS |
| `utsim-pert` (existe) | CPM, PERT e Monte Carlo do cronograma | simulador, LSS |
| `utsim-fit` (existe) | ajuste de distribuições para capacidade não normal | simulador, LSS |

`utsim-stats` depende só de `utsim-core` (por `Summary` e `t_975`), o que é leve (sem motor
gráfico). `utsim-quality` depende só de `utsim-stats`: o método dos percentis recebe a função quantil, e quem chama usa a distribuição do `utsim-fit` (o `utsim qualidade` faz assim). Nenhuma crate comum
depende de crate do app LSS.

**Crates do app LSS**:

- `utsim-lss-core` (pura): domínio (carteira, projeto, fases, entregáveis, tollgates, papéis,
  ações, riscos, reuniões, ganhos, lições), modelos de fluxo, regras dos checklists verificados,
  calendário, receitas de análise; serialização e migrações do formato do projeto.
- `utsim-lss-store` (nativa): pasta do projeto, diário de operações, bloqueio otimista,
  instantâneos, anexos por hash, índice local de busca (seção 4.5).
- `utsim-lss-app` (binário): servidor HTTP local, API JSON, página embutida, fios de trabalho
  para simulações, exportações.

---

## 4. Arquitetura

### 4.1 Repositório

| Opção | A favor | Contra |
|---|---|---|
| Mesmo workspace (crates `utsim-lss-*` em `crates/`) | uma mudança na estatística e no app no mesmo commit; testes e `scripts/verificar` cobrem tudo; sem publicar versões das crates comuns; mesma CI e mesmo empacotamento | repositório maior; a versão do workspace (`version.workspace`) é compartilhada e precisa ser separada por produto; tags de publicação por produto |
| Repositório próprio dependendo das crates por `git` com tag | ciclos de publicação independentes; repositório pequeno | toda mudança numa crate comum exige tag no UTSim e atualização no LSS; dois lugares para a CI; risco de versões divergentes da estatística entre os apps |

**Recomendação: mesmo workspace** (decisão R1), com:

- as crates do LSS com `version` próprio (não `version.workspace`), começando em `0.1.0`;
- tags de publicação com prefixo por produto (`v0.2.0` para o simulador, `lss-v0.1.0` para o
  app LSS), cada uma disparando só o empacotamento do seu produto;
- `utsim-lss-app` fora de `default-members` se o tempo de build pesar; as crates puras dentro;
- regra de dependência conferida na CI: crates comuns nunca dependem de `utsim-lss-*`.

Se no futuro o app LSS precisar de um time separado, as crates comuns já estarão isoladas e a
separação é mecânica.

### 4.2 Interface

| Opção | A favor | Contra |
|---|---|---|
| egui/eframe | um binário, desenvolvimento rápido em Rust, bom para ferramentas de engenharia | formulários longos, texto rico, tabelas editáveis grandes, impressão e acessibilidade são fracos; aparência pouco familiar para gestores; nada compartilhado com o painel web |
| Godot | já usado no simulador; bom para 3D | feito para cena e jogo, não para documentos e formulários; impressão e texto rico ruins; executável maior; o 3D não é necessário aqui (o app abre o UTSim quando preciso) |
| Web servida localmente | formulários, tabelas, kanban, Gantt, texto rico e impressão em PDF são o terreno natural do navegador; gestores já usam navegador; tablet no chão de fábrica pela rede local; mesmo módulo de gráficos SVG e mesma tecnologia do painel (decisões D1, D2 e D8 já aprovadas); as crates puras compilam para wasm e permitem um modo só navegador | duas linguagens (Rust e JavaScript); servidor local a manter; navegador como dependência de execução |

**Recomendação: web servida localmente** (decisão I1), no mesmo padrão do `utsim-dash`:

- servidor HTTP mínimo sobre `std::net` no próprio binário, escutando em `127.0.0.1` por
  padrão; página (HTML, CSS, JavaScript em módulos ES) embutida com `include_str!`, sem CDN,
  funcionando sem internet; JavaScript puro sem etapa de build, como `web/`;
- o app abre o navegador padrão no endereço local; opção futura de janela própria por uma
  visão web nativa do sistema operacional (decisão I2), sem mudar a página;
- o módulo de gráficos SVG é o mesmo do painel (um diretório compartilhado de JavaScript na
  árvore, servido pelos dois binários), e os dados dos gráficos chegam calculados do Rust;
- acesso pela rede local (tablet no gemba) só com opção explícita e token, como na seção 2.10 do
  plano do gêmeo;
- componentes de interface próprios e pequenos (tabela editável, quadro kanban, Gantt, editor de
  árvore, editor de texto simples com listas e negrito). Alternativa: embutir uma biblioteca
  pequena de componentes, de código aberto e sem build (decisão I3).

### 4.3 Modelo de dados

Entidades principais: `Carteira`, `Projeto` (abordagem, fase atual, termo versionado), `Fase`,
`Entregavel` (tipo de ferramenta, conteúdo, estado), `Tollgate` (checklist, decisões, hash
congelado), `Pessoa` e `Papel`, `Acao`, `Risco`, `Reuniao`, `Ganho` (lançamentos e validações),
`ConjuntoDeDados` (origem, hash, colunas), `Analise` (receita e resultado em cache),
`LigacaoModelo`, `Estudo`, `Licao`, `Anexo`. Identificadores são UUID v7 (ordenáveis pelo tempo,
gerados sem servidor).

### 4.4 Armazenamento local

| Opção | A favor | Contra |
|---|---|---|
| Arquivo SQLite por projeto | consultas, transações, um arquivo | arquivo único binário numa pasta sincronizada: o cliente de sincronização copia o arquivo inteiro, não entende o diário de transações, e duas pessoas gravando geram cópia em conflito do arquivo todo; diferenças ilegíveis |
| JSON/RON em pasta, um arquivo por entidade | legível, diferença por arquivo, conflito localizado | duas pessoas editando a mesma entidade ainda colidem |
| Pasta com diário de operações em JSON (só acréscimo) e instantâneos | cada gravação é um arquivo novo, que a sincronização nunca sobrescreve; histórico completo grátis; conflitos detectados e resolvidos por regra | reconstruir o estado pede ler o diário (mitigado por instantâneos) |

**Recomendação: pasta de projeto com diário de operações em JSON** (decisão S1). JSON e não
RON porque a interface é web e o formato é de dados, não de modelo; esquema com `formato: N` e
migrações no padrão de `docs/versionamento.md`.

```
Melhoria da linha 2.lss/
  projeto.json                 manifesto: id, formato, título, abordagem, fase, resumo para a carteira
  instantaneos/                estado completo até a operação X (gerado por qualquer pessoa)
    000184-<uuid>.json
  ops/                         diário: um arquivo por operação, nunca alterado
    2026/11/<uuid-v7>-<pessoa>.json
  dados/                       conjuntos importados (CSV imutável; nova versão = novo arquivo) e estudo.json
  anexos/                      por hash: <sha256>.<ext> (fotos, PDFs, planilhas)
  modelos/                     cópia do .ron ligado, quando copiado
  presenca/<pessoa>.json       quem está com o projeto aberto e em que tela (pulsação)
```

Uma operação é `{id, autor, instante, relogio_lamport, entidade, campo, valor, base}`, em que
`base` é a versão do campo que o autor viu. O estado é o último instantâneo mais as operações
seguintes, ordenadas por `(relogio_lamport, autor, id)`, o que dá o mesmo resultado em todas as
máquinas. Um cache local fora da pasta sincronizada (índice de busca, estado reconstruído,
resultados de análises) acelera a abertura e pode ser apagado a qualquer momento.

Para enviar um projeto a quem não está na pasta compartilhada: exportação em arquivo único
`.lssz` (zip da pasta) e importação.

### 4.5 Trabalho em equipe a custo zero

- **Pasta compartilhada no Google Drive** da equipe, sincronizada pelo cliente de desktop do
  próprio Drive; a carteira é uma pasta com uma subpasta `.lss` por projeto e uma pasta
  `modelos/` com os modelos da empresa (seção 4.7). O app só lê e grava arquivos locais; não
  usa API do Drive nem servidor.
- **Bloqueio otimista**: cada operação leva a versão de base; na reconstrução, se a versão
  atual do campo já não é a base (alguém gravou antes), há conflito. Regra: vence a operação de
  maior relógio de Lamport, e o conflito vai para uma lista "conflitos a revisar" com os dois
  valores e um botão para restaurar o outro. Textos longos (atas, descrições) têm conflito por
  parágrafo, não pelo campo inteiro.
- **Presença**: cada pessoa grava só o próprio arquivo de presença; a interface mostra "Ana está
  no FMEA" e pede atenção antes de editar a mesma entidade. É aviso, não trava.
- **Histórico**: o diário é o histórico; qualquer entidade mostra quem mudou o quê e quando, e
  volta a uma versão anterior por uma nova operação.
- **Cópias em conflito do Drive**: como nada é sobrescrito, são raras; o app detecta arquivos
  duplicados com o sufixo do cliente de sincronização e os incorpora como operações comuns.
- **Instantâneos**: gerados quando o diário passa de um limite (por exemplo 500 operações);
  dois instantâneos concorrentes são equivalentes (mesma reconstrução), então não há conflito.
- **Identificação e limites**: sem servidor não há autenticação; a pessoa se identifica no
  primeiro uso (nome e e-mail) e cada operação leva o autor. Integridade: cada operação inclui
  o hash da operação anterior do mesmo autor (cadeia por autor), o que revela remoção ou edição
  de arquivos do diário. Aprovações (tollgates, ganhos) podem ser assinadas com chave ed25519
  por pessoa (P7), reaproveitando a infraestrutura de assinatura do launcher; mesmo assim, não
  equivalem a assinatura digital com validade jurídica, e o manual diz isso.
- **Funciona desconectado**: tudo é local; a sincronização acontece quando o Drive sincronizar.

### 4.6 Exportação

| Saída | Como |
|---|---|
| A3 em PDF | página com `@page { size: A3 landscape }` e impressão pelo navegador (mesma decisão D8 do painel); o app abre a visualização pronta para imprimir |
| Relatório de tollgate em PDF | A4 retrato: resumo da fase, checklist com evidências, gráficos SVG vetoriais, receitas das análises, decisões e ressalvas |
| Termo de abertura, ata, plano de controle, FMEA, relatório de encerramento | mesmas folhas de estilo de impressão |
| Planilha | `.xlsx` com uma aba por tabela (ações, riscos, FMEA, plano de controle, ganhos, dados), com `rust_xlsxwriter` atrás de feature, como D9 do painel; CSV sempre |
| Imagens | cada diagrama e gráfico em SVG e PNG |
| Projeto inteiro | `.lssz` (seção 4.4) e JSON do estado |

Leitura de planilhas na importação de dados (`.xlsx`, `.ods`) por um leitor em Rust puro atrás
de feature; CSV com detecção de separador e de vírgula decimal.

### 4.7 Modelos de documentos e personalização

A empresa mantém na pasta `modelos/` da carteira, sem código: modelos de fluxo (fases,
entregáveis, checklists, aprovadores), escalas do FMEA (S, O, D) e a tabela de AP, critérios e
pesos da priorização, objetivos estratégicos, categorias e regras de ganhos, campos extras do
termo, categorias do Ishikawa, listas de 5S, logotipo e cabeçalho dos relatórios. O app traz um
conjunto padrão (DMAIC conforme ISO 13053, DMADV, kaizen, A3, PDCA). Os modelos são
versionados; um projeto registra a versão do modelo com que nasceu.

### 4.8 Idioma e distribuição

- **Idioma**: português do Brasil primeiro; todos os textos da interface em catálogos JSON por
  chave desde o início, para inglês e espanhol depois (extração por Haiku, revisão técnica por
  Sonnet, como no backlog). Termos consagrados (CTQ, DPMO, Cpk, SIPOC, VSM) ficam como siglas,
  com glossário. Números e datas no formato do idioma (vírgula decimal em pt-BR).
- **Launcher** (o `docs/plano_launcher.md` ainda não existe): o app LSS é mais um produto no
  manifesto de versões, com canais estável e testes, instalador próprio (NSIS no Windows, como
  o simulador) e a mesma assinatura. O launcher precisa expor onde cada produto está instalado,
  para o botão "abrir no UTSim", e permitir que o app LSS rode sem o simulador.

---

## 5. MVP, fases e estimativas

### 5.1 MVP proposto

Um green belt conduz um DMAIC do termo ao encerramento, com a equipe na pasta compartilhada,
usando as ferramentas essenciais, e importa o modelo UTSim para gerar SIPOC e fluxograma e para
comparar cenários. Ficam fora do MVP: MSA, DOE, regressão, VSM, gêmeo, DMADV e assinatura das
aprovações (decisão M1).

### 5.2 Fases

Horas Opus 5.5 por fase; "Modelo" indica a divisão sugerida (Opus para estatística,
arquitetura e regras de concorrência; Sonnet para interface, relatórios e testes a partir de
plano; Haiku para tarefas mecânicas).

| Fase | Conteúdo | Horas | Modelo |
|---|---|---|---|
| **L0 Fundação** (MVP) | crates `utsim-lss-core`, `-store`, `-app`; formato do projeto, diário, Lamport, instantâneos, conflitos, presença, migrações; servidor local e API; esqueleto da página, navegação, catálogo de textos; módulo de gráficos compartilhado com o painel; entrada no manifesto do launcher | 30–42 | Opus 14–20 (formato, armazenamento, concorrência); Sonnet 15–20 (servidor, página); Haiku 1–2 (CI, catálogo) |
| **L1 Gestão do projeto** (MVP) | carteira, banco de ideias, priorização (matriz, pontuação, alinhamento), escolha da abordagem, termo versionado, papéis e RACI, fases e tollgates com checklists manuais e verificados, kanban de ações, riscos, reuniões e atas, cronograma com Gantt e CPM/PERT do `utsim-pert` e calendário, ganhos com estados e validação, encerramento, lições | 45–62 | Opus 8–12 (regras de tollgate, calendário, ganhos); Sonnet 37–50 (telas) |
| **L2 Ferramentas qualitativas** (MVP) | VOC/CTQ, SIPOC manual, Ishikawa, 5 porquês, matriz C&E, FMEA com RPN e AP configurável, plano de coleta, plano de controle e de resposta, Pareto, A3 | 24–32 | Opus 3–4 (FMEA e C&E); Sonnet 21–28 |
| **L3 Estatística essencial** (MVP), em `utsim-stats` e `utsim-quality` | (**parte feita** em `utsim-quality`: A-D, capacidade normal, por percentis e Box-Cox, DPMO e nível sigma, as sete cartas com Western Electric e Nelson; faltam descritiva, gráficos e os testes de hipótese) descritiva, gráficos (dados), A-D, capacidade normal e por percentis e Box-Cox, DPMO e nível sigma, I-MR, X̄-R, X̄-S, p, np, c, u com Western Electric e Nelson, testes t (três), ANOVA de um fator com Tukey, qui-quadrado, proporções, Mann-Whitney e Wilcoxon; testes de referência (NIST StRD e livros) | 34–46 | Opus 28–38; Sonnet 6–8 (telas de análise) |
| **L4 Integração essencial** (MVP) | ligação ao modelo com hash, fluxograma e carta, SIPOC gerado, conjuntos simulados rotulados, cenários com diferença pareada e pacote `estudo.json`, gargalo e gargalo móvel, PERT observado, abrir no UTSim | 18–26 | Opus 8–12 (SIPOC, estudos, reprodutibilidade); Sonnet 10–14 |
| **L5 Relatórios e modelos** (MVP) | impressão do A3, tollgate, termo, ata, FMEA e plano de controle; `.xlsx` e CSV; modelos da empresa na pasta `modelos/`; base de conhecimento com busca; `.lssz` | 16–22 | Sonnet 15–20; Haiku 1–2 |
| **Total do MVP** | | **167–230** | Opus ≈ 61–86; Sonnet ≈ 104–140; Haiku ≈ 2–4 |
| P1 MSA | (**cálculos feitos** em `utsim-quality`, menos o estudo do tipo 1; falta o checklist) Gage R&R por ANOVA e médias e amplitudes, tipo 1, vício e linearidade, atributos com kappa de Cohen e Fleiss; checklist verificado do Medir | 18–25 | Opus 14–19; Sonnet 4–6 |
| P2 Estatística ampliada | Johnson, Levene e Bartlett, ANOVA de dois fatores, Kruskal-Wallis, Mood, Friedman, Fisher, poder e tamanho de amostra, correlação, regressão múltipla com diagnósticos, logística, seleção de variáveis, EWMA e CUSUM com ARL (**feitos** em `utsim-quality`) | 30–42 | Opus 25–35; Sonnet 5–7 |
| P3 DOE | fatorial, fracionado com aliases, Plackett-Burman, CCD, Box-Behnken, análise, Lenth, superfície e desejabilidade; DOE no simulador (gerador para `Experiment`) | 30–42 | Opus 24–34; Sonnet 6–8 |
| P4 VSM e espaguete | VSM atual e futuro gerados do modelo e editáveis, escada do lead time, espaguete sobre a planta | 20–28 | Opus 8–11; Sonnet 12–17 |
| P5 Controle com gêmeo e painel | leitura da API do painel e das gravações, cartas de fase II, alertas para o plano de resposta, realizado dos ganhos a partir dos KPIs | 12–18 | Opus 5–8; Sonnet 7–10 |
| P6 DMADV e ferramentas Lean | QFD, Pugh, tolerâncias por Monte Carlo, kaizen rápido completo, PDCA, poka-yoke, 5S, SMED, trabalho padronizado e balanceamento | 22–30 | Opus 5–7; Sonnet 17–23 |
| P7 Equipe avançada | conflito por parágrafo, assinatura ed25519 das aprovações, cadeia de hash conferida na abertura, visão "minhas ações" da carteira, acesso pela rede local com token | 12–18 | Opus 7–10; Sonnet 5–8 |
| P8 Idiomas | extração dos textos, inglês e espanhol, revisão de termos | 6–10 | Haiku 2–3; Sonnet 4–7 |
| **Total depois do MVP** | | **150–213** | Opus ≈ 88–124; Sonnet ≈ 60–86; Haiku ≈ 2–3 |
| **Total geral** | | **317–443** | |

**Sobreposição com o plano do gêmeo.** E4 (CEP) e E6 (capacidade) do plano do gêmeo estão
contados lá (bloco 3, 48–67 h). Se forem feitos primeiro em `utsim-quality`, L3 cai cerca de
12–16 h e P2 cerca de 4–6 h (EWMA e CUSUM). Do mesmo modo, o item 13 do plano de análises
(planejamento de experimentos) e P3 deste plano são uma só tarefa.

### 5.3 Ordem sugerida

1. Integrar `utsim-diagram` a `feat/fase-1` (pré-requisito de L4 e P4).
2. L0 e, em paralelo, L3 (não mexem nos mesmos arquivos: L3 só toca `utsim-stats` e a crate
   nova `utsim-quality`), com o painel do gêmeo já consumindo `utsim-quality`.
3. L1 e L2 (interface) com L4.
4. L5 e piloto interno com um projeto real; depois P1, P3, P4 e P5 conforme o uso.

### 5.4 Testes e validação

- Crates comuns: testes contra valores certificados e exemplos publicados (seção 2), testes de
  propriedade (`proptest`, já usado em `utsim-fit`), taxas de erro tipo I e poder por
  simulação, golden dos SVG.
- Armazenamento: testes de concorrência com duas "máquinas" simuladas gravando operações
  intercaladas em ordens diferentes, conferindo que a reconstrução é idêntica e os conflitos
  são listados; cópias em conflito artificiais; perda de arquivos do diário detectada pela
  cadeia de hash.
- Fluxo: um projeto DMAIC de exemplo completo (com dados sintéticos e o modelo
  `examples/centro_de_distribuicao.ron`) como teste de ponta a ponta da API e como tutorial.
- Interface: teste de fumaça da página em navegador sem janela, como `web/smoke.mjs`.

### 5.5 Riscos

- **Escopo**: o pedido cobre o equivalente a vários produtos; sem o corte do MVP o app não sai.
- **Correção estatística**: um índice errado num tollgate tem custo real; por isso todo cálculo
  passa por referência antes de aparecer na interface, e os itens "[conferir]" são resolvidos
  antes de cada fase.
- **Material protegido**: tabelas de manuais (AP do FMEA, critérios de MSA) não são copiadas
  para o programa; o app traz a lógica e permite cadastro.
- **Sincronização do Drive**: atraso de minutos entre máquinas e cópias em conflito; mitigado
  pelo diário só de acréscimo, mas a equipe deve saber que não é edição simultânea em tempo
  real.
- **Duas interfaces web na família** (painel e LSS) podem divergir; mitigado pelo módulo de
  gráficos e componentes compartilhados.

---

## 6. Requisitos sugeridos

### Funcionais

- **LSS-RF-01** Banco de ideias e carteira com estados de ideia a encerrada, em quadro.
- **LSS-RF-02** Priorização por matriz impacto × esforço, pontuação ponderada e alinhamento
  estratégico, com critérios e pesos configuráveis.
- **LSS-RF-03** Escolha da abordagem (ação direta, kaizen, A3, PDCA, DMAIC, DMADV) por roteiro
  de perguntas, com decisão registrada do champion.
- **LSS-RF-04** Termo de abertura versionado, com nova aprovação a cada mudança de meta ou
  escopo.
- **LSS-RF-05** Papéis por projeto e matriz RACI gerada e editável.
- **LSS-RF-06** Fases e entregáveis conforme o modelo de fluxo do projeto.
- **LSS-RF-07** Tollgate com checklist de itens manuais e verificados automaticamente.
- **LSS-RF-08** Decisão de tollgate por aprovador (aprovar, com ressalvas, refazer, suspender,
  cancelar), com regra de aprovação configurável.
- **LSS-RF-09** Congelamento dos entregáveis aprovados por hash e aviso de alteração posterior.
- **LSS-RF-10** Cronograma com tarefas em três pontos, dependências com defasagem, linha de base
  e Gantt.
- **LSS-RF-11** Datas mais cedo e mais tarde, folgas, caminho crítico, probabilidade de cumprir
  cada tollgate e Monte Carlo pelo `utsim-pert`, com calendário de dias úteis.
- **LSS-RF-12** Registro de riscos com matriz probabilidade × impacto e respostas que viram
  ações.
- **LSS-RF-13** Quadro kanban de ações em 5W2H, com origem, limites por coluna e verificação de
  eficácia.
- **LSS-RF-14** Visão "minhas ações" de toda a carteira.
- **LSS-RF-15** Reuniões com pauta gerada, ata, decisões e ações criadas a partir da ata.
- **LSS-RF-16** Ganhos hard e soft com estados estimado, previsto, realizado e validado, memória
  de cálculo e validação pela controladoria.
- **LSS-RF-17** Acompanhamento mensal dos ganhos por período configurável depois do Controlar.
- **LSS-RF-18** Encerramento com resultado contra a meta, aceite do dono do processo e lições
  aprendidas com etiquetas.
- **LSS-RF-19** Base de conhecimento com busca por texto e etiqueta e sugestão de projetos
  parecidos.
- **LSS-RF-20** Modelos de fluxo DMAIC, DMADV, kaizen, A3 e PDCA, e criação de modelos pela
  empresa sem código.
- **LSS-RF-21** VOC e árvore CTQ com definição operacional, unidade e limites.
- **LSS-RF-22** SIPOC manual e gerado do modelo UTSim.
- **LSS-RF-23** Fluxograma e carta de processo a partir do modelo (`utsim-diagram`).
- **LSS-RF-24** VSM atual e futuro gerados do modelo, editáveis, com escada do lead time e takt.
- **LSS-RF-25** Diagrama de espaguete a partir do rastro, sobre a planta CAD quando houver.
- **LSS-RF-26** Plano de coleta de dados com amostragem e formulário em planilha.
- **LSS-RF-27** Importação de CSV e planilha com detecção de separador e vírgula decimal;
  conjuntos imutáveis com hash e origem Real, Gêmeo ou Simulado.
- **LSS-RF-28** Estatística descritiva com IC e gráficos (histograma, boxplot, Pareto,
  dispersão, série, probabilidade normal, valores individuais).
- **LSS-RF-29** MSA por ANOVA e por médias e amplitudes, com %contribuição, %VE, %tolerância e
  ndc.
- **LSS-RF-30** MSA por atributos com concordância e kappa de Cohen e Fleiss.
- **LSS-RF-31** Capacidade Cp, Cpk, Pp, Ppk, Cpm com IC, PPM, DPMO e nível sigma.
- **LSS-RF-32** Capacidade não normal por percentis da distribuição ajustada e por
  transformação Box-Cox e Johnson.
- **LSS-RF-33** Normalidade por Anderson-Darling com p-valor e gráfico de probabilidade.
- **LSS-RF-34** Testes t (uma, duas amostras e pareado), variâncias, ANOVA com comparações
  múltiplas, qui-quadrado, Fisher, proporções e não paramétricos, com tamanho de efeito.
- **LSS-RF-35** Poder e tamanho de amostra para os testes principais.
- **LSS-RF-36** Correlação e regressão simples, múltipla e polinomial com diagnósticos.
- **LSS-RF-37** DOE fatorial, fracionado com estrutura de confusão, Plackett-Burman, composto
  central e Box-Behnken, com análise e otimização por desejabilidade.
- **LSS-RF-38** Ishikawa, 5 porquês e matriz causa-efeito, com ligação das causas às análises
  que as verificaram.
- **LSS-RF-39** FMEA com RPN e ação prioritária por tabela configurável, ações para o kanban e
  reavaliação.
- **LSS-RF-40** Poka-yoke, 5S, SMED e trabalho padronizado com balanceamento contra o takt.
- **LSS-RF-41** Cartas I-MR, X̄-R, X̄-S, p, np, c, u, EWMA e CUSUM, fases I e II, regras de
  Western Electric e Nelson selecionáveis.
- **LSS-RF-42** Plano de controle e plano de resposta, com sinal gerando cartão no kanban.
- **LSS-RF-43** Análise reprodutível: receita (dados com hash, função, parâmetros, versão) e
  recálculo determinístico.
- **LSS-RF-44** Ligação a um modelo `.ron` com hash e aviso quando o modelo muda.
- **LSS-RF-45** Cenários e DOE rodados no simulador com números aleatórios comuns e comparação
  estatística contra a linha de base.
- **LSS-RF-46** Pacote de evidência `estudo.json` reprodutível, citado no relatório de tollgate.
- **LSS-RF-47** Gargalo, gargalo móvel e caminho crítico observado do modelo na fase Analisar.
- **LSS-RF-48** Leitura de KPIs do painel e de gravações do gêmeo para as cartas de fase II e o
  realizado dos ganhos.
- **LSS-RF-49** Abrir o modelo no UTSim a partir do projeto e receber estudos enviados pelo
  UTSim.
- **LSS-RF-50** Trabalho em equipe por pasta compartilhada com diário de operações, bloqueio
  otimista, lista de conflitos, presença e histórico.
- **LSS-RF-51** Exportação em PDF (pela impressão do navegador) do A3, tollgate, termo, ata,
  FMEA e plano de controle; `.xlsx`, CSV, SVG e PNG.
- **LSS-RF-52** Exportação e importação do projeto em arquivo único `.lssz`.
- **LSS-RF-53** Assinatura opcional das aprovações com chave por pessoa e conferência da cadeia
  de hash.

### Não funcionais

- **LSS-RNF-01** Toda função estatística tem teste contra valor certificado ou exemplo
  publicado antes de aparecer na interface.
- **LSS-RNF-02** As contas estatísticas ficam nas crates comuns, sem duplicata entre simulador,
  painel e app LSS; a CI confere que crates comuns não dependem de `utsim-lss-*`.
- **LSS-RNF-03** Funciona sem internet, sem servidor e sem o UTSim instalado (integração
  desabilitada com explicação).
- **LSS-RNF-04** Nenhuma gravação sobrescreve arquivo existente na pasta sincronizada.
- **LSS-RNF-05** A reconstrução do estado é idêntica em qualquer máquina, para qualquer ordem
  de chegada das operações.
- **LSS-RNF-06** Projeto com 5.000 operações abre em menos de 2 s com o cache local e em menos
  de 10 s sem ele.
- **LSS-RNF-07** Servidor escuta só em `127.0.0.1` por padrão; rede local apenas com opção e
  token.
- **LSS-RNF-08** Formato do projeto versionado com migrações; versão mais nova que a suportada
  dá erro claro.
- **LSS-RNF-09** Todos os textos da interface em catálogo; pt-BR completo na primeira versão.
- **LSS-RNF-10** Origem do dado (Real, Gêmeo, Simulado) visível em todo gráfico, índice e
  relatório.

---

## 7. Decisões do dono

**Decididas em 2026-11-03.** O dono escolheu um aplicativo de verdade, sem navegador, e aceitou
as demais propostas, com uma mudança no FMEA:

- **N1** nome provisório "UTSim Seis Sigma" (pode mudar depois; muda só o rótulo).
- **I1, I2 e I3 substituídas:** janela própria em Rust com `eframe`/`egui`, a mesma versão do
  launcher (`=0.36.2`) e do visualizador, para o `Cargo.lock` manter um só egui. Nada de
  servidor HTTP nem navegador. Gráficos com `egui_plot` na tela; impressão (A3, tollgate,
  termo, ata, FMEA, plano de controle) por PDF gerado no próprio programa. Formulários,
  tabelas editáveis, kanban, Gantt e editores de árvore são componentes egui próprios. O que a
  seção 4.2 diz sobre a página web fica como histórico da análise.
- **R1** mesmo workspace, versão e tag próprias. **S1** pasta de projeto com diário de
  operações em JSON. **C1** crate `utsim-doe`. **C2** `utsim-quality` como casa única de CEP,
  capacidade e MSA. **U1** belts de uma empresa piloto, com uso em aula em paralelo. **M1** MVP
  L0 a L5 com o MSA (P1) dentro. **G1** ganhos com as definições padrão e 12 meses. **G2** nível
  sigma com deslocamento de 1,5σ ligado e explícito no relatório.
- **T1 mudada:** o FMEA vem com a tabela de ação prioritária (AP) já no app, pronta para usar.
  A tabela embutida é própria do UTSim, escrita a partir da lógica pública do método (a
  severidade pesa primeiro, depois a ocorrência, depois a detecção) e cobrindo as 1000
  combinações; não é cópia da tabela do manual, que é protegida. A empresa que tem direito de
  uso da tabela oficial pode cadastrá-la (importação CSV) e ela passa a valer no projeto.

Registro original das perguntas:


1. **N1 Nome do app.** Provisório desde 2026-11-03: "UTSim Seis Sigma" (o dono deixou a escolha
   e pode mudar). As crates ficam com o prefixo `utsim-lss-*`, que não depende do nome exibido;
   trocar o nome muda só o rótulo no launcher e nas telas.
2. **I1 Interface.** Proposta: web servida localmente pelo próprio binário, no padrão do painel.
   Alternativas: egui/eframe ou Godot (seção 4.2).
3. **I2 Janela própria.** Proposta: abrir no navegador padrão na primeira versão; janela própria
   por visão web nativa depois, se os usuários pedirem.
4. **I3 Componentes de interface.** Proposta: componentes próprios em JavaScript puro, sem build.
   Alternativa: biblioteca pequena de código aberto embutida.
5. **R1 Repositório.** Proposta: mesmo workspace, versão e tag próprias por produto.
   Alternativa: repositório separado dependendo das crates por `git`.
6. **S1 Armazenamento.** Proposta: pasta de projeto com diário de operações em JSON e
   instantâneos; SQLite só como cache local. Alternativa: SQLite por projeto (não recomendada em
   pasta sincronizada).
7. **C1 Lugar do DOE.** Proposta: crate `utsim-doe`. Alternativa: módulo `doe` de `utsim-stats`.
8. **C2 Crate de qualidade.** Confirmar `utsim-quality` (CEP, capacidade, MSA) como a casa
   única de E4 e E6 do plano do gêmeo.
9. **U1 Público inicial.** Opções: (a) uso interno do autor e alunos (ensino de Seis Sigma com
   simulação); (b) green e black belts de uma empresa piloto; (c) gestores da carteira. A
   proposta é (b) com (a) em paralelo; o público define se o MVP prioriza a carteira (gestores) ou
   as ferramentas (belts).
10. **M1 Conteúdo do MVP.** Proposta da seção 5.1 (L0 a L5, 167–230 h). Pontos a confirmar:
    entrar o MSA (P1) no MVP, por ser entregável obrigatório do Medir, somando 18–25 h; deixar
    fora DOE, VSM e gêmeo.
11. **G1 Ganhos.** Confirmar as definições padrão de hard e soft savings e o período de
    acompanhamento de 12 meses; a empresa pode mudar no modelo.
12. **G2 Deslocamento de 1,5σ.** Confirmar que o nível sigma é mostrado com o deslocamento de 1,5
    ligado por padrão e explícito no relatório.
13. **T1 Tabelas de manuais.** Confirmar a abordagem de não embutir tabelas de manuais protegidos
    (AP do FMEA) e oferecer tabela padrão própria com cadastro pela empresa.
