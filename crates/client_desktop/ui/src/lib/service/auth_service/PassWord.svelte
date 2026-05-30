<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import { FieldValidator } from "$lib/service/validate/FieldValidator.svelte";
    import type { AuthStep } from '$lib/models/AuthStep';
    import type {PasswordDataShort} from "$lib/models/PasswordDataShort"

    // Текстовые значения полей ввода (двухсторонняя привязка)
    let nickname = $state('');
    let innPhys = $state('');
    let innOrg = $state('');
    let kppOrg = $state('');
    let password = $state('');

    // СОЗДАЕМ ИЗОЛИРОВАННЫЕ ВАЛИДАТОРЫ ДЛЯ КАЖДОГО ПОЛЯ
    const innPhysValidator = new FieldValidator();
    const innOrgValidator = new FieldValidator();
    const kppOrgValidator = new FieldValidator();

    // Функция сброса ошибки WrongPassword при изменении данных в полях
    function clearWrongPasswordStatus() {
        if ('WrongPassword' in currAuthStep.step) {
            currAuthStep.step = { NeedPassword: {} };
        }
    }

    // Функция отправки всей формы в Rust
    async function handleAuthSubmit() {
        // Жесткая проверка: все обязательные поля должны быть заполнены
        if (!nickname || !innPhys || !innOrg || !kppOrg || !password) return;
        
        // Защита: если хотя бы один из личных валидаторов вернул false, отменяем отправку
        if (!innPhysValidator.isValid || !innOrgValidator.isValid || !kppOrgValidator.isValid) return;

        // Переводим глобальное состояние в Loading, чтобы Layout показал экран загрузки
        currAuthStep.step = { Loading: {} }; 

        const sendData: PasswordDataShort = {
            nick: nickname,
            password: password,
            pers_inn: innPhys,
            comp_inn: innOrg,
            kpp: kppOrg
        };

        try {
            const nextStep = await invoke<AuthStep>('cmd_session_by_password', {
                data: sendData
            });

            if (!('WrongPassword' in nextStep)) {
                innPhysValidator.reset();
                innOrgValidator.reset();
                kppOrgValidator.reset();
            }

            currAuthStep.step = nextStep;

            
        } catch (err) {
            currAuthStep.step = { TryLater: { status: "SystemErr" } };
            console.error("Критическая ошибка cmd_auth_with_password:", err);
        }
    }
</script>

<div class="auth-card">
    <h2>Вход по учетным данным</h2>

    {#if 'WrongPassword' in currAuthStep.step}
        <div class="global-error">
            ⚠️ Неверный пароль или учетные данные. Повторите попытку.
        </div>
    {/if}

    <!-- Поле: Никнейм -->
    <div class="form-group">
        <label for="nickname">Никнейм</label>
        <input 
            id="nickname" 
            type="text" 
            bind:value={nickname} 
            oninput={clearWrongPasswordStatus}
            placeholder="Введите ваш никнейм"
            class="input-field" 
        />
    </div>

    <!-- Поле: ИНН Физического лица -->
    <div class="form-group">
        <label for="innPhys">ИНН физического лица</label>
        <input 
            id="innPhys" 
            type="text" 
            bind:value={innPhys} 
            oninput={() => {
                clearWrongPasswordStatus();
                innPhysValidator.validate({ Inn: innPhys }); // Вызов личного валидатора
            }} 
            placeholder="12 цифр ИНН ФЛ"
            class="input-field"
            class:invalid={!innPhysValidator.isValid}
        />
    </div>

    <!-- Поле: ИНН Организации -->
    <div class="form-group">
        <label for="innOrg">ИНН организации</label>
        <input 
            id="innOrg" 
            type="text" 
            bind:value={innOrg} 
            oninput={() => {
                clearWrongPasswordStatus();
                innOrgValidator.validate({ Inn: innOrg }); // Вызов личного валидатора
            }} 
            placeholder="10 цифр ИНН ЮЛ"
            class="input-field"
            class:invalid={!innOrgValidator.isValid}
        />
    </div>

    <!-- Поле: КПП Организации -->
    <div class="form-group">
        <label for="kppOrg">КПП организации</label>
        <input 
            id="kppOrg" 
            type="text" 
            bind:value={kppOrg} 
            oninput={() => {
                clearWrongPasswordStatus();
                kppOrgValidator.validate({ Kpp: kppOrg }); // Вызов личного валидатора
            }} 
            placeholder="9 знаков КПП"
            class="input-field"
            class:invalid={!kppOrgValidator.isValid}
        />
    </div>

    <!-- Поле: Пароль -->
    <div class="form-group">
        <label for="password">Пароль</label>
        <input 
            id="password" 
            type="password" 
            bind:value={password} 
            oninput={clearWrongPasswordStatus}
            placeholder="Введите пароль"
            class="input-field" 
        />
    </div>

    <!-- Кнопка автоматически блокируется, если поля пустые или хоть один валидатор вернул false -->
    <button 
        type="button" 
        onclick={handleAuthSubmit}
        disabled={
            !nickname || !innPhys || !innOrg || !kppOrg || !password || 
            !innPhysValidator.isValid || 
            !innOrgValidator.isValid || 
            !kppOrgValidator.isValid
        }
        class="submit-btn"
    >
        Войти
    </button>
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

    /* Подсвечивает инпут красным, если isValid внутри инстанса равен false */
    .input-field.invalid {
        border-color: #ff4d4d;
        background-color: #fff5f5;
    }

    .global-error {
        padding: 12px;
        background-color: #fff5f5;
        border: 1px solid #ff4d4d;
        color: #ff4d4d;
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
