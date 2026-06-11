<script lang='ts'>
    import {currAuthStep} from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import {invoke} from "@tauri-apps/api/core";

    import type {AuthStep} from "$lib/models/AuthStep";
	import { onMount } from "svelte";
    
    let isPolling = $state(false);
    
    const externalId = "CallIn" in currAuthStep.step ? currAuthStep.step.CallIn.external_id : "";
    const phone = "CallIn" in currAuthStep.step ? currAuthStep.step.CallIn.phone : "";

    let err_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};

    let data = {
        externalId: externalId,
        nick: currAuthStep.data.nick.value
    };

    async function poll_back_api() {
        if (!externalId) return;
        try {
            let next_step = await invoke<AuthStep>("cmd_session_by_tel_call", data);
            if ("CallInWaiting" in next_step) {
                if (currAuthStep.step && "CallIn" in currAuthStep.step) {
                    currAuthStep.step.CallIn.text = "Звонок по указанному номеру не был осуществлен, позвоните по этому номеру"; 
                }
            } else {
                currAuthStep.add(next_step);
            }
        } catch (err) {
            console.error("Error:", err);
            currAuthStep.add(err_step);
        }
    }

    onMount(() => {
        if (!externalId) {
            currAuthStep.add(err_step);
            return;
        }

        isPolling = true;


        const interval = setInterval(() => {
            if (isPolling) {
                poll_back_api();
            }
        }, 4000);

        return () => {
            clearInterval(interval);
        };
    });

</script>


<div class="auth-card">
    <p class="status-text">
        {currAuthStep.currentText}
    </p>

    {#if phone}
        <div class="phone-display">
            <span class="label">Номер для звонка:</span>
            <span class="phone-number">{phone}</span>
        </div>
    {/if}

    <div class="actions">
        <a 
            href="tel:{phone}"
            class="call-button" 
            class:disabled={isPolling}
            aria-disabled={isPolling}
            onclick={(e) => {
                if (isPolling) {
                    e.preventDefault();
                }
            }}
        >
            {#if isPolling}
                <span class="spinner"></span>
                Проверяем звонок...
            {:else}
                Позвонить
            {/if}
        </a>
    </div>
</div>

<style>
    .auth-card {
        max-width: 450px;
        margin: 2rem auto;
        padding: 1.5rem;
        background-color: #ffffff;
        border-radius: 12px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
        font-family: system-ui, -apple-system, sans-serif;
    }

    .status-text {
        font-size: 1.05rem;
        line-height: 1.5;
        color: #2c3e50;
        margin-bottom: 1.5rem;
        text-align: center;
    }

    .phone-display {
        display: flex;
        flex-direction: column;
        align-items: center;
        background-color: #f8f9fa;
        padding: 1rem;
        border-radius: 8px;
        margin-bottom: 2rem;
        border: 1px solid #e9ecef;
    }

    .phone-display .label {
        font-size: 0.85rem;
        color: #6c757d;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        margin-bottom: 0.25rem;
    }

    .phone-display .phone-number {
        font-size: 1.6rem;
        font-weight: 700;
        color: #0d6efd;
        letter-spacing: 1px;
    }

    .actions {
        display: flex;
        justify-content: center;
    }

    .call-button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        width: 100%;
        padding: 0.75rem 1.5rem;
        font-size: 1.1rem;
        font-weight: 600;
        color: #ffffff;
        background-color: #198754; /* Зеленая кнопка для звонка */
        border-radius: 8px;
        text-decoration: none;
        text-align: center;
        transition: background-color 0.2s, transform 0.1s;
        cursor: pointer;
        border: none;
    }

    .call-button:hover:not(.disabled) {
        background-color: #157347;
    }

    .call-button:active:not(.disabled) {
        transform: scale(0.98);
    }

    /* Стили для неактивного состояния (когда идет поллинг) */
    .call-button.disabled {
        background-color: #6c757d;
        color: #e9ecef;
        cursor: not-allowed;
        opacity: 0.8;
    }

    /* Простенький спиннер анимации ожидания */
    .spinner {
        width: 18px;
        height: 18px;
        border: 2px solid #ffffff;
        border-bottom-color: transparent;
        border-radius: 50%;
        display: inline-block;
        animation: rotation 1s linear infinite;
    }

    @keyframes rotation {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
</style>