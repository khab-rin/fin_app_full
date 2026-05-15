# === ЭТАП 1: Сборка ===
FROM rust:1.75-slim-bookworm AS builder
WORKDIR /app

# Смена зеркала для ускорения
RUN sed -i 's/deb.debian.org/mirror.yandex.ru/g' /etc/apt/sources.list.d/debian.sources || \
    sed -i 's/deb.debian.org/mirror.yandex.ru/g' /etc/apt/sources.list && \
    apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Копируем Cargo файлы
COPY Cargo.toml Cargo.lock ./

# Чтобы Cargo не искал недостающие части воркспейса (client-desktop), 
# мы создаем минимально необходимые папки-заглушки с пустыми Cargo.toml
# ИЛИ (если это проще) копируем весь корень проекта БЕЗ лишних данных через .dockerignore
COPY crates/shared-lib/ ./crates/shared-lib/
COPY crates/back-api/ ./crates/back-api/

# ХАК ДЛЯ WORKSPACE: Создаем пустую заглушку для tauri-клиента, 
# чтобы Cargo не выдавал ошибку "No such file or directory"
RUN mkdir -p crates/client-desktop/src-tauri && \
    echo '[package]\nname = "client-desktop"\nversion = "0.1.0"\nedition = "2021"' > crates/client-desktop/src-tauri/Cargo.toml && \
    mkdir -p crates/client-desktop/src-tauri/src && \
    touch crates/client-desktop/src-tauri/src/lib.rs

# Сборка только нужного бинарника
RUN cargo build --release --bin back_api

# === ЭТАП 2: Финал ===
FROM debian:bookworm-slim
WORKDIR /app

RUN sed -i 's/deb.debian.org/mirror.yandex.ru/g' /etc/apt/sources.list.d/debian.sources || \
    sed -i 's/deb.debian.org/mirror.yandex.ru/g' /etc/apt/sources.list && \
    apt-get update --fix-missing && \
    apt-get install -y --no-install-recommends \
    curl libccid pcscd lsb-release && rm -rf /var/lib/apt/lists/*

# ВАЖНО: Убедитесь, что архив лежит в той же папке, что и Dockerfile
ADD linux-amd64_deb.tgz /tmp/

RUN cd /tmp/linux-amd64_deb && \
    ./install.sh && \
    dpkg -i cprocsp-pki-cades*.deb cprocsp-pki-plugin*.deb || true && \
    apt-get update && \
    apt-get install -y -f --no-install-recommends && \
    rm -rf /tmp/linux-amd64_deb /var/lib/apt/lists/*

RUN curl -k -L -o /tmp/guc.crt https://gu-st.ru/content/lending/russian_trusted_root_ca.cer && \
    /opt/cprocsp/bin/amd64/certmgr -inst -store root -file /tmp/guc.crt && \
    rm /tmp/guc.crt

COPY --from=builder /app/target/release/back_api /app/back_api
ENV PATH="/opt/cprocsp/bin/amd64:/opt/cprocsp/sbin/amd64:${PATH}"
EXPOSE 8080
CMD ["./back_api"]