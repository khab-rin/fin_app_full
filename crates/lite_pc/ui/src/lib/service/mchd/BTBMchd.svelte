<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { SvelteSet } from 'svelte/reactivity';
    import { currentMchdStep } from "$lib/models/Mchd/mchdManager.svelte"

    import type { HomeMchdPower } from "$lib/models/rustModels/HomeMchdPower";
    import type {MchdStep} from "$lib/models/rustModels/MchdStep";
    import type {MchdType} from "$lib/models/rustModels/MchdType";


    let selectedPowers = new SvelteSet<HomeMchdPower>();
    let allPowers = $state<HomeMchdPower[]>([]);

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

    let firstStep = $state(true);
    let secondStep = $state(false);
    let thirdStep = $state(false);
    let forthStep = $state(false);
    let allPowersSelected = $derived(allPowers.length == selectedPowers.size);

    let isMainPushed = $state(false);

    function switchFirstStep() {
        firstStep = true;
        secondStep = false;
        thirdStep = false;
        forthStep = false;
    }

    function switchSecondStep() {
        if ( firstDone ) {return;}
        firstStep = false;
        secondStep = true;
        thirdStep = false;
        forthStep = false;
    }

    function switchThirdStep() {
        if (firstDone || secondDone) {return;}
        firstStep = false;
        secondStep = false;
        thirdStep = true;
        forthStep = false;
    }

    function switchForthStep() {
        if (firstDone || secondDone || thirdDone) {return;}
        firstStep = false;
        secondStep = false;
        thirdStep = false;
        forthStep = true;
    }

    async function loadPowers() {
        try {
            allPowers = await invoke<HomeMchdPower[]>("cmd_get_all_btb_powers");
        } catch(err) {
            console.error("Ошибка при получении полномочий:", err);
        }
    }

    function togglePower(tax_power: HomeMchdPower) {
        if (selectedPowers.has(tax_power)) {
            selectedPowers.delete(tax_power);
        } else {
            selectedPowers.add(tax_power);
        }
    }

    function selectAllPowers() {
        if (allPowersSelected) {
            selectedPowers.clear();
            allPowersSelected = false;
        } else {
            allPowers.forEach(p => selectedPowers.add(p));
            allPowersSelected = true;
        }
    }

    async function lendMchd() {
        if (allDone) {return}
        let data = {
            poaNumber: currentMchdStep.data.PoaNumber.value,
            poaEndDate: currentMchdStep.data.PoaEndDate.value,
            taxOrgIdent: currentMchdStep.data.taxOrgIdent.value,

            managerTittle: currentMchdStep.data.managerTitle.value,
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
            mchdType: "BTBMchd" satisfies MchdType,
            powers: Array.from(selectedPowers)
        }

        try {
            const nextStep = await invoke<MchdStep>("cmd_make_xml_doc_files", {data: data})
            currentMchdStep.step = nextStep;
        } catch(err) {
            console.error("cmd_make_xml_doc_files FAILED, err = ", err);
            const nextStep: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.step = nextStep;
        }
    }

    onMount(() => {
        loadPowers();
    });
    
</script>

{#if firstStep}
	<section class="w-40">
		<div class='w-v50'>
			<label class="label-close" for="MchdBtbPoaNumber">Внутренний номер доверенности организации</label>
			<input
				id="MchdBtbPoaNumber"
				type="text"
				bind:value={currentMchdStep.data.PoaNumber.value}
				disabled={isMainPushed}
				placeholder="строка до 50 знаков"
				class="input-gr"
				class:input-error={!currentMchdStep.data.PoaNumber.isValid}
			/>
			{#if !currentMchdStep.data.PoaNumber.isValid}
				<span class="green-error-span">Некоректный номер доверенности</span>
			{/if}
		</div>

		<div class='w-v50'>
			<label class="label-close" for="MchdBtbPoaEndDate">Дата до которой действует доверенность</label>
			<input
				id="MchdBtbPoaEndDate"
				type="text"
				bind:value={currentMchdStep.data.PoaEndDate.value}
				disabled={isMainPushed}
				placeholder="Введите дату в формаде дд.мм.гггг"
				class="input-gr"
				class:input-error={!currentMchdStep.data.PoaNumber.isValid}
			/>
			{#if !currentMchdStep.data.PoaEndDate.isValid}
				<span class="text-fill">Некорректная дата</span>
			{/if}
		</div>
	</section>

	<div class='w-40'>
		<button
			type="button"
			onclick={switchSecondStep}
			disabled={isMainPushed || firstDone}
			class="but-gr"
			id="MchdBtbmchd-tax-firstStep-button"
			
		>
			<span class="text-fill">
				Завершить 1 этап
			</span>

		</button>
	</div>
{/if}


{#if secondStep}
	<section class='w-66 x-self-l'>
		<h5 class='w-v100 x-self-c x-ce t-fill'>
			Заполните данные лица с правом действия без доверенности
		</h5>
	</section>

	<section class="w-40">
		<div class='w-v50'>
			<label class="label-close t-fill" for="MchdBtbmanagerTitle">Должность лица действующего без доверенности (руководителя организации)</label>
			<input
				id="MchdBtbmanagerTitle"
				type="text"
				bind:value={currentMchdStep.data.managerTitle.value}
				disabled={isMainPushed}
				placeholder="Например: Директор"
				class="input-gr"
				class:input-error={!currentMchdStep.data.managerTitle.isValid}
			/>
			{#if !currentMchdStep.data.managerTitle.isValid}
				<span class="text-fill">Введите должность</span>
			{/if}
		</div>
		<div class='w-v50'>
			<label class="label-close t-fill" for="MchdBtbuserIsCitizen">Гражданство</label>
			<select
				id="MchdBtbuserIsCitizen"
				bind:value={currentMchdStep.data.userIsCitizen.value}
				disabled={isMainPushed}
				class="input-gr"
				class:input-error={!currentMchdStep.data.userIsCitizen.isValid}
			>
				<option value="" disabled selected>Выберите статус гражданства</option>
				<option value="1">1 — Гражданин РФ</option>
				<option value="2">2 — Иностранный гражданин</option>
				<option value="3">3 — Лицо без гражданства</option>
			</select>
			{#if !currentMchdStep.data.userIsCitizen.isValid}
				<span class="text-fill">Выберите статус из списка</span>
			{/if}
		</div>
	</section>

	<section class='w-60'>
		<div  class='w-v33'>
			<label class="label-close" for="MchdBtbmamagerSurName">Фамилия лица действующего без доверенности (руководителя организации)</label>
			<input
				id="MchdBtbmamagerSurName"
				type="text"
				bind:value={currentMchdStep.data.managerSurName.value}
				disabled={isMainPushed}
				placeholder="Иванов"
				class="input-gr"
				class:input-error={!currentMchdStep.data.managerSurName.isValid}
			/>
			{#if !currentMchdStep.data.managerSurName.isValid}
				<span class="text-fill">Введите фамилию</span>
			{/if}
		</div>

		<div  class='w-v33'>
			<label class="label-close" for="MchdBtbmamagerFirstName">Имя лица действующего без доверенности (руководителя организации)</label>
			<input
				id="MchdBtbmamagerFirstName"
				type="text"
				bind:value={currentMchdStep.data.managerFirstName.value}
				disabled={isMainPushed}
				placeholder="Иван"
				class="input-gr"
				class:input-error={!currentMchdStep.data.managerFirstName.isValid}
			/>
			{#if !currentMchdStep.data.managerFirstName.isValid}
				<span class="text-fill">Введите имя</span>
			{/if}
		</div>

		<div  class='w-v33'>
			<label 
				class="label-close" for="MchdBtbmanagerMidName"
			>Отчество  лица действующего без доверенности (руководителя организации)</label>
			<input
				id="MchdBtbmanagerMidName"
				type="text"
				bind:value={currentMchdStep.data.managerMidName.value}
				disabled={isMainPushed}
				placeholder="Иванович"
				class="input-gr"
				class:input-error={!currentMchdStep.data.managerMidName.isValid}
			/>
			{#if !currentMchdStep.data.managerMidName.isValid}
				<span class="text-fill">Введите отчество</span>
			{/if}
		</div>
	</section>

	<section class='w-60'>
		<div>
			<label class="label-close" for="MchdBtbmanagerBirthDay">Дата рождения лица действующего без доверенности (руководителя организации)</label>
			<input
				id="MchdBtbmanagerBirthDay"
				type="text"
				bind:value={currentMchdStep.data.managerBirthDay.value}
				disabled={isMainPushed}
				placeholder="дд.мм.гггг"
				class="input-gr"
				class:input-error={!currentMchdStep.data.managerBirthDay.isValid}
			/>
			{#if !currentMchdStep.data.managerBirthDay.isValid}
				<span class="text-fill">Некорректная дата</span>
			{/if}
		</div>

		<div>
			<label class="label-close" for="MchdBtbmanagerSnils">СНИЛС лица действующего без доверенности (руководителя организации)</label>
			<input
				id="MchdBtbmanagerSnils"
				type="text"
				bind:value={currentMchdStep.data.managerSnils.value}
				disabled={isMainPushed}
				placeholder="000-000-000 00"
				class="input-gr"
				class:input-error={!currentMchdStep.data.managerSnils.isValid}
			/>
			{#if !currentMchdStep.data.managerSnils.isValid}
				<span class="text-fill">Некорректный СНИЛС</span>
			{/if}
		</div>

		<div>
			<label class="label-close" for="MchdBtbmanagerInn">ИНН физического лица действующего без доверенности (руководителя организации)</label>
			<input
				id="MchdBtbmanagerInn"
				type="text"
				bind:value={currentMchdStep.data.managerInn.value}
				disabled={isMainPushed}
				placeholder="12 цифр"
				class="input-gr"
				class:input-error={!currentMchdStep.data.managerInn.isValid}
			/>
			{#if !currentMchdStep.data.managerInn.isValid}
				<span class="text-fill">Некорректный ИНН</span>
			{/if}
		</div>
	</section>

	<section class='w-40'>
		<button
			type="button"
			onclick={switchThirdStep}
			disabled={isMainPushed || secondDone}
			class="but-gr"
			id="MchdBtbmchd-tax-secondStep-button"
		>
			<span class="text-fill">
				Завершить 2 этап
			</span>
		</button>
	</section>
{/if}


{#if thirdStep}
	<section class='w-60 s-self-l'>
		<h4 class='t-fill x-self-c x-ce'>
			Заполните данные доверителя
		</h4>
	</section>

	<section class="w-66">
		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserSurName">Фамилия пользователя</label>
			<input
				id="MchdBtbuserSurName"
				type="text"
				bind:value={currentMchdStep.data.userSurName.value}
				disabled={isMainPushed}
				placeholder="Иванов"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userSurName.isValid}
			/>
			{#if !currentMchdStep.data.userSurName.isValid}
				<span class="text-fill">Введите фамилию</span>
			{/if}
		</div>

		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserFirstName">Имя пользователя</label>
			<input
				id="MchdBtbuserFirstName"
				type="text"
				bind:value={currentMchdStep.data.userFirstName.value}
				disabled={isMainPushed}
				placeholder="Иван"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userFirstName.isValid}
			/>
			{#if !currentMchdStep.data.userFirstName.isValid}
				<span class="text-fill">Введите имя</span>
			{/if}
		</div>

		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserMidName">Отчество пользователя</label>
			<input
				id="MchdBtbuserMidName"
				type="text"
				bind:value={currentMchdStep.data.userMidName.value}
				disabled={isMainPushed}
				placeholder="Иванович"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userMidName.isValid}
			/>
			{#if !currentMchdStep.data.userMidName.isValid}
				<span class="text-fill">Введите отчество</span>
			{/if}
		</div>
	</section>

	<section class='w-66'>
		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserBirthDay">Дата рождения пользователя</label>
			<input
				id="MchdBtbuserBirthDay"
				type="text"
				bind:value={currentMchdStep.data.userBirthDay.value}
				disabled={isMainPushed}
				placeholder="дд.мм.гггг"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userBirthDay.isValid}
			/>
			{#if !currentMchdStep.data.userBirthDay.isValid}
				<span class="text-fill">Некорректная дата</span>
			{/if}
		</div>

		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserGender">Пол пользователя</label>
			<select 
				id="MchdBtbuserGender"
				bind:value={currentMchdStep.data.userGender.value}
				disabled={isMainPushed}
				class="input-gr"
				class:input-error={!currentMchdStep.data.userGender.isValid}
			>
				<option value="" disabled selected>Выберите пол</option>
				<option value="1">1 — Мужской</option>
				<option value="2">2 — Женский</option>
			</select>
		</div>

		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserIsCitizen">Гражданство</label>
			<select
				id="MchdBtbuserIsCitizen"
				bind:value={currentMchdStep.data.userIsCitizen.value}
				disabled={isMainPushed}
				class="input-gr"
				class:input-error={!currentMchdStep.data.userIsCitizen.isValid}
			>
				<option value="" disabled selected>Выберите статус гражданства</option>
				<option value="1">1 — Гражданин РФ</option>
				<option value="2">2 — Иностранный гражданин</option>
				<option value="3">3 — Лицо без гражданства</option>
			</select>
			{#if !currentMchdStep.data.userIsCitizen.isValid}
				<span class="text-fill">Выберите статус из списка</span>
			{/if}
		</div>
	</section>

	<section class=w-40>
		<div class='w-v50'>
			<label class="label-close" for="MchdBtbuserSnils">СНИЛС пользователя</label>
			<input
				id="MchdBtbuserSnils"
				type="text"
				bind:value={currentMchdStep.data.userSnils.value}
				disabled={isMainPushed}
				placeholder="000-000-000 00"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userSnils.isValid}
			/>
			{#if !currentMchdStep.data.userSnils.isValid}
				<span class="text-fill">Некорректный СНИЛС</span>
			{/if}
		</div>

		<div class='w-v50'>
			<label class="label-close" for="MchdBtbuserInn">ИНН пользователя</label>
			<input
				id="MchdBtbuserInn"
				type="text"
				bind:value={currentMchdStep.data.userInn.value}
				disabled={isMainPushed}
				placeholder="12 цифр"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userInn.isValid}
			/>
			{#if !currentMchdStep.data.userInn.isValid}
				<span class="text-fill">Некорректный ИНН</span>
			{/if}
		</div>
	</section>

	<section class='w-60'>
		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserPassportNumber">Серия и номер паспорта</label>
			<input
				id="MchdBtbuserPassportNumber"
				type="text"
				bind:value={currentMchdStep.data.userPassportNumber.value}
				disabled={isMainPushed}
				placeholder="00 00 000000"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userPassportNumber.isValid}
			/>
		</div>

		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserPassportNumber">Дата выдачи пасспорта</label>
			<input
				id="MchdBtbuserPassportNumber"
				type="text"
				bind:value={currentMchdStep.data.userPassportIssueDate.value}
				disabled={isMainPushed}
				placeholder="00.00.0000"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userPassportIssueDate.isValid}
			/>
		</div>
		<div class='w-v33'>
			<label class="label-close" for="MchdBtbuserPassportUssuerCode">Код подразделения</label>
			<input
				id="MchdBtbuserPassportUssuerCode"
				type="text"
				bind:value={currentMchdStep.data.userPassportUssuerCode.value}
				disabled={isMainPushed}
				placeholder="000-000"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userPassportUssuerCode.isValid}
			/>
		</div>
	</section>

	<section class='w-60'>
		<div class='w-v100'>
			<label class="label-close" for="MchdBtbuserPassportIssueer">Кем выдан пасорт  пользователя</label>
			<input
				id="MchdBtbuserPassportIssueer"
				type="text"
				bind:value={currentMchdStep.data.userPassportIssueer.value}
				disabled={isMainPushed}
				placeholder="Наименование органа"
				class="input-gr"
				class:input-error={!currentMchdStep.data.userPassportIssueer.isValid}
			/>
		</div>

	</section>

	<section class='w-40'>
		<button
			type="button"
			onclick={ switchForthStep }
			disabled={isMainPushed || thirdDone}
			class="but-gr"
		>
			<span class="text-fill">
				Завершить 3 этап
			</span>
		</button>
	</section>
{/if}



{#if forthStep}
	<section class='w-100'>
		<h4 class='w-v100 t-fill x-self-c x-ce'>
			Выберите полномочия
		</h4>
	</section>

	<div class='w-100'>
		<input
			class='w-g5 x-self-c y-self-c'
			type="checkbox"
			checked={allPowersSelected}
			onchange={() => selectAllPowers()}
		/>

		<div class='w-g95'>
			<button
				class='but-bl'
				type="button"
				onclick={() => selectAllPowers()}
				style={allPowersSelected 
					? "background-color: #1d4ed8 !important; border-color: #1e40af !important; color: #ffffff !important;" 
					: ""
				}
			>
				<span class='t-fill x-self-l x-st'>
					Выбрать все машинописные полномочия для взаимодействия с контрагентами
				</span>
			</button>
		</div>
	</div>

	{#each allPowers as power (power)}
		<section class='w-100'>
			<input
				class='w-g5 x-self-c y-self-c'
				type='checkbox'
				checked={selectedPowers.has(power)}
				onchange={() => togglePower(power)}
			>
			<div class='w-g95'>
				<button
					class="but-gr"
					type="button"
					onclick={() => togglePower(power)}
					style={selectedPowers.has(power) 
						? "background-color: #15803d !important; border-color: #166534 !important; color: #ffffff !important;" 
						: ""
					}
						
				>
					<span class='t-fill w-g100 x-st'>
						{#await currentMchdStep.get_power_info(power)}
							Загрузка
						{:then info} 
							{info?.code} - {info?.name}
						{:catch error}
							ошибка - {error}
						{/await}
					</span>	
				</button>
			</div>
		</section>
	{/each}
{/if}



<section class='w-40'>
    <div class='w-v50'>
        <button
            class="but-pu"
            type="button"
            hidden={firstStep}
            onclick={switchFirstStep}
        >
            <span class="text-cut">
                Этап 1
            </span>
        </button>
    </div>

    <div class='w-v50'>
        <button
            class="but-pu"
            type="button"
            hidden={secondStep}
            onclick={switchSecondStep}
        >
            <span class="text-cut">
                Этап 2
            </span>
        </button>
    </div>
</section>

<section class='w-40'>
    <div class='w-v50'>
        <button
            class="but-pu"
            type="button"
            hidden={thirdStep}
            onclick={switchThirdStep}
        >
            <span class="text-cut">
                Этап 3
            </span>
        </button>
    </div>

    <div class='w-v50'>
        <button
            class="but-pu"
            type="button"
            hidden={forthStep}
            onclick={switchForthStep}
        >
            <span class="text-cut">
                Этап 4
            </span>
        </button>
    </div>
</section>


<section class='w-40'>
	<button
		type="button"
		id="MchdBtbMchdBtbLendBut"
		class="but-bl"
		onclick={lendMchd}
		disabled={allDone}>
		<span class=main-batton-span>
			Зарегистрировать
		</span>
	</button>
</section>
