<script lang='ts'>
    import {invoke} from "@tauri-apps/api/core";
    import { writeFile } from "@tauri-apps/plugin-fs";
    import { save, open as openFileDialog } from "@tauri-apps/plugin-dialog";

    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import type {AuthStep} from "$lib/models/rustModels/AuthStep";
    import type {IngoingData} from "$lib/models/rustModels/IngoingData";
    import type {SvelteRegistrationData} from "$lib/models/rustModels/SvelteRegistrationData";

    let firstStep = $state(false);
    let secondStep = $state(true);
    let secondStepOne = $state(false);
    let secondStepTwo = $state(true);

    let isPushedMakeDoc = $state(false);
    let isPushedReg = $state(false);
    
    
    let passwordRepeat = $state("")
    
    let initDocPath = $state("");
    let signPath = $state("");

    let isNickExist = $derived(currAuthStep.nick_names.includes(currAuthStep.data.nick.value));

    let isFormValid = $derived(
        isNickExist ||
        isPushedMakeDoc ||
        !currAuthStep.data.nick.isValid ||
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

    let isRegDataValid = $derived(
        isFormValid ||
        !currAuthStep.data.password.isValid ||
        passwordRepeat != currAuthStep.data.password.value ||
        initDocPath.length == 0 ||
        signPath.length == 0
    );

    let docPathsLoaded = $derived(initDocPath.length == 0 || signPath.length == 0 || !currAuthStep.data.password.isValid || passwordRepeat != currAuthStep.data.password.value);



    
    function makeInitregistrationFlags() {
        firstStep = false;
        secondStep = true;
        secondStepOne = false;
        secondStepTwo = true;
    }

    function makeSecondStepOneFlags() {
        firstStep = true;
        secondStep = false;
        secondStepOne = false;
        secondStepTwo = true;
    }

    function makeSecondStepTwoFlags() {
        firstStep = true;
        secondStep = false;
        secondStepOne = true;
        secondStepTwo = false;
    }

    async function selectSigFile() {
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

    async function selectDocFile() {
        try {
            const selected = await openFileDialog({
                multiple: false,
                directory: false,
                title: "Выберите изначальный файл заявления",
                filters: [{ name: 'документ doc', extensions: ['doc'] }]
            });


            if (selected && typeof selected === 'string') {
                initDocPath = selected;
            }
        } catch (err) {
            console.error("Ошибка при выборе файла подписи:", err);
        }
    }

    async function makeInitDoc() {
        if (isFormValid) return;

        isPushedMakeDoc = true;
        

        const ingoingData: IngoingData = {
            nick: currAuthStep.data.nick.value,
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

                initDocPath = docPath;
                makeSecondStepOneFlags();
    
            } else {
                isPushedMakeDoc = false;
            }
            isPushedMakeDoc = false;

        } catch (err) {
            console.error("MAKE INGOING FILE ERR: ", err);
            makeInitregistrationFlags();
            isPushedMakeDoc = false;
            currAuthStep.step = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
        }
    };


    async function handleRegistrationSubmit() {
        if (isPushedReg) return;

        if (isRegDataValid) return;

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
            document_path: initDocPath,  
            signature_path: signPath,
        }

        isPushedReg = true;

        try {
            let next_step: AuthStep = await invoke<AuthStep>("cmd_register_user", {data: regData});
            isPushedReg = false;
            makeInitregistrationFlags();
            await currAuthStep.updateNickNames();
            currAuthStep.add(next_step);
        } catch (err) {
            console.error("Registration FAILED, err = ", err);
            isPushedReg = false;
            let next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            makeInitregistrationFlags();
            currAuthStep.add(next_step);
        }
    }

</script>


<section hidden={firstStep} class="input-section">
    <div class="input-group">
        <label for="nick">Никнейм (Логин для входа)</label>
        <input 
            id="nick" 
            type="text" 
            bind:value={currAuthStep.data.nick.value} 
            disabled={isPushedMakeDoc}
            placeholder="Придумайте уникальный логин"
            class="input-field"
            class:input-error={!currAuthStep.data.nick.isValid || isNickExist}
        />
        {#if !currAuthStep.data.nick.isValid}
            <span class="input-error">Некорректный никнейм (от 1 до 50 символов)</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="surName">Фамилия</label>
        <input 
            id="surName" 
            type="text" 
            bind:value={currAuthStep.data.surName.value} 
            disabled={isPushedMakeDoc}
            placeholder="Только русские буквы"
            class="input-field"
            class:input-error={!currAuthStep.data.surName.isValid}
        />
        {#if !currAuthStep.data.surName.isValid}
            <span class="input-error">Некорректная фамилия</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="firstName">Имя</label>
        <input 
            id="firstName" 
            type="text" 
            bind:value={currAuthStep.data.firstName.value} 
            disabled={isPushedMakeDoc}
            placeholder="Только русские буквы"
            class="input-field"
            class:input-error={!currAuthStep.data.firstName.isValid}
        />
        {#if !currAuthStep.data.firstName.isValid}
            <span class="input-error">Некорректное имя</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="midName">Отчество (при наличии)</label>
        <input 
            id="midName" 
            type="text" 
            bind:value={currAuthStep.data.midName.value} 
            disabled={isPushedMakeDoc}
            placeholder="Только русские буквы"
            class="input-field"
            class:input-error={!currAuthStep.data.midName.isValid}
        />
        {#if !currAuthStep.data.midName.isValid}
            <span class="input-error">Некорректное отчество</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="persInn">Личный ИНН</label>
        <input 
            id="persInn" 
            type="text" 
            bind:value={currAuthStep.data.persInn.value} 
            disabled={isPushedMakeDoc}
            placeholder="12 цифр"
            class="input-field"
            class:input-error={!currAuthStep.data.persInn.isValid}
        />
        {#if !currAuthStep.data.persInn.isValid}
            <span class="input-error">Некорректный ИНН (должно быть 12 цифр)</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="snils">СНИЛС</label>
        <input 
            id="snils" 
            type="text" 
            bind:value={currAuthStep.data.snils.value} 
            disabled={isPushedMakeDoc}
            placeholder="Формат: 000-000-000 00"
            class="input-field"
            class:input-error={!currAuthStep.data.snils.isValid}
        />
        {#if !currAuthStep.data.snils.isValid}
            <span class="input-error">Некорректный СНИЛС</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="compInn">ИНН Организации</label>
        <input 
            id="compInn" 
            type="text" 
            bind:value={currAuthStep.data.compInn.value} 
            disabled={isPushedMakeDoc}
            placeholder="10 цифр"
            class="input-field"
            class:input-error={!currAuthStep.data.compInn.isValid}
        />
        {#if !currAuthStep.data.compInn.isValid}
            <span class="input-error">Некорректный ИНН организации (должно быть 10 цифр)</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="kpp">КПП</label>
        <input 
            id="kpp" 
            type="text" 
            bind:value={currAuthStep.data.kpp.value} 
            disabled={isPushedMakeDoc}
            placeholder="9 цифр"
            class="input-field"
            class:input-error={!currAuthStep.data.kpp.isValid}
        />
        {#if !currAuthStep.data.kpp.isValid}
            <span class="input-error">Некорректный КПП (должно быть 9 цифр)</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="phone">Номер телефона</label>
        <input 
            id="phone" 
            type="tel" 
            bind:value={currAuthStep.data.phone.value} 
            disabled={isPushedMakeDoc}
            placeholder="+7 (900) 000-00-00"
            class="input-field"
            class:input-error={!currAuthStep.data.phone.isValid}
        />
        {#if !currAuthStep.data.phone.isValid}
            <span class="input-error">Некорректный номер телефона</span>
        {/if}
    </div>

    <div class="input-group">
        <label for="email">Электронная почта</label>
        <input 
            id="email" 
            type="email" 
            bind:value={currAuthStep.data.email.value} 
            disabled={isPushedMakeDoc}
            placeholder="example@mail.ru"
            class="input-field"
            class:input-error={!currAuthStep.data.email.isValid}
        />
        {#if !currAuthStep.data.email.isValid}
            <span class="input-error">Некорректный email</span>
        {/if}
    </div>

    <section class="navi-button-section">
        <div class="navi-button-group">
            <button 
                type="button" 
                onclick={makeInitDoc}
                disabled={isFormValid}
                class="main-button"
                id="auth-make-doc-button"
            >
                <span class="navi-buttons.button-icon">
                    {#if isPushedMakeDoc}⏳{:else}🔑{/if}
                </span>
                <span class="button-label">
                    {#if isPushedMakeDoc}Формирование документа...{:else}Сформировать заявление{/if}
                </span>
            </button>
        </div>

        <div class="navi-button-group">
            <button 
                type="button" 
                onclick={makeSecondStepTwoFlags}
                class="main-button"
                id="auth-goto-load-button"
            >
                <span class="button-label">
                    Загрузить готовое
                </span>
            </button>
        </div>
    </section>
</section>

<section hidden={secondStep} class="input-section">

    <div class="input-group">
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
            <span class="input-error">Слишком короткий или простой пароль</span>
        {/if}
    </div>

    <div class="input-group">
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
            <span class="input-error">Пароли не совпадают</span>
        {/if}
    </div>


    <section hidden={secondStepOne} class='input-wide-button-section'>
        
        <div class="input-wide-button-grid">
    
            <label for="docPathSave">Сформированное заявление (.doc)</label>

            <input 
                id="docPathSave"
                type="text" 
                value={initDocPath} 
                class="input-field" 
            />
        

            <button 
                type="button" 
                class="wide-button" 
                onclick={selectDocFile}
                disabled={isPushedReg}
            >
                Выбрать файл
            </button>

            {#if initDocPath === ''}
                <span class="input-error">Необходимо прикрепить файл подписи</span>
            {/if}

        </div>
        


        
        <div class="input-wide-button-grid">
            <label for="sigPathSave">Файл электронной подписи (.doc.sig)</label>

            <input 
                id="sigPathSave"
                type="text" 
                value={signPath} 
                class="input-field" 
            />
    
            <button 
                type="button" 
                class="wide-button" 
                onclick={selectSigFile}
                disabled={isPushedReg}
            >
                Выбрать файл
            </button>


            {#if signPath === ''}
                <span class="input-error">Необходимо прикрепить файл подписи</span>
            {/if}
        </div>
    </section>


    <section hidden={secondStepTwo} class='input-wide-button-section'>
        
        <div class="input-wide-button-grid">
    
            <label for="docPathSave">Сформированное заявление (.doc)</label>

            <input 
                id="docPathSave"
                type="text" 
                value={initDocPath} 
                class="input-field" 
            />
        

            <button 
                type="button" 
                class="wide-button" 
                onclick={selectDocFile}
                disabled={isPushedReg}
            >
                Выбрать файл
            </button>

            {#if initDocPath === ''}
                <span class="input-error">Необходимо прикрепить файл подписи</span>
            {/if}

        </div>

        <div class="input-wide-button-grid">
            <label for="sigPathSave">Файл электронной подписи (.doc.sig)</label>

            <input 
                id="sigPathSave"
                type="text" 
                value={signPath} 
                class="input-field" 
            />
    
            <button 
                type="button" 
                class="wide-button" 
                onclick={selectSigFile}
                disabled={isPushedReg}
            >
                Выбрать файл
            </button>


            {#if signPath === ''}
                <span class="input-error">Необходимо прикрепить файл подписи</span>
            {/if}
        </div>
    </section>

    <div class="input-group">
        <button 
            type="button" 
            onclick={handleRegistrationSubmit}
            disabled={docPathsLoaded || isPushedReg}
            class="main-button"
            id="auth-submit-button"
        >
            <span class="navi-buttons.button-icon">
                {#if isPushedReg}⏳{:else}🔑{/if}
            </span>
            <span class="button-label">
                {#if isPushedReg}Регистрация...{:else}Отправить{/if}
            </span>
        </button>
    </div>


</section>




