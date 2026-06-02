<script lang='ts'>
	import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import { writeFile } from "@tauri-apps/plugin-fs";
    import { save, open as openFileDialog } from "@tauri-apps/plugin-dialog";
    import {FieldValidator} from "$lib/service/validate/FieldValidator.svelte";

    import type {AuthStep} from "$lib/models/AuthStep";
    import type {IngoingData} from "$lib/models/IngoingData";
    import type {SvelteRegistrationData} from "$lib/models/SvelteRegistrationData"
	


    let isPushedReg = $state(false);
    let isPushedDoc = $state(false);
    let isGeneratedDoc = $state(false);

    let passwordRepeat = $state("")

    let docPathSaved = $state("");
    let signPath = $state("");
    

    let nick = $state('');
    let surName = $state('');
    let firstName = $state('');
    let midName = $state('');
    let persInn = $state('');
    let snils = $state('');
    let compInn = $state('');
    let kpp = $state('');
    let password = $state('');
    let phone = $state('');
    let email = $state('');

    const nickValid = new FieldValidator();
    const surNameValid = new FieldValidator();
    const firstNameValid = new FieldValidator();
    const midNameValid = new FieldValidator();
    const persInnValid = new FieldValidator();
    const snilsValid = new FieldValidator();
    const compInnValid = new FieldValidator();
    const kppValid = new FieldValidator();
    const passwordValid = new FieldValidator();
    const phoneValid = new FieldValidator();
    const emailValid = new FieldValidator();

    $effect(() => {
        nickValid.validate({ String1_50: nick }); 
    });
    $effect(() => {
        surNameValid.validate({ SurName: surName }); 
    });
    $effect(() => {
        firstNameValid.validate({ FirstName: firstName }); 
    });
    $effect(() => {
        midNameValid.validate({ MidName: midName }); 
    });
    $effect(() => {
        persInnValid.validate({ PersInn: persInn }); 
    });
    $effect(() => {
        snilsValid.validate({ Snils: snils }); 
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
    $effect(() => {
        phoneValid.validate({ Phone: phone }); 
    });
    $effect(() => {
        emailValid.validate({ Email: email }); 
    });

    async function handleSelectSigFile() {
        try {
            // Вызываем нативный диалог Tauri для открытия файла
            const selected = await openFileDialog({
                multiple: false,  // Выбираем только один файл
                directory: false, // Нам нужен файл, а не папка
                title: "Выберите файл отсоединенной подписи заявления",
                filters: [{ name: 'Электронная подпись', extensions: ['sig', 'p7s'] }]
            });


            if (selected && typeof selected === 'string') {
                signPath = selected;
            }
        } catch (err) {
            console.error("Ошибка при выборе файла подписи:", err);
        }
    }

    async function handleIngoingDoc() {
        if (isPushedDoc) return;

        if (
            !surNameValid.isValid ||
            !firstNameValid.isValid ||
            !midNameValid.isValid ||
            !persInnValid.isValid ||
            !snilsValid.isValid ||
            !compInnValid.isValid ||
            !kppValid.isValid ||
            !phoneValid.isValid ||
            !emailValid.isValid
        ) return;

        const ingoingData: IngoingData = {
            sur_name: surName,
            first_name: firstName,
            mid_name: midName.trim() === "" ? null : midName,
            pers_inn: persInn,
            snils: snils,
            comp_inn: compInn,
            kpp: kpp,
            phone: phone,
            email: email
        };

        try {
            const fileBytesArray = await invoke<number[]>("cmd_make_ingoing_doc", {
                data: ingoingData
            });
            const fileBytes = new Uint8Array(fileBytesArray);

            const docPath = await save({
                title: "Сохранить заявление для подписания ЭЦП",
                defaultPath: "Заявление_на_регистрацию.doc",
                filters: [{ name: 'Документы Word', extensions: ['doc'] }]
                
            });
            if (docPath) {
                await writeFile(docPath, fileBytes);
                console.log("Документ успешно сохранен по пути:", docPath);

                docPathSaved = docPath;
                isGeneratedDoc = true;
    
            } else {
                isPushedDoc = false;
            }

        } catch (err) {
            console.error("MAKE INGOING FILE ERR: ", err);
            isPushedDoc = false;
            currAuthStep.step = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
        }
    };


    async function handleRegistrationSubmit() {
        if (isPushedReg) return;

        if (
            !nickValid.isValid ||
            !surNameValid.isValid ||
            !firstNameValid.isValid ||
            !midNameValid.isValid ||
            !persInnValid.isValid ||
            !snilsValid.isValid ||
            !compInnValid.isValid ||
            !kppValid.isValid ||
            !passwordValid.isValid ||
            !phoneValid.isValid ||
            !emailValid.isValid ||
            docPathSaved.length == 0 ||
            signPath.length == 0
        ) return;

        const regData: SvelteRegistrationData = {
            nick: nick,
            sur_name: surName,
            first_name: firstName,
            mid_name: midName,
            pers_inn: persInn,
            snils: snils,
            comp_inn: compInn,
            kpp: kpp,
            password: password,
            phone: phone,
            email: email,
            document_path: docPathSaved,  
            signature_path: signPath,
        }

        try {
            currAuthStep.step = await invoke<AuthStep>("cmd_register_user", {data: regData});
        } catch (err) {
            console.error("Registration FAILED, err = ", err);
            currAuthStep.step = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
        }
    }
</script>

<div class="auth-card">
    <p>
        {currAuthStep.currentText}
    </p>

    {#if !isGeneratedDoc}
        <div class="form-group">
            <label for="surName">Фамилия</label>
            <input 
                id="surName" 
                type="text" 
                bind:value={surName} 
                disabled={isPushedDoc}
                placeholder="Только русские буквы"
                class="input-field"
                class:input-error={!surNameValid.isValid}
            />
            {#if !surNameValid.isValid}
                <span class="error-message">Некорректная фамилия</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="firstName">Имя</label>
            <input 
                id="firstName" 
                type="text" 
                bind:value={firstName} 
                disabled={isPushedDoc}
                placeholder="Только русские буквы"
                class="input-field"
                class:input-error={!firstNameValid.isValid}
            />
            {#if !firstNameValid.isValid}
                <span class="error-message">Некорректное имя</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="midName">Отчество (при наличии)</label>
            <input 
                id="midName" 
                type="text" 
                bind:value={midName} 
                disabled={isPushedDoc}
                placeholder="Только русские буквы"
                class="input-field"
                class:input-error={!midNameValid.isValid}
            />
            {#if !midNameValid.isValid}
                <span class="error-message">Некорректное отчество</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="persInn">Личный ИНН</label>
            <input 
                id="persInn" 
                type="text" 
                bind:value={persInn} 
                disabled={isPushedDoc}
                placeholder="12 цифр"
                class="input-field"
                class:input-error={!persInnValid.isValid}
            />
            {#if !persInnValid.isValid}
                <span class="error-message">Некорректный ИНН (должно быть 12 цифр)</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="snils">СНИЛС</label>
            <input 
                id="snils" 
                type="text" 
                bind:value={snils} 
                disabled={isPushedDoc}
                placeholder="Формат: 000-000-000 00"
                class="input-field"
                class:input-error={!snilsValid.isValid}
            />
            {#if !snilsValid.isValid}
                <span class="error-message">Некорректный СНИЛС</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="compInn">ИНН Организации</label>
            <input 
                id="compInn" 
                type="text" 
                bind:value={compInn} 
                disabled={isPushedDoc}
                placeholder="10 цифр"
                class="input-field"
                class:input-error={!compInnValid.isValid}
            />
            {#if !compInnValid.isValid}
                <span class="error-message">Некорректный ИНН организации (должно быть 10 цифр)</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="kpp">КПП</label>
            <input 
                id="kpp" 
                type="text" 
                bind:value={kpp} 
                disabled={isPushedDoc}
                placeholder="9 цифр"
                class="input-field"
                class:input-error={!kppValid.isValid}
            />
            {#if !kppValid.isValid}
                <span class="error-message">Некорректный КПП (должно быть 9 цифр)</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="phone">Номер телефона</label>
            <input 
                id="phone" 
                type="tel" 
                bind:value={phone} 
                disabled={isPushedDoc}
                placeholder="+7 (900) 000-00-00"
                class="input-field"
                class:input-error={!phoneValid.isValid}
            />
            {#if !phoneValid.isValid}
                <span class="error-message">Некорректный номер телефона</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="email">Электронная почта</label>
            <input 
                id="email" 
                type="email" 
                bind:value={email} 
                disabled={isPushedDoc}
                placeholder="example@mail.ru"
                class="input-field"
                class:input-error={!emailValid.isValid}
            />
            {#if !emailValid.isValid}
                <span class="error-message">Некорректный email</span>
            {/if}
        </div>

        <button 
            type="button"
            onclick={handleIngoingDoc} 
            disabled={isPushedDoc}
            class="submit-btn"
        >
            {isPushedDoc ? 'Генерация файла...' : 'Сформировать заявление (.doc)'}
        </button>

    {:else}
    <div class="success-container">
        <p class="success-notice">🎉 Шаг 1 завершен: заявление успешно сформировано и сохранено!</p>
        
        <hr class="divider" />

        <div class="form-group">
            <label for="nick">Никнейм (Логин для входа)</label>
            <input 
                id="nick" 
                type="text" 
                bind:value={nick} 
                disabled={isPushedReg}
                placeholder="Придумайте уникальный логин"
                class="input-field"
                class:input-error={!nickValid.isValid}
            />
            {#if !nickValid.isValid}
                <span class="error-message">Некорректный никнейм (от 1 до 50 символов)</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="password">Придумайте пароль приложения</label>
            <input 
                id="password" 
                type="password" 
                bind:value={password} 
                disabled={isPushedReg}
                placeholder="Минимум 6 символов"
                class="input-field"
                class:input-error={!passwordValid.isValid}
            />
            {#if !passwordValid.isValid}
                <span class="error-message">Слишком короткий или простой пароль</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="passwordRepeat">Повторите пароль</label>
            <input 
                id="passwordRepeat" 
                type="password" 
                bind:value={passwordRepeat} 
                disabled={isPushedReg}
                placeholder="Введите пароль еще раз"
                class="input-field"
                class:input-error={password !== passwordRepeat && passwordRepeat !== ''}
            />
            {#if password !== passwordRepeat && passwordRepeat !== ''}
                <span class="error-message">Пароли не совпадают</span>
            {/if}
        </div>

        <hr class="divider" />

        <div class="form-group">
            <label>Сформированное заявление (.doc)</label>
            <div class="file-picker-wrapper">
                <input 
                    type="text" 
                    value={docPathSaved} 
                    disabled 
                    class="input-field file-path-input" 
                />
                <button 
                    type="button" 
                    class="secondary-btn" 
                    onclick={handleIngoingDoc}
                    disabled={isPushedReg}
                >
                    Пересохранить
                </button>
            </div>
        </div>

        <div class="form-group">
            <label for="sigPath">Файл электронной подписи (.doc.sig)</label>
            <div class="file-picker-wrapper">
                <input 
                    id="sigPath"
                    type="text" 
                    value={signPath || 'Файл подписи не выбран...'} 
                    disabled 
                    class="input-field file-path-input"
                    class:input-error={signPath === '' && isPushedReg}
                />
                <button 
                    type="button" 
                    class="primary-btn" 
                    onclick={handleSelectSigFile}
                    disabled={isPushedReg}
                >
                    Обзор...
                </button>
            </div>
            {#if signPath === '' && isPushedReg}
                <span class="error-message">Необходимо прикрепить файл подписи</span>
            {/if}
        </div>

        <button 
            type="button"
            onclick={handleRegistrationSubmit} 
            disabled={isPushedReg || password !== passwordRepeat || password === '' || !signPath}
            class="submit-btn register-btn"
        >
            {isPushedReg ? 'Регистрация...' : 'Завершить регистрацию'}
        </button>
    </div>
{/if}
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