<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import type { AuthStep } from '$lib/models/rustModels/AuthStep';
    import type {PasswordDataClientShort} from "$lib/models/rustModels/PasswordDataClientShort"

    let isPushed = $state(false);

    async function handleAuthSubmit() {
        if (isPushed) return;
       
        if (
            !currAuthStep.data.persInn.isValid || 
            !currAuthStep.data.compInn.isValid ||
            !currAuthStep.data.kpp.isValid ||
            !currAuthStep.data.password.isValid
        ) return;

        const sendData: PasswordDataClientShort = {
            password: currAuthStep.data.password.value,
            pers_inn: currAuthStep.data.persInn.value,
            comp_inn: currAuthStep.data.compInn.value,
            kpp: currAuthStep.data.kpp.value
        };

        isPushed = true;

       
        try {
            let nextStep = await invoke<AuthStep>('cmd_session_by_password', {
                data: sendData
            });
            isPushed = false;
            currAuthStep.step = nextStep;

        } catch (err) {
            console.error("Критическая ошибка cmd_auth_with_password:", err);
            let nextStep: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isPushed = false;
            currAuthStep.step = nextStep;
        }
    }

</script>

<div class="w-40 x-self-l">
	<div class='w-v100'>
		<label class="label-close" for="taxOrgIdent">
				4-значный номер налоговой
				
			</label>
		<label class='label-close' for="AuthPassPersInn">
			ИНН физического лица
			<span class='input-tool' data-input-tool="Валидация по алгоритму ФНС">?</span>
		</label>
		<input 
			id="AuthPassPersInn"
			class="input-gr"
			type="text" 
			bind:value={currAuthStep.data.persInn.value} 
			disabled={isPushed}
			placeholder="12 цифр ИНН ФЛ"
			class:input-error={!currAuthStep.data.persInn.isValid}
		/>
	</div>
</div>
	

<div class="w-40 x-self-l">
	<div class='w-v50'>
		<label class='label-close' for="AuthPasscompInn">
			ИНН организации
			<span class='input-tool' data-input-tool="Валидация по алгоритму ФНС">?</span>
		</label>
		<input 
			class="input-gr"
			id="AuthPasswcompInn" 
			type="text" 
			bind:value={currAuthStep.data.compInn.value}
			disabled={isPushed}
			placeholder="10 цифр ИНН ЮЛ"
			class:input-error={!currAuthStep.data.compInn.isValid}/>
	</div>

	<div class='w-v50'>
		<label class='label-close' for="AuthPasswkpp">
			КПП организации
			<span class='input-tool' data-input-tool="Валидация по алгоритму ФНС, для ип - [0, _]">?</span>
		</label>
		<input 
			class="input-gr"
			id="AuthPasswkpp" 
			type="text" 
			bind:value={currAuthStep.data.kpp.value}
			disabled={isPushed} 
			placeholder="9 знаков КПП"
			class:input-error={!currAuthStep.data.kpp.isValid}/>
	</div>
</div>

<div class="w-40 x-self-l">
	<div class='w-v50'>
		<label class='label-close' for="AuthPasswpassword">Пароль</label>
		<input 
			id="AuthPasswpassword" 
			class="input-gr"
			type="password" 
			bind:value={currAuthStep.data.password.value}
			disabled={isPushed} 
			placeholder="Введите пароль"
			
			class:input-error={!currAuthStep.data.password.isValid}/>
	</div>
	
	<div class='w-v50'>
		<label class='label-close' for='AuthPassEnterBut'>
			&nbsp;
		</label>
		<button 
			type="button" 
			class="but-gr x-self-c"
			onclick={handleAuthSubmit}
			disabled={
				isPushed || 
				!currAuthStep.data.persInn.isValid || 
				!currAuthStep.data.compInn.isValid || 
				!currAuthStep.data.kpp.isValid || 
				!currAuthStep.data.password.isValid
			}
			id="AuthPassEnterBut"
		>
			<span class="span-cut">
				{#if isPushed}Вход...{:else}Отправить{/if}
			</span>
		</button>
	</div>
</div>
