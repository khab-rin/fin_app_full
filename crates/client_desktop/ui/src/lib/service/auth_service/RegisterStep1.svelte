<script lang='ts'>
    import {onMount} from "svelte";
	import { invoke } from "@tauri-apps/api/core";
    
    import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";
    import type {AuthStep} from "$lib/models/rustModels/AuthStep";

    import type {RegInitData} from "$lib/models/rustModels/RegInitData";
    import type {BoxUuid} from "$lib/models/rustModels/BoxUuid";

    
    let isPushedMakeDocs = $state(false);

    let passwordRepeat = $state("");

    

    let deviceId = "";
    

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

    async function finishStep1() {
        if (isPushedMakeDocs) return;
        isPushedMakeDocs = true;
        const regInitData: RegInitData = {
            surName: currAuthStep.data.surName.value,
            firstName: currAuthStep.data.firstName.value,
            midName: currAuthStep.data.midName.value.trim() || null,
            persInn: currAuthStep.data.persInn.value,
            snils: currAuthStep.data.snils.value,
            compInn: currAuthStep.data.compInn.value,
            kpp: currAuthStep.data.kpp.value,
            phone: currAuthStep.data.phone.value,
            email: currAuthStep.data.email.value,
            password: currAuthStep.data.password.value,
        };

        try {
            const next_step = await invoke<AuthStep>("cmd_register_step1", 
                {data: regInitData}
            );
            isPushedMakeDocs = false;
            currAuthStep.add(next_step);
        } catch (err) {
            console.error("makeInitDocs FAILED, ERR = ", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isPushedMakeDocs = false;
            currAuthStep.add(next_step);
        }
    }

    onMount(async() => {
        try {
            deviceId = await invoke<BoxUuid>("cmd_get_device_id", {}); 
        } catch(err) {
            console.error("FAILED BY cmd_register_step1, err = ", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            currAuthStep.add(next_step);
        }
    });

</script>



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
        onclick={finishStep1}
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
