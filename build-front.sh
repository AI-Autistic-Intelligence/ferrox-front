#!/bin/bash
set -e

echo "🚀 Inizio build di Ferrox Front per la Produzione..."

# Verifica che Trunk sia installato
if ! command -v trunk &> /dev/null
then
    echo "Trunk non è installato. Installazione in corso..."
    cargo install trunk
fi

# Aggiunta target wasm
rustup target add wasm32-unknown-unknown

cd examples/admin-dashboard

echo "📦 Compilazione Wasm con ottimizzazioni di rilascio..."
trunk build --release

echo "✅ Build completata! I file statici si trovano in examples/admin-dashboard/dist"
echo "🌐 Per servire in locale, esegui: cd dist && python3 -m http.server 8080"
echo "🐳 Oppure costruisci l'immagine Docker: docker build -f ../../Dockerfile.frontend -t ferrox-front-app ../../"
