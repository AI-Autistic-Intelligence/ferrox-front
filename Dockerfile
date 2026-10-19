# Fase 1: Build dell'applicazione Wasm con Trunk
FROM rust:1.76-alpine AS builder

# Installa le dipendenze per compilare Rust Wasm su Alpine
RUN apk add --no-cache musl-dev gcc make pkgconfig openssl-dev binaryen

# Installa wasm32-unknown-unknown e Trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked

WORKDIR /app

# Copia l'intero workspace
COPY . .

# Compila l'app demo in modalità release
WORKDIR /app/examples/admin-dashboard
RUN trunk build --release

# Fase 2: Immagine Nginx minimale per servire i file statici
FROM nginx:alpine

# Copia i file compilati (HTML, JS, Wasm) da Trunk alla webroot di Nginx
COPY --from=builder /app/examples/admin-dashboard/dist /usr/share/nginx/html

# Aggiunge una configurazione Nginx custom per gestire il routing SPA (fallback su index.html)
RUN echo 'server { \
    listen 80; \
    server_name localhost; \
    location / { \
        root /usr/share/nginx/html; \
        index index.html index.htm; \
        try_files $uri $uri/ /index.html; \
    } \
    # Ottimizzazione cache per file Wasm \
    location ~ \.wasm$ { \
        add_header Cache-Control "public, max-age=31536000, immutable"; \
        types { application/wasm wasm; } \
    } \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
