<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import type { AuthStep } from '$lib/models/AuthStep';
    import type {PasswordDataShort} from "$lib/models/PasswordDataShort"

    let isPushed = $state(false);

    $effect(() => {
        currAuthStep.data.nick.validate(); 
    });

    $effect(() => {
        currAuthStep.data.persInn.validate(); 
    });

    async function handleAuthSubmit() {
        if (isPushed) return;
       
        if (
            !currAuthStep.data.nick.isValid || 
            !currAuthStep.data.persInn.isValid || 
            !currAuthStep.data.compInn.isValid ||
            !currAuthStep.data.kpp.isValid ||
            !currAuthStep.data.password.isValid
        ) return;

        const sendData: PasswordDataShort = {
            nick: currAuthStep.data.nick.value,
            password: currAuthStep.data.password.value,
            pers_inn: currAuthStep.data.persInn.value,
            comp_inn: currAuthStep.data.compInn.value,
            kpp: currAuthStep.data.kpp.value
        };

        isPushed = true;

        let next_step: AuthStep = { TryLater: { text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение" } };
    
        try {
            next_step = await invoke<AuthStep>('cmd_session_by_password', {
                data: sendData
            });

        } catch (err) {
            console.error("Критическая ошибка cmd_auth_with_password:", err);

        } finally {
            isPushed = false;
            currAuthStep.add(next_step);
        }
    }

    function handleGoToRegister() {
        let next_step: AuthStep = {NeedRegistration: {text: ""}};
        currAuthStep.add(next_step);
    }

    function handleGoToNickName() {
        let next_step: AuthStep = {NickName: {text: ""}};
        currAuthStep.add(next_step);
    }

    function handleGoBack() {
        currAuthStep.back(); 
    }

    function handleGoNext() {
        currAuthStep.next(); 
    }

</script>

<div class="auth-card">
    <p>
        {currAuthStep.currentText}
    </p>

    <!-- Поле: Никнейм -->
    <div class="form-group">
        <label for="nick">Имя пользователя</label>
        <input 
            id="nick" 
            type="text" 
            bind:value={currAuthStep.data.nick.value} 
            disabled={isPushed}
            placeholder="Введите ваш никнейм"
            class="input-field" 
            class:input-error={!currAuthStep.data.nick.isValid}/>
        
        {#if !currAuthStep.data.nick.isValid}
            <span class="error-message">Некорректный никнейм</span>
        {/if}
    </div>

    <div class="form-group">
        <label for="persInn">ИНН физического лица</label>
        <input 
            id="persInn" 
            type="text" 
            bind:value={currAuthStep.data.persInn.value} 
            disabled={isPushed}
            placeholder="12 цифр ИНН ФЛ"
            class="input-field"
            class:input-error={!currAuthStep.data.persInn.isValid}
        />
        {#if !currAuthStep.data.persInn.isValid}
            <span class="error-message">Некорректный инн физического лица</span>
        {/if}
    </div>

    <!-- Поле: ИНН Организации -->
    <div class="form-group">
        <label for="compInn">ИНН организации</label>
        <input 
            id="innOrg" 
            type="text" 
            bind:value={currAuthStep.data.compInn.value}
            disabled={isPushed}
            placeholder="10 цифр ИНН ЮЛ"
            class="input-field"
            class:input-error={!currAuthStep.data.compInn.isValid}/>
        {#if !currAuthStep.data.compInn.isValid}
            <span class="error-message">Некорректный инн юридического лица</span>
        {/if}
    </div>

    <!-- Поле: КПП Организации -->
    <div class="form-group">
        <label for="kpp">КПП организации</label>
        <input 
            id="kppOrg" 
            type="text" 
            bind:value={currAuthStep.data.kpp.value}
            disabled={isPushed} 
            placeholder="9 знаков КПП"
            class="input-field"
            class:input-error={!currAuthStep.data.kpp.isValid}/>
        {#if !currAuthStep.data.kpp.isValid}
            <span class="error-message">Некорректный кпп</span>
        {/if}
    </div>

    <!-- Поле: Пароль -->
    <div class="form-group">
        <label for="password">Пароль</label>
        <input 
            id="password" 
            type="password" 
            bind:value={currAuthStep.data.password.value}
            disabled={isPushed} 
            placeholder="Введите пароль"
            class="input-field"
            class:input-error={!currAuthStep.data.password.isValid}/>
        {#if !currAuthStep.data.password.isValid}
            <span class="error-message">Пароль некоректен в рамках прилжоения</span>
        {/if}
    </div>

    <section class="grid-section">
        <div class="quick-links">
            
            <button 
                type="button" 
                onclick={handleAuthSubmit}
                disabled={
                    isPushed || 
                    !currAuthStep.data.nick.isValid || 
                    !currAuthStep.data.persInn.isValid || 
                    !currAuthStep.data.compInn.isValid || 
                    !currAuthStep.data.kpp.isValid || 
                    !currAuthStep.data.password.isValid}
                class="grid-item"
                id="auth-submit-btn"
            >
                <span class="btn-icon">
                    {#if isPushed}⏳{:else}🔑{/if}
                </span>
                <span class="btn-label">
                    {#if isPushed}Вход...{:else}Войти{/if}
                </span>
            </button>

            <button 
                type="button" 
                onclick={handleGoToRegister} 
                disabled={isPushed} 
                class="grid-item"
                id="auth-register-btn"
            >
                <span class="btn-icon">📝</span>
                <span class="btn-label">Регистрация</span>
            </button>

            <button 
                type="button" 
                onclick={handleGoToNickName} 
                disabled={isPushed} 
                class="grid-item"
                id="auth-select-user-btn"
            >
                <span class="btn-icon">👤</span>
                <span class="btn-label">Выбрать пользователя</span>
            </button>

            <button 
                type="button" 
                onclick={handleGoBack} 
                class="grid-item"
                id="auth-select-user-btn"
            >
                <span class="btn-icon">👤</span>
                <span class="btn-label">Назад</span>
            </button>

            <button 
                type="button" 
                onclick={handleGoNext} 
                class="grid-item"
                id="auth-select-user-btn"
            >
                <span class="btn-icon">👤</span>
                <span class="btn-label">Вперед</span>
            </button>

        </div>
    </section>
</div>

<style>
    .auth-card {
        display: flex;
        flex-direction: column;
        gap: 15px;
        max-width: 400px;
        margin: 40px auto;
        padding: 24px;
        background: #ffffff;
        border: 1px solid #e0e0e0;
        border-radius: 8px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    label {
        font-size: 14px;
        font-weight: 500;
        color: #333333;
    }

    .input-field {
        padding: 10px;
        font-size: 14px;
        border: 1px solid #cccccc;
        border-radius: 4px;
        outline: none;
        transition: border-color 0.2s, background-color 0.2s;
    }

    .input-field:focus {
        border-color: #007bff;
    }

    /* ИСПРАВЛЕНО: Изменено название класса с .invalid на .input-error */
    /* Сделали красный цвет чуть мягче для приятного UX */
    .input-field.input-error {
        border-color: #dc3545;
        background-color: #fdf2f2;
    }

    /* ДОБАВЛЕНО: Стили для текста ошибок под инпутами */
    .error-message {
        font-size: 12px;
        color: #dc3545;
        margin-top: -2px;
    }

    /* ДОБАВЛЕНО: Визуальное состояние инпутов при отправке в Tauri */
    .input-field:disabled {
        background-color: #f5f5f5;
        color: #888888;
        cursor: not-allowed;
    }

</style>