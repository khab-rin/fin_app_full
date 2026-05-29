<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';

    // Входные параметры (props) в Svelte 5
    let { 
        phone = '', 
        external_id = '', 
        nick = '',
        onSuccess = () => {} 
    } = $props();

    // Состояние ввода и процесса
    let code = $state('');
    let isProcessing = $state(false);
    let errorMessage = $state('');

    // Таймер для повторного звонка (в секундах)
    let countdown = $state(60);
    let timerInterval: ReturnType<typeof setInterval> | undefined;

    function startTimer() {
        countdown = 60;
        clearInterval(timerInterval);
        timerInterval = setInterval(() => {
            if (countdown > 0) {
                countdown--;
            } else {
                clearInterval(timerInterval);
            }
        }, 1000);
    }

    onMount(() => {
        startTimer();
        return () => clearInterval(timerInterval);
    });

    // Отправка последних 4 цифр на бэкенд
    async function handleVerifyCode(e: SubmitEvent) {
        e.preventDefault();
        if (code.length < 4) {
            errorMessage = 'Введите последние 4 цифры входящего номера';
            return;
        }

        isProcessing = true;
        errorMessage = '';

        try {
            await invoke('auth_verify_call_code', { 
                nick, 
                externalId: external_id, 
                code 
            });
            console.log('Код успешно проверен');
            onSuccess(); // Переход на главный экран
        } catch (e) {
            errorMessage = 'Неверный код или ошибка проверки: ' + e;
        } finally {
            isProcessing = false;
        }
    }

    // Запрос повторного звонка
    async function handleRetryCall() {
        isProcessing = true;
        errorMessage = '';
        try {
            await invoke('auth_retry_tel_call', { nick });
            startTimer();
            code = '';
        } catch (e) {
            errorMessage = 'Не удалось заказать повторный звонок: ' + e;
        } finally {
            isProcessing = false;
        }
    }
</script>

<div>
    <h2>Подтверждение входа</h2>
    
    <div>
        <p>Мы совершаем бесплатный робот-звонок на ваш номер телефона:</p>
        <p><b>{phone}</b></p>
        <p>Вам отвечать на звонок не нужно.</p>
    </div>

    <form onsubmit={handleVerifyCode}>
        <label for="code-input">
            Введите <b>последние 4 цифры</b> номера, который вам сейчас звонит:
        </label>
        
        <input 
            id="code-input"
            type="text" 
            bind:value={code} 
            maxlength="4"
            placeholder="0000"
            disabled={isProcessing}
            autocomplete="off"
            pattern="[0-9]*"
            inputmode="numeric"
            required 
        />

        <button type="submit" disabled={isProcessing || code.length !== 4}>
            {isProcessing ? 'Проверка...' : 'Подтвердить'}
        </button>
    </form>

    <div>
        {#if countdown > 0}
            <p>Запросить повторный звонок можно через: <b>{countdown} сек.</b></p>
        {:else}
            <button 
                type="button" 
                onclick={handleRetryCall} 
                disabled={isProcessing}
            >
                Позвонить еще раз
            </button>
        {/if}
    </div>

    {#if errorMessage}
        <p style="color: red; font-weight: bold;">
            {errorMessage}
        </p>
    {/if}
</div>
