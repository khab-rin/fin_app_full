<script lang='ts'>
    import {onMount} from 'svelte';
    import {invoke} from '@tauri-apps/api/core';
    import {open as OpenFileDialog} from '@tauri-apps/plugin-dialog';

	import {dialogBackdrop} from '$lib/rules/dialogBorders';
	import {fitText} from '$lib/rules/text';
    import {FieldValidator} from '$lib/models/Auth/FieldValidator.svelte';

    import type {OperationStep} from '$lib/models/rustModels/OperationStep';
    import type {RasBicAcc} from '$lib/models/rustModels/RasBicAcc';
	import { operStep } from '$lib/models/Operation/OperationManager.svelte';

    let path = $state('');
    let selectedBankAcc = $state<RasBicAcc | null>(null);
    let bankAccounts = $state<RasBicAcc[]>([]);

    let isPushedAccLoad = $state(false);
    let isPushFileLoad = $state(false);
    let isPushParseStatement = $state(false);
    let isPushAddAcc = $state(false);

    let bic = new FieldValidator('Bic', "");
    let rasAcc = new FieldValidator('RasAcc', "");
    let bankAccReady = $derived(
        !bic.isValid || !rasAcc.isValid
    );

    let parseStatementDisabled = $derived(
        isPushParseStatement ||
        path === '' ||
        selectedBankAcc === null
    );
	
	function bankAccStr(bankAcc: RasBicAcc | null) {
		if (bankAcc == null) {
			return "Счет не выбран"
		} else {
			return `бик ${bankAcc.bic}, счет ${bankAcc.ras_acc}`
		}
	}

	function selectElem(id: string, value: RasBicAcc): RasBicAcc {
		(document.getElementById(id) as HTMLDialogElement)?.close()
		return value;
	}

	function openModal(id: string) {
        (document.getElementById(id) as HTMLDialogElement)?.showModal();
    }

	function closeModal(id: string) {
        (document.getElementById(id) as HTMLDialogElement)?.close();
    }
	
    async function selectFile() {
        if (isPushFileLoad) return;
        isPushFileLoad = true;

        try {
            const selected = await OpenFileDialog({
                multiple: false,
                directory: false,
                title: "Выберите банковскую выписку",
                filters: [{name: "документ txt", extensions: ["txt"]}]
            });

            if (selected) {
                if (typeof selected === 'string') {
                    path = selected;
                    isPushFileLoad = false;
                } else {
                    isPushFileLoad = false;
                }
            } else {
                isPushFileLoad = false;
            }
        } catch(err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("get_own_bank_accs FAILED BY PATH LOAD, err = ", err);
            operStep.step = next_step;
        }
    }

    async function freshAccs() {
        try {
            bankAccounts = await invoke<RasBicAcc[]>('cmd_get_comp_bank_accs', {});

        } catch (err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("cmd_get_own_bank_accs FAILED, err = ", err);
            operStep.step = next_step;
        }
    };

    async function addAcc() {
        if (isPushAddAcc) return;
        isPushAddAcc = true;
        try {
            let data = {
                bic: bic.value,
                rasAcc: rasAcc.value
            };
            bankAccounts = await invoke<RasBicAcc[]>('cmd_add_comp_bank_acc', data);
			bic.value="";
			rasAcc.value="";
            isPushAddAcc= false;

        } catch (err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("cmd_add_comp_bank_acc FAILED, err = ", err);
            isPushAddAcc= false;
            operStep.step = next_step;
        }
    };

    async function parseStatement() {
        if (isPushParseStatement) return;
        if (path == '') return;
        if (selectedBankAcc == null) return;

        isPushParseStatement = true;
        
        let data = {
            rasBicAcc: selectedBankAcc,
            path: path
        };


        try {
            const next_step: OperationStep = await invoke<OperationStep>("cmd_load_bank_statement", data);
            isPushParseStatement = false;
            operStep.step = next_step;
        } catch(err) {
            console.error("FUN cmd_load_bank_statement FAILED, err = ", err);
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isPushParseStatement = false;
            operStep.step = next_step;

        }
    }

    onMount(async() => {
        freshAccs()
    });
</script>

<dialog 
	class="dial-t-l"
	onclick={dialogBackdrop}
    id="OperStatProcBankAccs" 
>
	<div class='w-100'>
		<p class= 'x-self-c x-cont-c'>
			Выбор счета
		</p>
	</div>

	{#if bankAccounts.length > 0}
		{#each bankAccounts as acc (acc)}
			<div class='w-100'>
				<div class='w-v100'>
					<button 
						type="button" 
						class="but-ye"
						onclick={()=>{selectedBankAcc=selectElem("OperStatProcBankAccs", acc)}}
					>
						<span class="t-fill" use:fitText>
							{bankAccStr(acc)}
						</span>
					</button>
				</div>
			</div>
		{/each}

		<div class='w-100'>
			<div class='w-v100'>
				<button 
					class="but-ye"
					type="button"
					onclick={()=>closeModal("OperStatProcBankAccs")}
					>			
					<span>
						Отмена
					</span>
				</button>
			</div>
		</div>
	{:else}
		<div class='w-100'>
			<p class= 'x-self-c, x-cont-c'>
				На этом устройстве еще нет сохраненных счетов
			</p>
		</div>
	{/if}
</dialog>



<div class='w-100'>
    <div class='w-v100'>
        <label class="label-close" for="statementPath">
            Укажите путь до файла выписки
        </label>
        <input
            type='text'
            id='statementPath'
            value={path}
            class='input-gr'
        />
	</div>
</div>

<div class='w-100'>
    <div class='w-v100'>
		<button
			type='button'
			id='statementPathButton'
			class='but-gr'
			onclick={selectFile}
			disabled={isPushFileLoad}
		>
			Выбрать файл выписки
		</button>
	</div>
</div>

<div class='w-100'>
    <div class='w-v100'>	
		<button
			type="button"
			class="but-gr"
			disabled={isPushedAccLoad}
			
			onclick={()=>openModal("OperStatProcBankAccs")}
		>
			<span>
				{bankAccStr(selectedBankAcc)}
			</span>
		</button>
	</div>
</div>

<div class='w-100'>
    <div class='w-v100'>
		<button
			type='button'
			class='but-gr'
			disabled={parseStatementDisabled}
			onclick ={parseStatement}
		>	
			{#if isPushParseStatement}
				<span class="loading-content">
					<span class="loading-spinner"></span>
					Обработка...
				</span>
			{:else}
				Загрузить выписку
			{/if}

		</button>
	</div>
</div>

<div class='w-100'>
    <div class='w-v100'>
        <label class='label-close' for='inputBic'>
            Введите бик Вашего банка
        </label>
        <input
            type='text'
            id='inputBic'
            bind:value={bic.value}
            disabled={isPushAddAcc}
            placeholder="9 цифр"
            class='input-ye'
            class:input-error={!bic.isValid}
        />
	</div>
</div>

<div class='w-100'>
    <div class='w-v100'>
        <label class='label-close' for='operStateLoaderRassAcc'>
            Введите номер расчетного счета
        </label>
        <input
            type='text'
            id='operStateLoaderRassAcc'
            bind:value={rasAcc.value}
            disabled={isPushAddAcc}
            placeholder="20 цифр"
            class='input-ye'
            class:input-error={!rasAcc.isValid}
        />
	</div>
</div>

<div class='w-100'>
    <div class='w-v100'>
		<button
			type='button'
			class='but-gr'
			disabled={isPushAddAcc || bankAccReady}
			onclick ={addAcc}
		>	
			{#if isPushAddAcc}
				<span class="loading-content">
					<span class="loading-spinner"></span>
					Добавление...
				</span>
			{:else}
				Добавить расчетный счет
			{/if}
		</button>
	</div>
</div>

