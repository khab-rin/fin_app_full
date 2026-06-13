<script lang='ts'>
	import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import { writeFile } from "@tauri-apps/plugin-fs";
    import { save, open as openFileDialog } from "@tauri-apps/plugin-dialog";

    import type {AuthStep} from "$lib/models/rustModels/AuthStep";
    import type {IngoingData} from "$lib/models/rustModels/IngoingData";
    import type {SvelteRegistrationData} from "$lib/models/rustModels/SvelteRegistrationData"
	
    

    let isPushedReg = $state(false);
    let isPushedDoc = $state(false);
    let isGeneratedDoc = $state(false);
    let passwordRepeat = $state("")
    let docPathSaved = $state("");
    let signPath = $state("");

    let isFormInvalid = $derived(
        isPushedDoc ||
        !currAuthStep.data.surName.isValid ||
        !currAuthStep.data.firstName.isValid ||
        !currAuthStep.data.midName.isValid ||
        !currAuthStep.data.persInn.isValid ||
        !currAuthStep.data.snils.isValid ||
        !currAuthStep.data.compInn.isValid ||
        !currAuthStep.data.kpp.isValid ||
        !currAuthStep.data.phone.isValid ||
        !currAuthStep.data.email.isValid
    );
    


    async function handleSelectSigFile() {
        try {
            const selected = await openFileDialog({
                multiple: false,
                directory: false,
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
        if (isFormInvalid) return;

        isPushedDoc = true;

        const ingoingData: IngoingData = {
            sur_name: currAuthStep.data.surName.value,
            first_name: currAuthStep.data.firstName.value,
            mid_name: currAuthStep.data.midName.value.trim() || null,
            pers_inn: currAuthStep.data.persInn.value,
            snils: currAuthStep.data.snils.value,
            comp_inn: currAuthStep.data.compInn.value,
            kpp: currAuthStep.data.kpp.value,
            phone: currAuthStep.data.phone.value,
            email: currAuthStep.data.email.value
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
                isPushedDoc = false;
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
            !currAuthStep.data.nick.isValid ||
            !currAuthStep.data.surName.isValid ||
            !currAuthStep.data.firstName.isValid ||
            !currAuthStep.data.midName.isValid ||
            !currAuthStep.data.persInn.isValid ||
            !currAuthStep.data.snils.isValid ||
            !currAuthStep.data.compInn.isValid ||
            !currAuthStep.data.kpp.isValid ||
            !currAuthStep.data.password.isValid ||
            !currAuthStep.data.phone.isValid ||
            !currAuthStep.data.email.isValid ||
            docPathSaved.length == 0 ||
            signPath.length == 0
        ) return;

        const regData: SvelteRegistrationData = {
            nick: currAuthStep.data.nick.value,
            sur_name: currAuthStep.data.surName.value,
            first_name: currAuthStep.data.firstName.value,
            mid_name: currAuthStep.data.midName.value,
            pers_inn: currAuthStep.data.persInn.value,
            snils: currAuthStep.data.snils.value,
            comp_inn: currAuthStep.data.compInn.value,
            kpp: currAuthStep.data.kpp.value,
            password: currAuthStep.data.password.value,
            phone: currAuthStep.data.phone.value,
            email: currAuthStep.data.email.value,
            document_path: docPathSaved,  
            signature_path: signPath,
        }

        isPushedReg = true;

        try {
            let next_step: AuthStep = await invoke<AuthStep>("cmd_register_user", {data: regData});
            isPushedReg = false;
            currAuthStep.add(next_step);
        } catch (err) {
            console.error("Registration FAILED, err = ", err);
            isPushedReg = false;
            let next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            currAuthStep.add(next_step);
        }
    }

    function handleGoBack() {
        currAuthStep.back(); 
    }

    function handleGoNext() {
        currAuthStep.next(); 
    }

    function handleGoPassword() {
        const next_step: AuthStep = {NeedPassword: {text: ""}};
        currAuthStep.add(next_step); 
    }
</script>

<div class="auth-card">
    <p class="auth-text-step">
        {currAuthStep.currentText}
    </p>

    {#if !isGeneratedDoc}
        <div class="form-group">
            <label for="surName">Фамилия</label>
            <input 
                id="surName" 
                type="text" 
                bind:value={currAuthStep.data.surName.value} 
                disabled={isPushedDoc}
                placeholder="Только русские буквы"
                class="input-field"
                class:input-error={!currAuthStep.data.surName.isValid}
            />
            {#if !currAuthStep.data.surName.isValid}
                <span class="error-message">Некорректная фамилия</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="firstName">Имя</label>
            <input 
                id="firstName" 
                type="text" 
                bind:value={currAuthStep.data.firstName.value} 
                disabled={isPushedDoc}
                placeholder="Только русские буквы"
                class="input-field"
                class:input-error={!currAuthStep.data.firstName.isValid}
            />
            {#if !currAuthStep.data.firstName.isValid}
                <span class="error-message">Некорректное имя</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="midName">Отчество (при наличии)</label>
            <input 
                id="midName" 
                type="text" 
                bind:value={currAuthStep.data.midName.value} 
                disabled={isPushedDoc}
                placeholder="Только русские буквы"
                class="input-field"
                class:input-error={!currAuthStep.data.midName.isValid}
            />
            {#if !currAuthStep.data.midName.isValid}
                <span class="error-message">Некорректное отчество</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="persInn">Личный ИНН</label>
            <input 
                id="persInn" 
                type="text" 
                bind:value={currAuthStep.data.persInn.value} 
                disabled={isPushedDoc}
                placeholder="12 цифр"
                class="input-field"
                class:input-error={!currAuthStep.data.persInn.isValid}
            />
            {#if !currAuthStep.data.persInn.isValid}
                <span class="error-message">Некорректный ИНН (должно быть 12 цифр)</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="snils">СНИЛС</label>
            <input 
                id="snils" 
                type="text" 
                bind:value={currAuthStep.data.snils.value} 
                disabled={isPushedDoc}
                placeholder="Формат: 000-000-000 00"
                class="input-field"
                class:input-error={!currAuthStep.data.snils.isValid}
            />
            {#if !currAuthStep.data.snils.isValid}
                <span class="error-message">Некорректный СНИЛС</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="compInn">ИНН Организации</label>
            <input 
                id="compInn" 
                type="text" 
                bind:value={currAuthStep.data.compInn.value} 
                disabled={isPushedDoc}
                placeholder="10 цифр"
                class="input-field"
                class:input-error={!currAuthStep.data.compInn.isValid}
            />
            {#if !currAuthStep.data.compInn.isValid}
                <span class="error-message">Некорректный ИНН организации (должно быть 10 цифр)</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="kpp">КПП</label>
            <input 
                id="kpp" 
                type="text" 
                bind:value={currAuthStep.data.kpp.value} 
                disabled={isPushedDoc}
                placeholder="9 цифр"
                class="input-field"
                class:input-error={!currAuthStep.data.kpp.isValid}
            />
            {#if !currAuthStep.data.kpp.isValid}
                <span class="error-message">Некорректный КПП (должно быть 9 цифр)</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="phone">Номер телефона</label>
            <input 
                id="phone" 
                type="tel" 
                bind:value={currAuthStep.data.phone.value} 
                disabled={isPushedDoc}
                placeholder="+7 (900) 000-00-00"
                class="input-field"
                class:input-error={!currAuthStep.data.phone.isValid}
            />
            {#if !currAuthStep.data.phone.isValid}
                <span class="error-message">Некорректный номер телефона</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="email">Электронная почта</label>
            <input 
                id="email" 
                type="email" 
                bind:value={currAuthStep.data.email.value} 
                disabled={isPushedDoc}
                placeholder="example@mail.ru"
                class="input-field"
                class:input-error={!currAuthStep.data.email.isValid}
            />
            {#if !currAuthStep.data.email.isValid}
                <span class="error-message">Некорректный email</span>
            {/if}
        </div>

        <section class="navi-buttons">
            <button 
                type="button" 
                onclick={handleIngoingDoc}
                disabled={isFormInvalid}
                class="main-button"
                id="auth-submit-btn"
            >
                <span class="navi-buttons.btn-icon">
                    {#if isPushedDoc}⏳{:else}🔑{/if}
                </span>
                <span class="btn-label">
                    {#if isPushedDoc}Формирование документа...{:else}Отправить{/if}
                </span>
            </button>

            <div class="buttons-grid-row">
                <button type="button" onclick={handleGoBack} class="nav-btn-item">
                    <span class="nav-btn-text">Назад</span>
                </button>

                <button type="button" onclick={handleGoPassword} disabled={isPushedDoc} class="nav-btn-item">
                    <span class="nav-btn-text">Ввод пароля</span>
                </button>

                <button type="button" onclick={handleGoNext} class="nav-btn-item">
                    <span class="nav-btn-text">Вперед</span>
                </button>
            </div>

        </section>




    {:else}
        <p class="success-notice">🎉 Шаг 1 завершен: заявление успешно сформировано и сохранено!</p>
        
        <hr class="divider" />

        <div class="form-group">
            <label for="nick">Никнейм (Логин для входа)</label>
            <input 
                id="nick" 
                type="text" 
                bind:value={currAuthStep.data.nick.value} 
                disabled={isPushedReg}
                placeholder="Придумайте уникальный логин"
                class="input-field"
                class:input-error={!currAuthStep.data.nick.isValid}
            />
            {#if !currAuthStep.data.nick.isValid}
                <span class="error-message">Некорректный никнейм (от 1 до 50 символов)</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="password">Придумайте пароль приложения</label>
            <input 
                id="password" 
                type="password" 
                bind:value={currAuthStep.data.password.value} 
                disabled={isPushedReg}
                placeholder="Минимум 6 символов"
                class="input-field"
                class:input-error={!currAuthStep.data.password.isValid}
            />
            {#if !currAuthStep.data.password.isValid}
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
                class:input-error={currAuthStep.data.password.value !== passwordRepeat && passwordRepeat !== ''}
            />
            {#if currAuthStep.data.password.value !== passwordRepeat && passwordRepeat !== ''}
                <span class="error-message">Пароли не совпадают</span>
            {/if}
        </div>

        <hr class="divider" />

        <div class="form-group">
            <label for="document">Сформированное заявление (.doc)</label>
            <div class="file-picker-wrapper">
                <input 
                    id="document"
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


        <section class="navi-buttons">
            <button 
                type="button" 
                onclick={handleRegistrationSubmit}
                disabled={!currAuthStep.data.nick.isValid ||
                    isPushedReg || 
                    currAuthStep.data.password.value !== passwordRepeat ||  
                    !docPathSaved ||
                    !signPath
                }
                class="main-button"
                id="auth-submit-btn"
            >
                <span class="navi-buttons.btn-icon">
                    {#if isPushedReg}⏳{:else}🔑{/if}
                </span>
                <span class="btn-label">
                    {#if isPushedReg}Регистрация...{:else}Отправить{/if}
                </span>
            </button>

            <div class="buttons-grid-row">
                <button type="button" onclick={handleGoBack} class="nav-btn-item">
                    <span class="nav-btn-text">Назад</span>
                </button>

                <button type="button" onclick={handleGoPassword} disabled={isPushedReg} class="nav-btn-item">
                    <span class="nav-btn-text">Ввод пароля</span>
                </button>

                <button type="button" onclick={handleGoNext} class="nav-btn-item">
                    <span class="nav-btn-text">Вперед</span>
                </button>
            </div>

        </section>
    {/if}
</div>



