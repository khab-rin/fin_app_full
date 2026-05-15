#!/bin/bash

# Запуск бэкенда в отдельном процессе
cargo run --bin back-api &

# Пауза 2 секунды, чтобы бэк успел занять порт
sleep 2

# Переход в папку и запуск таури без лишних флагов
cd crates/client-desktop/src-tauri && cargo tauri dev

# Убиваем бэк при выходе
kill %1