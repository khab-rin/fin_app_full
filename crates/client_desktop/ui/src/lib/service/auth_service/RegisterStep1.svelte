<script lang='ts'>
	import { invoke } from "@tauri-apps/api/core";
    import {writeFile} from "@tauri-apps/plugin-fs";
    import {save} from "@tauri-apps/plugin-dialog";
    
    import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";
    import type {AuthStep} from "$lib/models/rustModels/AuthStep";

    import type {IngoingData} from "$lib/models/rustModels/IngoingData";
    import type {InitFiles} from "$lib/models/rustModels/InitFiles"

    

    let getData = $state(true);

    let isPushedMakeDocs = $state(false);

    let isSaved = $state({
        doc: false,
        json: false
    });

    let filePath = {
        doc: "",
        json: ""
    };

    let fileNames = {
        doc: "",
        json: ""
    };

    let passwordRepeat = $state("");

    let isDataValid = $derived(
        !currAuthStep.data.surName.isValid ||
        !currAuthStep.data.firstName.isValid ||
        !currAuthStep.data.midName.isValid ||
        !currAuthStep.data.persInn.isValid ||
        !currAuthStep.data.snils.isValid ||
        !currAuthStep.data.compInn.isValid ||
        !currAuthStep.data.kpp.isValid ||
        !currAuthStep.data.phone.isValid ||
        !currAuthStep.data.email.isValid ||
        !currAuthStep.data.password.isValid ||
        currAuthStep.data.password.value != passwordRepeat
    );

    let docFileBytesArray: Uint8Array | null = null;
    let jsonFileByteArray: Uint8Array | null = null;



    async function makeInitDocs() {
        if (isPushedMakeDocs) return;
        isPushedMakeDocs = true;
        const ingoingData: IngoingData = {
            surName: currAuthStep.data.surName.value,
            firstName: currAuthStep.data.firstName.value,
            midName: currAuthStep.data.midName.value.trim() || null,
            persInn: currAuthStep.data.persInn.value,
            snils: currAuthStep.data.snils.value,
            compInn: currAuthStep.data.compInn.value,
            kpp: currAuthStep.data.kpp.value,
            phone: currAuthStep.data.phone.value,
            email: currAuthStep.data.email.value,
            password: currAuthStep.data.password.value
        };

        try {
            const initFiles = await invoke<InitFiles>("cmd_make_ingoing_doc", 
                {data: ingoingData}
            );
            fileNames.doc = initFiles.docName;
            fileNames.json = initFiles.jsonName;
            docFileBytesArray = new Uint8Array(initFiles.docFile);
            jsonFileByteArray = new Uint8Array(initFiles.jsonFile);
            isPushedMakeDocs = false;
            currAuthStep.data.password.value = "";
            getData = false;
            

        } catch (err) {
            console.error("makeInitDocs FAILED, ERR = ", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isPushedMakeDocs = false;
            currAuthStep.add(next_step);
        }
    }

    async function saveDoc(file: Uint8Array | null, fileType: 'doc' | 'json') {
        if (!file) {
            console.error("Ошибка: Попытка сохранить пустой файл (null)");
            return;
        }

        if (isSaved[fileType]) return;
        isSaved[fileType] = true;

        try {
            const path = await save({
                title: "Сохранить файл",
                defaultPath: fileNames[fileType],
                filters: [{name: "Документы", extensions: [fileType]}]

            });
            if (path) {
                await writeFile(path, file);
                filePath[fileType] = path;
            } else {
                isSaved[fileType] = false;
            }
            
        } catch (err) {
            console.error("FILE SAVING FAILED, ERROR = ", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isSaved[fileType] = false;
            currAuthStep.add(next_step);
        }
    }

</script>


{#if getData}
    <section class='input-section'>
        <div class="input-group">
            <label for="surName">Фамилия</label>
            <input 
                id="surName" 
                type="text" 
                bind:value={currAuthStep.data.surName.value} 
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
                placeholder="example@mail.ru"
                class="input-field"
                class:input-error={!currAuthStep.data.email.isValid}
            />
            {#if !currAuthStep.data.email.isValid}
                <span class="input-error">Некорректный email</span>
            {/if}
        </div>

        <div class="input-group">
            <label for="password">Придумайте пароль приложения</label>
            <input 
                id="password" 
                type="password" 
                bind:value={currAuthStep.data.password.value} 
                disabled={isPushedMakeDocs}
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
                disabled={isPushedMakeDocs}
                placeholder="Введите пароль еще раз"
                class="input-field"
                class:input-error={currAuthStep.data.password.value !== passwordRepeat && passwordRepeat !== ''}
            />
            {#if currAuthStep.data.password.value !== passwordRepeat || passwordRepeat == ''}
                <span class="input-error">Пароли не совпадают</span>
            {/if}
        </div>
    </section>

    <div class="main-button-group">
        <button 
            type="button" 
            onclick={makeInitDocs}
            disabled={isPushedMakeDocs || isDataValid}
            class="main-button"
            id="auth-make-doc-button"
        >
            <span class="navi-buttons.button-icon">
                {#if isPushedMakeDocs}⏳{:else}🔑{/if}
            </span>
            <span class="button-label">
                {#if isPushedMakeDocs}Формирование файлов...{:else}Сформировать файлы{/if}
            </span>
        </button>
    </div>
{:else}
    <section class='input-section'>
        <div class='input-wide-button-grid'>
            <label class='input-wide-button-grid-label' for='docFile'>
                Сохранить файл в формате doc
            </label>
            <input
                id='docFile'
                type='text'
                value={filePath.doc}
                class='input-field'
            >

            <button
                type='button'
                class='wide-button'
                onclick={() => saveDoc(docFileBytesArray, 'doc')}
                disabled={isSaved['doc']}
            >
                Сохранить
            </button>
        </div>

        <div class='input-wide-button-grid'>
            <label class='input-wide-button-grid-label' for='jsonFile'>
                Сохранить файл в формате json
            </label>
            <input
                id='jsonFile'
                type='text'
                value={filePath.json}
                class='input-field'
            >

            <button
                type='button'
                class='wide-button'
                onclick={() => saveDoc(jsonFileByteArray, 'json')}
                disabled={isSaved['json']}
            >
                Сохранить
            </button>
        </div>


    </section>

    <p></p>
{/if}