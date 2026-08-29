<script lang='ts'>
    import {onMount} from 'svelte';
    import {invoke} from '@tauri-apps/api/core';
    import {open as OpenFileDialog} from '@tauri-apps/plugin-dialog';
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

    let dialogRef = $state<HTMLDialogElement | null>(null);

    function openAccModal() {
        if (dialogRef) dialogRef.showModal();
    }

    function closeAccModal() {
        if (dialogRef) dialogRef.close();
    }

    function selectAcc(acc: RasBicAcc) {
        if (isPushedAccLoad) return;

        isPushedAccLoad = true;

        selectedBankAcc = acc;
        closeAccModal();

        isPushedAccLoad = false;
        
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

<section class='group-one'>
    <div>
        <label class="green-input-label" for="statementPath">
            Укажите путь до файла выписки
        </label>
        <input
            type='text'
            id='statementPath'
            value={path}
            class='green-field'

        />
        <button
            type='button'
            id='statementPathButton'
            class='green-button'
            onclick={selectFile}
            disabled={isPushFileLoad}
        >
            Загрузите файл выписки
        </button>
    </div>

	<div>
		<button
			type="button"
			class="green-button"
			disabled={isPushedAccLoad}
			
			onclick={openAccModal}
		>

			<span class="wide-button-span">
				{bankAccStr(selectedBankAcc)}
			</span>
    </button>


	</div>
</section>


<dialog 
    bind:this={dialogRef} 
    class="selector-dialog"
    onclick={(e) => { if (e.target === dialogRef) closeAccModal(); }}
>

    <h5>Выбор счета</h5>


    <div class="group-one">
        {#if bankAccounts.length > 0}
            <ul>
                {#each bankAccounts as acc (acc)}
                    <li>
						<button 
							type="button" 
							class="yellow-button"
							onclick={() => selectAcc(acc)}
						>
							<span class="yellow-button-span">
								{bankAccStr(acc)}
							</span>
						</button>
                    </li>
                {/each}
            </ul>

            <div>
                <button class="yellow-button"
                    type="button"
                    onclick={closeAccModal}
                    >
                    
                    <span class="yellow-button-span">
                        Отмена
                    </span>


                </button>
            </div>

        {:else}
            <p>На этом устройстве еще нет сохраненных счетов</p>
        {/if}
    </div>

</dialog>

<div class = 'group-one'>
    <button
        type='button'
        class='green-button'
        disabled={parseStatementDisabled}
        onclick ={parseStatement}
    >

        Загрузить выписку

    </button>
</div>

<section class='group-one'>
    <div>
        <label class='yellow-field-label' for='inputBic'>
            Введите бик Вашего банка
        </label>
        <input
            type='text'
            id='inputBic'
            bind:value={bic.value}
            disabled={isPushAddAcc}
            placeholder="9 цифр"
            class='yellow-field'
            class:input-error={!bic.isValid}
        />
        {#if !bic.isValid}
            <span class="yellow-field-error-span">
                Некорректный БИК
            </span>
        {/if}
    </div>

    <div>
        <label class='yellow-field-label' for='operStateLoaderRassAcc'>
            Введите номер расчетного счета
        </label>
        <input
            type='text'
            id='operStateLoaderRassAcc'
            bind:value={rasAcc.value}
            disabled={isPushAddAcc}
            placeholder="20 цифр"
            class='yellow-field'
            class:input-error={!rasAcc.isValid}
        />
        {#if !rasAcc.isValid}
            <span class="yellow-field-error-span">
                Некорректный БИК
            </span>
        {/if}
    </div>
</section>

<div class = 'main-button-group'>
    <button
        type='button'
        class='main-button'
        disabled={isPushAddAcc || bankAccReady}
        onclick ={addAcc}
    >
        Добавить расчетный счет
    </button>
</div>