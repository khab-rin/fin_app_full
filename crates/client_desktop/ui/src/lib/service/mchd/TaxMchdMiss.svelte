<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { SvelteSet } from 'svelte/reactivity';
    import { currentMchdStep } from "$lib/models/Mchd/mchdManager.svelte"

    import type { MchdTaxFields } from "$lib/models/rustModels/MchdTaxFields";
    import type {MchdStep} from "$lib/models/rustModels/MchdStep";


    let selectedPowers = new SvelteSet<MchdTaxFields>();
    let allTaxPowers = $state<MchdTaxFields[]>([]);

    let firstDone = $derived(
        !currentMchdStep.data.PoaNumber.isValid || 
        !currentMchdStep.data.PoaEndDate.isValid);

    let secondDone = $derived(
        !currentMchdStep.data.managerTitle.isValid ||
        !currentMchdStep.data.managerSurName.isValid ||
        !currentMchdStep.data.managerFirstName.isValid ||
        !currentMchdStep.data.managerMidName.isValid ||
        !currentMchdStep.data.managerBirthDay.isValid ||
        !currentMchdStep.data.managerSnils.isValid ||
        !currentMchdStep.data.managerInn.isValid ||
        !currentMchdStep.data.managerIsCitizen.isValid);

    let thirdDone = $derived(
        !currentMchdStep.data.userSurName.isValid ||
        !currentMchdStep.data.userFirstName.isValid ||
        !currentMchdStep.data.userMidName.isValid ||
        !currentMchdStep.data.userBirthDay.isValid ||
        !currentMchdStep.data.userGender.isValid ||
        !currentMchdStep.data.userSnils.isValid ||
        !currentMchdStep.data.userInn.isValid ||
        !currentMchdStep.data.userPassportNumber.isValid ||
        !currentMchdStep.data.userPassportIssueDate ||
        !currentMchdStep.data.userPassportIssueer.isValid ||
        !currentMchdStep.data.userPassportUssuerCode.isValid ||
        !currentMchdStep.data.userIsCitizen.isValid);

    let forthDone = $derived(selectedPowers.size == 0);

    let allDone = $derived(firstDone || secondDone || thirdDone || forthDone)

    let firstStep = $state(false);
    let secondStep = $state(true);
    let thirdStep = $state(true);
    let forthStep = $state(true);
    let allPowers = $state(false);

    let isMainPushed = $state(false);

    function switchFirstStep() {
        firstStep = false;
        secondStep = true;
        thirdStep = true;
        forthStep = true;
    }

    function switchSecondStep() {
        if ( firstDone ) {return;}
        firstStep = true;
        secondStep = false;
        thirdStep = true;
        forthStep = true;
    }

    function switchThirdStep() {
        if (firstDone || secondDone) {return;}
        firstStep = true;
        secondStep = true;
        thirdStep = false;
        forthStep = true;
    }

    function switchForthStep() {
        if (firstDone || secondDone || thirdDone) {return;}
        firstStep = true;
        secondStep = true;
        thirdStep = true;
        forthStep = false;
    }

    async function loadPowers() {
        try {
            allTaxPowers = await invoke<MchdTaxFields[]>("cmd_get_tax_powers");
        } catch(err) {
            console.error("Ошибка при получении полномочий:", err);
        }
    }

    function togglePower(tax_power: MchdTaxFields) {
        if (selectedPowers.has(tax_power)) {
            selectedPowers.delete(tax_power);
        } else {
            selectedPowers.add(tax_power);
        }
    }

    function selectAllPowers() {
        if (allPowers) {
            selectedPowers.clear();
            allPowers = false;
        } else {
            allTaxPowers.forEach(p => selectedPowers.add(p));
            allPowers = true;
        }
    }

    async function lendMchd() {
        if (allDone) {return}
        let data = {
            PoaNumber: currentMchdStep.data.PoaNumber.value,
            PoaEndDate: currentMchdStep.data.PoaEndDate.value,

            managerTitle: currentMchdStep.data.managerTitle.value,
            managerSurName: currentMchdStep.data.managerSurName.value,
            managerFirstName: currentMchdStep.data.managerFirstName.value,
            managerMidName: currentMchdStep.data.managerMidName.value,
            managerBirthDay: currentMchdStep.data.managerBirthDay.value,
            managerSnils: currentMchdStep.data.managerSnils.value,
            managerInn: currentMchdStep.data.managerInn.value,
            managerIsCitizen: currentMchdStep.data.managerIsCitizen.value,

            userSurName: currentMchdStep.data.userSurName.value,
            userFirstName: currentMchdStep.data.userFirstName.value,
            userMidName: currentMchdStep.data.userMidName.value,
            userBirthDay: currentMchdStep.data.userBirthDay.value,
            userGender: currentMchdStep.data.userGender.value,
            userSnils: currentMchdStep.data.userSnils.value,
            userInn: currentMchdStep.data.userInn.value,
            userPassportNumber: currentMchdStep.data.userPassportNumber.value,
            userPassportIssueDate: currentMchdStep.data.userPassportIssueDate.value,
            userPassportIssueer: currentMchdStep.data.userPassportIssueer.value,
            userPassportUssuerCode: currentMchdStep.data.userPassportUssuerCode.value,
            userIsCitizen: currentMchdStep.data.userIsCitizen.value,
            powers: selectedPowers
        }

        try {
            const next_step = await invoke<MchdStep>("cmd_register_tax_mchd", {new_mchd_data: data})
            currentMchdStep.add(next_step);
        } catch(err) {
            console.error("cmd_register_tax_mchd FAILED, err = ", err);
            const next_step: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.add(next_step);
        }
    }

    onMount(() => {
        loadPowers();
    });
    
</script>


<div>
    <section class="section-section">
        
        <section class="input-section" hidden={firstStep}>
            <div class="input-group">
                <label class="input-group-label" for="PoaNumber">Внутренний номер доверенности организации</label>
                <input
                    id="PoaNumber"
                    type="text"
                    bind:value={currentMchdStep.data.PoaNumber.value}
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
                <label class="input-group-label" for="PoaEndDate">Дата до которой действует доверенность</label>
                <input
                    id="PoaEndDate"
                    type="text"
                    bind:value={currentMchdStep.data.PoaEndDate.value}
                    disabled={isMainPushed}
                    placeholder="Введите дату в формаде дд.мм.гггг"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.PoaNumber.isValid}
                />
                {#if !currentMchdStep.data.PoaEndDate.isValid}
                    <span class="input-error-message">Некорректная дата</span>
                {/if}
            </div>
        </section>

        <button
            type="button"
            hidden={firstStep}
            onclick={switchSecondStep}
            disabled={isMainPushed || firstDone}
            class="main-button"
            id="mchd-tax-firstStep-btn"
            
        >
            <span class="btn-label">
                Завершить 1 этап
            </span>

        </button>

        <section class="input-section" hidden={secondStep}>
            <div class="input-group">
                <label class="input-group-label" for="managerTitle">Должность лица действующего без доверенности (руководителя организации)</label>
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
                <label class="input-group-label" for="mamagerSurName">Фамилия лица действующего без доверенности (руководителя организации)</label>
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
                <label class="input-group-label" for="mamagerFirstName">Имя лица действующего без доверенности (руководителя организации)</label>
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
                <label class="input-group-label" for="managerMidName">Отчество  лица действующего без доверенности (руководителя организации)</label>
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
                <label class="input-group-label" for="managerBirthDay">Дата рождения лица действующего без доверенности (руководителя организации)</label>
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
                <label class="input-group-label" for="managerSnils">СНИЛС лица действующего без доверенности (руководителя организации)</label>
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
                <label class="input-group-label" for="managerInn">ИНН физического лица действующего без доверенности (руководителя организации)</label>
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

            <div class="input-group">
                <label class="input-group-label" for="managerIsCitizen">Гражданство</label>
                <select
                    id="managerIsCitizen"
                    bind:value={currentMchdStep.data.managerIsCitizen.value}
                    disabled={isMainPushed}
                    class="input-field"
                    class:input-error={!currentMchdStep.data.managerIsCitizen.isValid}
                >
                    <option value="" disabled selected>Выберите статус гражданства</option>
                    <option value="1">1 — Гражданин РФ</option>
                    <option value="2">2 — Иностранный гражданин</option>
                    <option value="3">3 — Лицо без гражданства</option>
                </select>
                {#if !currentMchdStep.data.managerIsCitizen.isValid}
                    <span class="input-error-message">Выберите статус из списка</span>
                {/if}
            </div>
        </section>

        <button
            type="button"
            hidden={secondStep}
            onclick={switchThirdStep}
            disabled={isMainPushed || secondDone}
            class="main-button"
            id="mchd-tax-secondStep-btn"
        >
            <span class="btn-label">
                Завершить 2 этап
            </span>
        </button>

        <section class="input-section" hidden={thirdStep}>
            <div class="input-group">
                <label class="input-group-label" for="userSurName">Фамилия пользователя</label>
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
                <label class="input-group-label" for="userFirstName">Имя пользователя</label>
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
                <label class="input-group-label" for="userMidName">Отчество пользователя</label>
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
                <label class="input-group-label" for="userBirthDay">Дата рождения пользователя</label>
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
                <label class="input-group-label" for="userGender">Пол пользователя</label>
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
                <label class="input-group-label" for="userSnils">СНИЛС пользователя</label>
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
                <label class="input-group-label" for="userInn">ИНН пользователя</label>
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
                <label class="input-group-label" for="userPassportNumber">Серия и номер паспорта</label>
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
                <label class="input-group-label" for="userPassportNumber">Дата выдачи пасспорта</label>
                <input
                    id="userPassportNumber"
                    type="text"
                    bind:value={currentMchdStep.data.userPassportIssueDate.value}
                    disabled={isMainPushed}
                    placeholder="00.00.0000"
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userPassportIssueDate.isValid}
                />
            </div>

            <div class="input-group">
                <label class="input-group-label" for="userPassportIssueer">Кем выдан пасорт  пользователя</label>
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
                <label class="input-group-label" for="userPassportUssuerCode">Код подразделения</label>
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
                <label class="input-group-label" for="userIsCitizen">Гражданство</label>
                <select
                    id="userIsCitizen"
                    bind:value={currentMchdStep.data.userIsCitizen.value}
                    disabled={isMainPushed}
                    class="input-field"
                    class:input-error={!currentMchdStep.data.userIsCitizen.isValid}
                >
                    <option value="" disabled selected>Выберите статус гражданства</option>
                    <option value="1">1 — Гражданин РФ</option>
                    <option value="2">2 — Иностранный гражданин</option>
                    <option value="3">3 — Лицо без гражданства</option>
                </select>
                {#if !currentMchdStep.data.userIsCitizen.isValid}
                    <span class="input-error-message">Выберите статус из списка</span>
                {/if}
            </div>
        </section>

        <button
            type="button"
            hidden={thirdStep}
            onclick={ switchForthStep }
            disabled={isMainPushed || thirdDone}
            class="main-button"
        >
            <span class="btn-label">
                Завершить 3 этап
            </span>
        </button>


        <section class="input-section" hidden={forthStep}>
            <h3 class="h3">Выберите полномочия</h3>

            <label class="check-box-label">
                <input
                    type="checkbox"
                    checked={allPowers}
                    onchange={() => selectAllPowers()}
                />
                <span class="check-box-span">Выбрать все машинописные полномочия для взаимодействия с ФНС РФ</span>
            </label>

            <ul class="input-group">
                {#each allTaxPowers as power (power)}
                    <li class="check-box-li">
                        <label class="check-box-label">
                            <input
                                type="checkbox"
                                checked={selectedPowers.has(power)}
                                onchange={() => togglePower(power)}
                                disabled={allPowers}
                            />
                            
                            <span class="check-box-small-span">{power}</span>

                            {#await currentMchdStep.get_power_info(power)}
                                <span class="check-box-span">Загрузка...</span>
                            {:then info}
                                <span class="check-box-span">{info?.name}</span>
                            {:catch error}
                                <span title={error} class="check-box-span" style="color: red;">Ошибка</span>
                            {/await}
                        </label>
                    </li>
                {/each}
            </ul>

        </section>

        <section class="navi-buttons">
            <div class="buttons-grid-row">
                <button
                    type="button"
                    hidden={!firstStep}
                    onclick={switchFirstStep}
                >
                    <span class="navi-btn-text">
                        Вернуться к этапу 1
                    </span>
                </button>
            </div>

            <div class="buttons-grid-row">
                <button
                    type="button"
                    hidden={!secondStep}
                    onclick={switchSecondStep}
                >
                    <span class="navi-btn-text">
                        Вернуться к этапу 2
                    </span>
                </button>
            </div>

            <div class="buttons-grid-row">
                <button
                    type="button"
                    hidden={!thirdStep}
                    onclick={switchThirdStep}
                >
                    <span class="navi-btn-text">
                        Вернуться к этапу 3
                    </span>
                </button>
            </div>

            <div class="buttons-grid-row">
                <button
                    type="button"
                    hidden={!forthStep}
                    onclick={switchForthStep}
                >
                    <span class="navi-btn-text">
                        Вернуться к этапу 4
                    </span>
                </button>
            </div>
            
            <button
                type="button"
                id="lend-mchd"
                class="main-button"
                onclick={lendMchd}
                disabled={allDone}>
                <span class=main-batton-span>
                    Зарегистрировать
                </span>
            </button>

        </section>




    </section>

</div>