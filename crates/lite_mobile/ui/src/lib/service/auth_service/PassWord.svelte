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

<div class="w-100">
	<div class='w-v100'>
		<label class='label-close' for="persInn">
			ИНН физического лица
			<span class='input-tool' data-input-tool="Валидация по алгоритму ФНС">?</span>
		</label>
		<input 
			id="persInn"
			class="input-gr"
			type="text" 
			bind:value={currAuthStep.data.persInn.value} 
			disabled={isPushed}
			placeholder="12 цифр ИНН ФЛ"
			class:input-error={!currAuthStep.data.persInn.isValid}
		/>
	</div>
</div>

<div class="w-100">

	<div class='w-v100'>
		<label class='label-close' for="compInn">
			ИНН организации
			<span class='input-tool' data-input-tool="Валидация по алгоритму ФНС">?</span>
		</label>
		<input 
			class="input-gr"
			id="innOrg" 
			type="text" 
			bind:value={currAuthStep.data.compInn.value}
			disabled={isPushed}
			placeholder="10 цифр ИНН ЮЛ"
			class:input-error={!currAuthStep.data.compInn.isValid}/>
	</div>
</div>


<div class="w-100">
	<div class='w-v100'>
		<label class='label-close' for="kpp">
			КПП организации
			<span class='input-tool' data-input-tool="Валидация по алгоритму ФНС, для ип - [0, _]">?</span>
		</label>
		<input 
			class="input-gr"
			id="kppOrg" 
			type="text" 
			bind:value={currAuthStep.data.kpp.value}
			disabled={isPushed} 
			placeholder="9 знаков КПП"
			class:input-error={!currAuthStep.data.kpp.isValid}/>
	</div>
</div>


<div class="w-100">
	<div class='w-v100'>
		<label class='label-close' for="password">
			Пароль
			<span class='input-tool' data-input-tool="Не менее 8 символов">?</span>
		</label>
		<input 
			id="password" 
			class="input-gr"
			type="password" 
			bind:value={currAuthStep.data.password.value}
			disabled={isPushed} 
			placeholder="Введите пароль"
			
			class:input-error={!currAuthStep.data.password.isValid}/>
	</div>
</div>


<div class="w-100">
	<button 
		type="button" 
		class="but-gr"
		onclick={handleAuthSubmit}
		disabled={
			isPushed || 
			!currAuthStep.data.persInn.isValid || 
			!currAuthStep.data.compInn.isValid || 
			!currAuthStep.data.kpp.isValid || 
			!currAuthStep.data.password.isValid
		}
		
		id="auth-submit-button"
	>	
		{#if isPushed}
			<span class="loading-content">
				<span class="loading-spinner"></span>
				Авторизация...
			</span>
		{:else}
			Авторизоваться
		{/if}
	</button>

</div>
