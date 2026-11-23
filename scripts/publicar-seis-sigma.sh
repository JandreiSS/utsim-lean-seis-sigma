#!/usr/bin/env bash
# Publica o UTSim LSS pelo launcher, como produto próprio (`lean-seis-sigma`): monta a
# pasta do programa, cifra, assina e grava o pacote e o manifesto na pasta de publicação (a
# pasta do Google Drive). Não envia nada à internet: o Google Drive para computador sincroniza
# a pasta. O launcher já instalado passa a mostrar o produto; ele não precisa mudar.
#
# Uso: bash scripts/publicar-seis-sigma.sh <pasta_de_publicação> [canal] [versão]
#   canal   padrão: testes (ou estavel)
#   versão  padrão: a de crates/utsim-lss-app/Cargo.toml (versão própria, tag lss-v<versão>)
#
# Antes: cargo build --release -p utsim-lss-app -p utsim-publicar
# Notas da versão: a seção "## [lss-<versão>]" do CHANGELOG.md, se existir; senão, os itens
# da seção "## [Não lançado]" que citam o UTSim LSS (ou o nome antigo, UTSim Seis Sigma).

set -eu

cd "$(dirname "$0")/.." || exit 1

[ $# -ge 1 ] || { echo "uso: $0 <pasta_de_publicação> [canal] [versão]" >&2; exit 2; }
SAIDA=$1
CANAL=${2:-testes}
VERSAO=${3:-$(awk -F'"' '/^version = /{print $2; exit}' crates/utsim-lss-app/Cargo.toml)}
VERSAO=${VERSAO#lss-}
VERSAO=${VERSAO#v}

PUBLICAR=target/release/utsim-publicar
[ -x "$PUBLICAR" ] || PUBLICAR=target/release/utsim-publicar.exe
[ -x "$PUBLICAR" ] || { echo "erro: rode antes cargo build --release -p utsim-publicar" >&2; exit 1; }

case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*) PLATAFORMA=windows-x64; NOME=utsim-seis-sigma.exe ;;
    Linux) PLATAFORMA=linux-x64; NOME=utsim-seis-sigma ;;
    *) echo "erro: plataforma não suportada" >&2; exit 1 ;;
esac
BIN=target/release/$NOME
[ -f "$BIN" ] || { echo "erro: $BIN não encontrado; rode antes cargo build --release -p utsim-lss-app" >&2; exit 1; }

PASTA=$(mktemp -d)
trap 'rm -rf "$PASTA"' EXIT
mkdir -p "$PASTA/seis-sigma/bin" "$PASTA/seis-sigma/manual"
cp "$BIN" "$PASTA/seis-sigma/bin/"
cp docs/manual/src/seis-sigma.md "$PASTA/seis-sigma/manual/"

if grep -q "^## \[lss-$VERSAO\]" CHANGELOG.md; then
    bash scripts/notas-versao.sh "lss-$VERSAO" > "$PASTA/notas.md"
else
    awk '/^## \[/ { n++ } n == 1 && /^- / && /UTSim (LSS|Seis Sigma)/ { print }' CHANGELOG.md > "$PASTA/notas.md"
fi
[ -s "$PASTA/notas.md" ] || echo "(sem notas)" > "$PASTA/notas.md"

"$PUBLICAR" publicar \
    --saida "$SAIDA" \
    --produto lean-seis-sigma \
    --nome "UTSim LSS" \
    --descricao "Gerenciador de projetos Lean Seis Sigma da família UTSim" \
    --subtitulo "Projetos Lean Seis Sigma" \
    --autor "UTSim" \
    --cor "#4FD06A" \
    --icone assets/icone/familia/lean-seis-sigma_256.png \
    --versao "$VERSAO" \
    --canal "$CANAL" \
    --plataforma "$PLATAFORMA" \
    --pasta "$PASTA/seis-sigma" \
    --executavel "bin/$NOME" \
    --notas "$PASTA/notas.md"
