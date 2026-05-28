<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

    // Входные параметры (props) для Svelte 5
    let { 
        onSuccess = () => {},
        onStepChange = () => {},
        nik = $bindable('') // Передаем введенный никнейм вверх в layout
    } = $props<{
        onSuccess?: () => void;
        onStepChange?: (step: unknown) => void;
        nik?: string;
    }>();

    // Статический список сохраненных аккаунтов для выпадающего списка
    let savedNicks = $state(['Admin', 'Manager', 'Analyst']);
    
    let isProcessing = $state(false);
    let errorMessage = $state('');

    // Функция проверки никнейма при входе
    async function handleLogin(e: SubmitEvent) {
        e.preventDefault();
        if (!nik) return;
        
        isProcessing = true;
        errorMessage = '';
        try {
            // Шлем запрос в Tauri-команду для проверки токена/пользователя
            const authStep = await invoke<unknown>('auth_login', { nickname: nik });
            console.log('Ответ бэкенда на ввод ника:', authStep);
            
            if (authStep === 'SuccessShort') {
                onSuccess(); // Токен в keyring валиден — сразу пускаем в приложение
            } else {
                onStepChange(authStep); // Переключаем на 'NeedPassword' или 'NeedRegistration'
            }
        } catch (e) {
            errorMessage = 'Ошибка проверки никнейма: ' + e;
        } finally {
            isProcessing = false;
        }
    }
</script>

<div>
    <h2>Вход в систему</h2>

    <form onsubmit={handleLogin}>
        <div>
            <label for="nick-select">Выберите существующий аккаунт:</label>
            <br />
            <!-- Выпадающий список, синхронизированный с переменной nik -->
            <select id="nick-select" bind:value={nik} disabled={isProcessing}>
                <option value="" disabled selected>-- Выберите из списка --</option>
                <!-- Добавляем (saved) в качестве уникального ключа -->
                {#each savedNicks as saved (saved)}
                    <option value={saved}>{saved}</option>
                {/each}
            </select>
        </div>

        <br />

        <div>
            <label for="nick-input">Или введите никнейм вручную:</label>
            <br />
            <input 
                id="nick-input"
                type="text" 
                bind:value={nik} 
                placeholder="Никнейм..." 
                disabled={isProcessing}
                required 
            />
        </div>

        <br />
        
        <button type="submit" disabled={isProcessing || !nik}>
            {isProcessing ? 'Проверка...' : 'Войти'}
        </button>
    </form>

    {#if errorMessage}
        <p style="color: red; font-weight: bold; margin-top: 15px;">
            {errorMessage}
        </p>
    {/if}
</div>
