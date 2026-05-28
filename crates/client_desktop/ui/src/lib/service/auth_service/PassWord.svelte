<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

    // Входные параметры (props)
    let { 
        onSuccess = () => {},
        onStepChange = () => {} // Пустая функция без аргументов — компилятор будет доволен!
    } = $props<{
        onSuccess?: () => void;
        onStepChange?: (step: unknown) => void;
    }>();

    // Состояние процесса и ошибок
    let isProcessing = $state(false);
    let errorMessage = $state('');

    // Поле никнейма для локального keyring на клиенте
    let nik = $state('');

    // Данные формы в точном соответствии со структурой PasswordData в Rust
    let passwordData = $state({
        password: '',
        pers_inn: '',
        comp_inn: '',
        kpp: ''
    });

    // Функция отправки формы
    async function handlePasswordAuth(e: SubmitEvent) {
        e.preventDefault();
        isProcessing = true;
        errorMessage = '';

        try {
            // Вызываем команду Tauri, передавая структуру данных и ник отдельно
            // Метод возвращает enum AuthStep
            const authStep = await invoke<unknown>('cmd_restore_by_password', { 
                data: passwordData,
                nik: nik
            });

            console.log('Ответ авторизации:', authStep);

            // Проверяем, какой шаг вернул бэкенд
            if (authStep === 'SuccessShort') {
                onSuccess();
            } else if (authStep && typeof authStep === 'object' && 'SuccessFull' in authStep) {
                onSuccess();
            } else {
                // Во всех остальных случаях пробрасываем шаг наверх для смены экрана
                onStepChange(authStep);
            }
        } catch (e) {
            errorMessage = 'Ошибка входа: ' + e;
        } finally {
            isProcessing = false;
        }
    }
</script>

<div>
    <h2>Авторизация по паролю</h2>

    <form onsubmit={handlePasswordAuth}>
        <div>
            <label for="nik-input">Никнейм (для локального профиля):</label>
            <input 
                id="nik-input"
                type="text" 
                bind:value={nik} 
                placeholder="Ваш никнейм" 
                disabled={isProcessing}
                required 
            />
        </div>

        <div>
            <label for="password-input">Пароль:</label>
            <input 
                id="password-input"
                type="password" 
                bind:value={passwordData.password} 
                placeholder="Введите пароль" 
                disabled={isProcessing}
                required 
            />
        </div>

        <div>
            <label for="pers-inn-input">ИНН физического лица:</label>
            <input 
                id="pers-inn-input"
                type="text" 
                bind:value={passwordData.pers_inn} 
                placeholder="12 цифр ИНН" 
                disabled={isProcessing}
                required 
            />
        </div>

        <div>
            <label for="comp-inn-input">ИНН Организации:</label>
            <input 
                id="comp-inn-input"
                type="text" 
                bind:value={passwordData.comp_inn} 
                placeholder="10 цифр ИНН" 
                disabled={isProcessing}
                required 
            />
        </div>

        <div>
            <label for="kpp-input">КПП:</label>
            <input 
                id="kpp-input"
                type="text" 
                bind:value={passwordData.kpp} 
                placeholder="9 цифр КПП" 
                disabled={isProcessing}
                required 
            />
        </div>

        <br />
        <button type="submit" disabled={isProcessing}>
            {isProcessing ? 'Проверка данных...' : 'Войти в систему'}
        </button>
    </form>

    {#if errorMessage}
        <p style="color: red; font-weight: bold; margin-top: 15px;">
            {errorMessage}
        </p>
    {/if}
</div>
