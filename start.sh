#!/bin/bash

set -e

echo "⚙️  Welcome! Please select what you want to launch:"
echo "1) 🌐 Launch Backend API Only (Docker + Rust Server)"
echo "2) 📱 Launch Mobile Application (Docker + Backend + Tauri UI)"
echo "3) 🖥️ Launch Desktop PC Application (Docker + Backend + Tauri UI)"
echo "--------------------------------------------------"
read -r -p "Enter action number (1-3): " launch_choice

case $launch_choice in
    1)	
        echo "▶️ Launching Backend API..."
        # Гасим старый бэкенд перед стартом
        pkill -f back_api || true
        
        cd crates/back_api
        docker compose up -d
        sleep 2
        cargo run
        ;;
        
    2)
        echo "▶️ Launching Mobile Application Workspace..."
        # Принудительно очищаем старый бэкенд, бинарник мобилки и Vite-сервер
        pkill -f back_api || true
        pkill -f xpinat_lite_mobile || true
        pkill -f "vite" || true
        
        # Запуск бэкенда в фоне
        cd crates/back_api
        docker compose up -d
        sleep 2
        cargo run > backend.log 2>&1 &
        
        # Переход к мобильному приложению
        cd ../lite_mobile
        cargo tauri dev
        ;;
        
    3)
        echo "▶️ Launching Desktop PC Application Workspace..."
        # Принудительно очищаем старый бэкенд, бинарник ПК и Vite-сервер
        pkill -f back_api || true
        pkill -f xpinat_lite_pc || true
        pkill -f "vite" || true
        
        # Запуск бэкенда в фоне
        cd crates/back_api
        docker compose up -d
        sleep 2
        cargo run > backend.log 2>&1 &
        
        # Переход к ПК приложению
        cd ../lite_pc
        cargo tauri dev
        ;;
        
    *)
        echo "❌ Invalid selection. Exiting."
        exit 1
        ;;
esac
