<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { pageManager } from "$lib/models/MainManager/MainManager.svelte";
    import { currentMchdStep } from "$lib/models/Mchd/mchdManager.svelte"

    import type { MchdTaxFields } from "$lib/models/rustModels/MchdTaxFields";

    let all_tax_powers: MchdTaxFields[] = [];

    let first_done = $derived(
        !currentMchdStep.data.PoaNumber.isValid || 
        !currentMchdStep.data.PoaEndDate.isValid);

    let second_done = $derived(
        !currentMchdStep.data.managerTitle.isValid ||
        !currentMchdStep.data.managerSurName.isValid ||
        !currentMchdStep.data.managerFirstName.isValid ||
        !currentMchdStep.data.managerMidName.isValid ||
        !currentMchdStep.data.managerBirthDay.isValid ||
        !currentMchdStep.data.managerSnils.isValid ||
        !currentMchdStep.data.managerInn.isValid ||
        !currentMchdStep.data.managerisSitizen.isValid);

    let third_done = $derived(
        !currentMchdStep.data.userSurName.isValid ||
        !currentMchdStep.data.userFirstName.isValid ||
        !currentMchdStep.data.userMidName.isValid ||
        !currentMchdStep.data.userBirthDay.isValid ||
        !currentMchdStep.data.userGender.isValid ||
        !currentMchdStep.data.userSnils.isValid ||
        !currentMchdStep.data.userInn.isValid ||
        !currentMchdStep.data.userPassportNumber.isValid ||
        !currentMchdStep.data.userPassportIssueer.isValid ||
        !currentMchdStep.data.userPassportUssuerCode.isValid ||
        !currentMchdStep.data.userisSitizen.isValid);

    

    let first_step = $state(false);
    let second_step = $state(true);
    let thirt_step = $state(true);
    let forth_step = $state(true);

    let isMainPushed = $state(false);

    function switch_first_step() {
        first_step = false;
        second_step = true;
        thirt_step = true;
        forth_step = true;
    }

    function switch_second_step() {
        if ( first_done ) {return;}
        first_step = true;
        second_step = false;
        thirt_step = true;
        forth_step = true;
    }

    function switch_thirt_step() {
        if (first_done || second_done) {return;}
        first_step = true;
        second_step = true;
        thirt_step = false;
        forth_step = true;
    }

    function switch_forth_step() {
        if (first_done || second_done || third_done) {return;}
        first_step = true;
        second_step = true;
        thirt_step = true;
        forth_step = false;
    }

    async function loadPowers() {
        try {
            all_tax_powers = await invoke<MchdTaxFields[]>("cmd_get_tax_powers");
        } catch(err) {
            console.error("Ошибка при получении полномочий:", err);
        }
    }

    onMount(() => {
        loadPowers();
    });

    let selectedPowers = new Set<MchdTaxFields>();

    function togglePower(tax_power: MchdTaxFields) {
        if (selectedPowers.has(tax_power)) {
            selectedPowers.delete(tax_power);
        } else {
            selectedPowers.add(tax_power);
        }
        selectedPowers = selectedPowers;
    }
    
</script>


<div>
    <p class="mchd-p">
        {currentMchdStep.currentText}
    </p>

    <section class="section-section">
        
        <section class="input-section" hidden={first_step}>
            <div class="input-group">
                <label for="PoaNumber">Внутренний номер доверенности организации</label>
                <input
                    id="PoaNumber"
                    type="text"
                    bind:value={currentMchdStep.data.PoaNumber}
                    disabled={isMainPushed}
                    placeholder="строка до 50 знаков"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.PoaNumber.isValid}
                />
                {#if !currentMchdStep.data.PoaNumber.isValid}
                    <span class="input-error-message">Некоректный номер доверенности</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="PoaEndDate">Дата до которой действует доверенность</label>
                <input
                    id="PoaEndDate"
                    type="text"
                    bind:value={currentMchdStep.data.PoaEndDate}
                    disabled={isMainPushed}
                    placeholder="Введите дату в формаде дд.мм.гггг"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.PoaNumber.isValid}
                />
                {#if !currentMchdStep.data.PoaNumber.isValid}
                    <span class="input-error-message">Некорректная дата</span>
                {/if}
            </div>
        </section>

        <button
            type="button"
            hidden={first_step}
            onclick={switch_second_step}
            disabled={isMainPushed || first_done}
            class="main-button"
            id="mchd-tax-firstStep-btn"
            
        >
            <span class="btn-label">
                Завершить 1 этап
            </span>

        </button>

        <section class="input-section" hidden={second_step}>
            <div class="input-group">
                <label for="managerTitle">Должность лица действующего без доверенности (руководителя организации)</label>
                <input
                    id="managerTitle"
                    type="text"
                    bind:value={currentMchdStep.data.managerTitle.value}
                    disabled={isMainPushed}
                    placeholder="Например: Директор"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.managerTitle.isValid}
                />
                {#if !currentMchdStep.data.managerTitle.isValid}
                    <span class="input-error-message">Введите должность</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="mamagerSurName">Фамилия лица действующего без доверенности (руководителя организации)</label>
                <input
                    id="mamagerSurName"
                    type="text"
                    bind:value={currentMchdStep.data.managerSurName.value}
                    disabled={isMainPushed}
                    placeholder="Иванов"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.managerSurName.isValid}
                />
                {#if !currentMchdStep.data.managerSurName.isValid}
                    <span class="input-error-message">Введите фамилию</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="mamagerFirstName">Имя  лица действующего без доверенности (руководителя организации)</label>
                <input
                    id="mamagerFirstName"
                    type="text"
                    bind:value={currentMchdStep.data.managerFirstName.value}
                    disabled={isMainPushed}
                    placeholder="Иван"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.managerFirstName.isValid}
                />
                {#if !currentMchdStep.data.managerFirstName.isValid}
                    <span class="input-error-message">Введите имя</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="managerMidName">Отчество  лица действующего без доверенности (руководителя организации)</label>
                <input
                    id="managerMidName"
                    type="text"
                    bind:value={currentMchdStep.data.managerMidName.value}
                    disabled={isMainPushed}
                    placeholder="Иванович"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.managerMidName.isValid}
                />
                {#if !currentMchdStep.data.managerMidName.isValid}
                    <span class="input-error-message">Введите отчество</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="managerBirthDay">Дата рождения лица действующего без доверенности (руководителя организации)</label>
                <input
                    id="managerBirthDay"
                    type="text"
                    bind:value={currentMchdStep.data.managerBirthDay.value}
                    disabled={isMainPushed}
                    placeholder="дд.мм.гггг"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.managerBirthDay.isValid}
                />
                {#if !currentMchdStep.data.managerBirthDay.isValid}
                    <span class="input-error-message">Некорректная дата</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="managerSnils">СНИЛС лица действующего без доверенности (руководителя организации)</label>
                <input
                    id="managerSnils"
                    type="text"
                    bind:value={currentMchdStep.data.managerSnils.value}
                    disabled={isMainPushed}
                    placeholder="000-000-000 00"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.managerSnils.isValid}
                />
                {#if !currentMchdStep.data.managerSnils.isValid}
                    <span class="input-error-message">Некорректный СНИЛС</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="managerInn">ИНН физического лица действующего без доверенности (руководителя организации)</label>
                <input
                    id="managerInn"
                    type="text"
                    bind:value={currentMchdStep.data.managerInn.value}
                    disabled={isMainPushed}
                    placeholder="12 цифр"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.managerInn.isValid}
                />
                {#if !currentMchdStep.data.managerInn.isValid}
                    <span class="input-error-message">Некорректный ИНН</span>
                {/if}
            </div>
        </section>

        <button
            type="button"
            hidden={second_step}
            onclick={switch_thirt_step}
            disabled={isMainPushed || second_done}
            class="main-button"
            id="mchd-tax-secondStep-btn"
        >
            <span class="btn-label">
                Завершить 2 этап
            </span>
        </button>

        <section class="input-section" hidden={thirt_step}>
            <div class="input-group">
                <label for="userSurName">Фамилия пользователя</label>
                <input
                    id="userSurName"
                    type="text"
                    bind:value={currentMchdStep.data.userSurName.value}
                    disabled={isMainPushed}
                    placeholder="Иванов"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userSurName.isValid}
                />
                {#if !currentMchdStep.data.userSurName.isValid}
                    <span class="input-error-message">Введите фамилию</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="userFirstName">Имя пользователя</label>
                <input
                    id="userFirstName"
                    type="text"
                    bind:value={currentMchdStep.data.userFirstName.value}
                    disabled={isMainPushed}
                    placeholder="Иван"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userFirstName.isValid}
                />
                {#if !currentMchdStep.data.userFirstName.isValid}
                    <span class="input-error-message">Введите имя</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="userMidName">Отчество пользователя</label>
                <input
                    id="userMidName"
                    type="text"
                    bind:value={currentMchdStep.data.userMidName.value}
                    disabled={isMainPushed}
                    placeholder="Иванович"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userMidName.isValid}
                />
                {#if !currentMchdStep.data.userMidName.isValid}
                    <span class="input-error-message">Введите отчество</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="userBirthDay">Дата рождения пользователя</label>
                <input
                    id="userBirthDay"
                    type="text"
                    bind:value={currentMchdStep.data.userBirthDay.value}
                    disabled={isMainPushed}
                    placeholder="дд.мм.гггг"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userBirthDay.isValid}
                />
                {#if !currentMchdStep.data.userBirthDay.isValid}
                    <span class="input-error-message">Некорректная дата</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="userGender">Пол пользователя</label>
                <select 
                    id="userGender"
                    bind:value={currentMchdStep.data.userGender.value}
                    disabled={isMainPushed}
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userGender.isValid}
                >
                    <option value="" disabled selected>Выберите пол</option>
                    <option value="1">1 — Мужской</option>
                    <option value="2">2 — Женский</option>
                </select>
            </div>

            <div class="input-group">
                <label for="userSnils">СНИЛС пользователя</label>
                <input
                    id="userSnils"
                    type="text"
                    bind:value={currentMchdStep.data.userSnils.value}
                    disabled={isMainPushed}
                    placeholder="000-000-000 00"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userSnils.isValid}
                />
                {#if !currentMchdStep.data.userSnils.isValid}
                    <span class="input-error-message">Некорректный СНИЛС</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="userInn">ИНН пользователя</label>
                <input
                    id="userInn"
                    type="text"
                    bind:value={currentMchdStep.data.userInn.value}
                    disabled={isMainPushed}
                    placeholder="12 цифр"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userInn.isValid}
                />
                {#if !currentMchdStep.data.userInn.isValid}
                    <span class="input-error-message">Некорректный ИНН</span>
                {/if}
            </div>

            <div class="input-group">
                <label for="userPassportNumber">Серия и номер паспорта</label>
                <input
                    id="userPassportNumber"
                    type="text"
                    bind:value={currentMchdStep.data.userPassportNumber.value}
                    disabled={isMainPushed}
                    placeholder="00 00 000000"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userPassportNumber.isValid}
                />
            </div>

            <div class="input-group">
                <label for="userPassportIssueer">Кем выдан пасорт  пользователя</label>
                <input
                    id="userPassportIssueer"
                    type="text"
                    bind:value={currentMchdStep.data.userPassportIssueer.value}
                    disabled={isMainPushed}
                    placeholder="Наименование органа"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userPassportIssueer.isValid}
                />
            </div>

            <div class="input-group">
                <label for="userPassportUssuerCode">Код подразделения</label>
                <input
                    id="userPassportUssuerCode"
                    type="text"
                    bind:value={currentMchdStep.data.userPassportUssuerCode.value}
                    disabled={isMainPushed}
                    placeholder="000-000"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userPassportUssuerCode.isValid}
                />
            </div>

            <!-- Гражданство -->
            <div class="input-group">
                <label for="userisSitizen">Гражданство</label>
                <select
                    id="userisSitizen"
                    bind:value={currentMchdStep.data.userisSitizen.value}
                    disabled={isMainPushed}
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userisSitizen.isValid}
                >
                    <option value="" disabled selected>Выберите статус гражданства</option>
                    <option value="1">1 — Гражданин РФ</option>
                    <option value="2">2 — Иностранный гражданин</option>
                    <option value="3">3 — Лицо без гражданства</option>
                </select>
                {#if !currentMchdStep.data.userisSitizen.isValid}
                    <span class="input-error-message">Выберите статус из списка</span>
                {/if}
            </div>
        </section>

        <button
            type="button"
            hidden={thirt_step}
            onclick={ switch_forth_step }
            disabled={isMainPushed || third_done}
            class="main-button"
        >
            <span class="btn-label">
                Завершить 3 этап
            </span>
        </button>

        <section class="input-section" hidden={forth_step}>
            <div class="powers-container">
                {#each all_tax_powers as tax_power (tax_power)}
                    <div class="input-group">
                        {#await currentMchdStep.get_power_info(tax_power)}
                            <div class="loading-placeholder">Загрузка информации...</div>
                        {:then info}
                            <label class="checkbox-item">
                                <input
                                    type="checkbox"
                                    bind:group={currentMchdStep.data.selectedPowers.value}
                                    value={tax_power}
                                    disabled={isMainPushed}
                                    class="checkbox-input"
                                />
                                <span class="power-text">
                                    <span class="power-code">{info.code}</span> — {info.name}
                                </span>
                            </label>
                        {:catch error}
                            <span class="input-error-message">Ошибка: {error.message}</span>
                        {/await}
                    </div>
                {/each}
            </div>

            {#if currentMchdStep.data.selectedPowers.isValid}
                <span class="input-error-message">Выберите хотя бы одно полномочие</span>
            {/if}
        </section>




    </section>

</div>