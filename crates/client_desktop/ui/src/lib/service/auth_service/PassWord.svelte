<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import { FieldValidator } from "$lib/service/validate/FieldValidator.svelte";
    import type { AuthStep } from '$lib/models/AuthStep';
    import type {PasswordDataShort} from "$lib/models/PasswordDataShort"

    let nick = $state('');
    let persInn = $state('');
    let compInn = $state('');
    let kpp = $state('');
    let password = $state('');

    let isPushed = $state(false);

    const nickNameValid = new FieldValidator();
    const persInnValid = new FieldValidator();
    const compInnValid = new FieldValidator();
    const kppValid = new FieldValidator();
    const passwordValid = new FieldValidator();

    $effect(() => {
        nickNameValid.validate({ String1_50: nick }); 
    });

    $effect(() => {
        persInnValid.validate({ PersInn: persInn });
    });

    $effect(() => {
        compInnValid.validate({ CompInn: compInn });
    });

    $effect(() => {
        kppValid.validate({ Kpp: kpp });
    });

    $effect(() => {
        passwordValid.validate({ Password: password });
    });



    async function handleAuthSubmit() {
        if (isPushed) return;
       
        if (
            !nickNameValid.isValid || 
            !persInnValid.isValid || 
            !compInnValid.isValid ||
            !kppValid.isValid ||
            !passwordValid.isValid
        ) return;

        const sendData: PasswordDataShort = {
            nick: nick,
            password: password,
            pers_inn: persInn,
            comp_inn: compInn,
            kpp: kpp
        };

        try {

            currAuthStep.step = await invoke<AuthStep>('cmd_session_by_password', {
                data: sendData
            });
            
        } catch (err) {
            currAuthStep.step = { TryLater: { text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение" } };
            console.error("Критическая ошибка cmd_auth_with_password:", err);
            isPushed = false;
        }
    }

    function handleGoToRegister() {
        currAuthStep.step = { NeedRegistration: {text: ""} }; 
    }

    function handleGoToNickName() {
        currAuthStep.step = { Loading: {text: ""} }; 
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
            bind:value={nick} 
            disabled={isPushed}
            placeholder="Введите ваш никнейм"
            class="input-field" 
            class:input-error={!nickNameValid.isValid} 
        />
        
        {#if !nickNameValid.isValid}
            <span class="error-message">Некорректный никнейм</span>
        {/if}
    </div>

    <!-- Поле: ИНН Физического лица -->
    <div class="form-group">
        <label for="persInn">ИНН физического лица</label>
        <input 
            id="persInn" 
            type="text" 
            bind:value={persInn} 
            disabled={isPushed}
            placeholder="12 цифр ИНН ФЛ"
            class="input-field"
            class:input-error={!persInnValid.isValid}
        />
        {#if !persInnValid.isValid}
            <span class="error-message">Некорректный инн физического лица</span>
        {/if}
    </div>

    <!-- Поле: ИНН Организации -->
    <div class="form-group">
        <label for="compInn">ИНН организации</label>
        <input 
            id="innOrg" 
            type="text" 
            bind:value={compInn}
            disabled={isPushed}
            placeholder="10 цифр ИНН ЮЛ"
            class="input-field"
            class:input-error={!compInnValid.isValid}
        />
        {#if !compInnValid.isValid}
            <span class="error-message">Некорректный инн юридического лица</span>
        {/if}
    </div>

    <!-- Поле: КПП Организации -->
    <div class="form-group">
        <label for="kpp">КПП организации</label>
        <input 
            id="kppOrg" 
            type="text" 
            bind:value={kpp}
            disabled={isPushed} 
            placeholder="9 знаков КПП"
            class="input-field"
            class:input-error={!kppValid.isValid}
        />
        {#if !kppValid.isValid}
            <span class="error-message">Некорректный кпп</span>
        {/if}
    </div>

    <!-- Поле: Пароль -->
    <div class="form-group">
        <label for="password">Пароль</label>
        <input 
            id="password" 
            type="password" 
            bind:value={password}
            disabled={isPushed} 
            placeholder="Введите пароль"
            class="input-field"
            class:input-error={!passwordValid.isValid}
        />
        {#if !passwordValid.isValid}
            <span class="error-message">Пароль некоректен в рамках прилжоения</span>
        {/if}
    </div>

    <section class="grid-section">
        <div class="quick-links">
            
            <button 
                type="button" 
                onclick={handleAuthSubmit}
                disabled={isPushed || !nickNameValid.isValid || !persInnValid.isValid || !compInnValid.isValid || !kppValid.isValid || !passwordValid.isValid}
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

    .global-error {
        padding: 12px;
        background-color: #fdf2f2;
        border: 1px solid #dc3545;
        color: #dc3545;
        border-radius: 4px;
        font-size: 14px;
    }

    .submit-btn {
        margin-top: 10px;
        padding: 12px;
        font-size: 16px;
        font-weight: 600;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .submit-btn:hover:not(:disabled) {
        background-color: #0056b3;
    }

    .submit-btn:disabled {
        background-color: #cccccc;
        color: #666666;
        cursor: not-allowed;
    }
</style>