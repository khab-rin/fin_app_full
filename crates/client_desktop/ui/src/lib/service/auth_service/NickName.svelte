<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import type { AuthStep } from "$lib/models/AuthStep";

    let isLoading = $state(false);
    let isPolling = $state(false);
    let pollTimer: ReturnType<typeof setTimeout> | null = null;
    let localNotice = $state("");
    let noticeType = $state<"warning" | "error" | "">("");

    // Извлекаем данные шага CallIn
    const callInData = $derived(() => {
        if (currAuthStep.step && "CallIn" in currAuthStep.step) {
            return currAuthStep.step.CallIn;
        }
        return null;
    });

    // Единая функция проверки статуса (и для полинга, и для кнопки)
    async function verifyStatus(isManualClick = false) {
        const data = callInData();
        if (!data || isLoading) return;

        // Если это ручной клик, сбрасываем текущий фоновый таймер, чтобы запросы не пересекались
        if (pollTimer) {
            clearTimeout(pollTimer);
            pollTimer = null;
        }

        try {
            if (isManualClick) {
                isLoading = true; // Спиннер на кнопке включаем только при ручном нажатии
                localNotice = "";
                noticeType = "";
            }

            // Запрос в Tauri
            const nextStep = await invoke<AuthStep>("check_sms_call_status", {
                externalId: data.external_id
            });

            // Если фаза сменилась (Успех / Ошибка) — останавливаем всё и двигаем менеджер шагов
            if (!("CallIn" in nextStep)) {
                isPolling = false;
                currAuthStep.add(nextStep);
                return;
            }

            // Если вернулся тот же CallIn, но обновился внутренний стейт/текст
            if (JSON.stringify(nextStep) !== JSON.stringify(currAuthStep.step)) {
                currAuthStep.step = nextStep;
            }

            // Если пользователь нажал кнопку руками, а звонка еще нет — покажем подсказку
            if (isManualClick) {
                localNotice = "Система пока не зафиксировала звонок. Пожалуйста, убедитесь, что набрали номер, или подождите немного.";
                noticeType = "warning";
            }

        } catch (err) {
            console.error("Ошибка верификации звонка:", err);
            if (isManualClick) {
                localNotice = "Не удалось связаться с сервером. Попробуйте нажать еще раз.";
                noticeType = "error";
            }
        } finally {
            if (isManualClick) {
                isLoading = false;
            }
            
            // Расписание на следующий фоновый тик (если полинг активен)
            if (isPolling) {
                pollTimer = setTimeout(() => verifyStatus(false), 4000);
            }
        }
    }

    onMount(() => {
        if (callInData()) {
            isPolling = true;
            // Первый фоновый запрос пойдет через 4 секунды после открытия экрана
            pollTimer = setTimeout(() => verifyStatus(false), 4000);
        } else {
            localNotice = "Данные для авторизации по звонку не найдены.";
            noticeType = "error";
        }
    });

    onDestroy(() => {
        isPolling = false;
        if (pollTimer) clearTimeout(pollTimer);
    });

    function handleBack() {
        currAuthStep.back();
    }
</script>

<div class="call-in-container">
    {#if callInData()}
        <div class="auth-card">
            <h2 class="title">Проверка входящего вызова</h2>
            
            <p class="instruction-text">
                Для авторизации на новом устройстве позвоните по номеру телефона ниже, 
                сбросьте вызов после первого гудка и нажмите на кнопку <strong>"Авторизоваться"</strong> 
                (или просто подождите, система проверяет статус автоматически).
            </p>

            <div class="phone-display">
                <span class="phone-label">Номер для звонка:</span>
                <span class="phone-number">{callInData()?.phone}</span>
            </div>

            {#if !localNotice && isPolling}
                <div class="polling-indicator">
                    <span class="dot animate-ping"></span>
                    <span class="text">Автоматическая проверка статуса каждые 4 сек...</span>
                </div>
            {/if}

            {#if localNotice}
                <div class="status-message {noticeType}">
                    {localNotice}
                </div>
            {/if}

            <div class="actions-vertical">
                <button 
                    class="btn-primary" 
                    disabled={isLoading} 
                    onclick={() => verifyStatus(true)}
                >
                    {#if isLoading}
                        <span class="spinner-small"></span> Проверяем...
                    {:else}
                        Авторизоваться
                    {/if}
                </button>

                <button class="btn-link" disabled={isLoading} onclick={handleBack}>
                    ⬅ Назад (изменить данные)
                </button>
            </div>
        </div>
    {:else}
        <div class="loading-state">
            <span class="spinner">⏳</span>
            <p>Загрузка данных авторизации...</p>
        </div>
    {/if}
</div>

<style>
    /* Все предыдущие стили остаются в силе, добавляем пару новых для индикатора полинга */
    .call-in-container {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 2rem;
        min-height: 400px;
    }
    .auth-card {
        background: #1e1e2e;
        border: 1px solid #313244;
        border-radius: 12px;
        padding: 2.5rem;
        width: 100%;
        max-width: 450px;
        text-align: center;
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
    }
    .title {
        font-size: 1.4rem;
        font-weight: 700;
        color: #cdd6f4;
        margin-bottom: 1.2rem;
    }
    .instruction-text {
        font-size: 0.95rem;
        color: #a6adc8;
        line-height: 1.6;
        margin-bottom: 2rem;
    }
    .instruction-text strong {
        color: #b4befe;
    }
    .phone-display {
        background: #11111b;
        border-radius: 8px;
        padding: 1.25rem;
        margin-bottom: 1.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
        border: 1px dashed #45475a;
    }
    .phone-label {
        font-size: 0.75rem;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: #6c7086;
    }
    .phone-number {
        font-family: monospace;
        font-size: 1.6rem;
        font-weight: 700;
        color: #a6e3a1;
        letter-spacing: 0.02em;
    }
    
    /* Индикатор автоматического полинга */
    .polling-indicator {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        margin-bottom: 1.5rem;
    }
    .polling-indicator .dot {
        width: 8px;
        height: 8px;
        background-color: #a6e3a1;
        border-radius: 50%;
    }
    .polling-indicator .text {
        font-size: 0.8rem;
        color: #6c7086;
    }

    .status-message {
        font-size: 0.85rem;
        padding: 0.75rem;
        border-radius: 6px;
        margin-bottom: 1.5rem;
        line-height: 1.4;
    }
    .status-message.warning {
        background: rgba(249, 226, 175, 0.1);
        color: #f9e2af;
        border: 1px solid rgba(249, 226, 175, 0.2);
    }
    .status-message.error {
        background: rgba(243, 139, 168, 0.1);
        color: #f38ba8;
        border: 1px solid rgba(243, 139, 168, 0.2);
    }
    .actions-vertical {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        margin-top: 0.5rem;
    }
    .btn-primary {
        background: #b4befe;
        color: #11111b;
        font-weight: 600;
        padding: 0.75rem;
        border-radius: 6px;
        border: none;
        cursor: pointer;
        transition: background 0.2s;
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.5rem;
        font-size: 1rem;
        width: 100%;
    }
    .btn-primary:hover:not(:disabled) {
        background: #cba6f7;
    }
    .btn-primary:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
    .btn-link {
        background: transparent;
        border: none;
        color: #6c7086;
        cursor: pointer;
        font-size: 0.85rem;
    }
    .btn-link:hover:not(:disabled) {
        color: #a6adc8;
        text-decoration: underline;
    }
    .spinner-small {
        width: 16px;
        height: 16px;
        border: 2px solid rgba(17, 17, 27, 0.2);
        border-top-color: #11111b;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }
    @keyframes spin {
        to { transform: rotate(360deg); }
    }
    @keyframes animate-ping {
        0% { transform: scale(1); opacity: 1; }
        70%, 100% { transform: scale(2.5); opacity: 0; }
    }
</style>