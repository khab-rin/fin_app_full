<script lang='ts'>
    import {onMount} from 'svelte';
    import {currAuthStep} from '$lib/models/Auth/AuthStep.svelte';
    import {AuthStepType} from '$lib/models/Auth/AuthValues';

    let surName = "";
    let firstName = "";
    let midName = "";
    let persInn = "";
    let snils = "";
    let compInn = "";
    let kpp = "";
    let phone = "";
    let email = "";

    onMount(async() => {
        if (AuthStepType.RegisterStep1Duplicate in currAuthStep.step) {
            const data = currAuthStep.step.RegisterStep1Duplicate;
            surName = data.sur_name;
            firstName = data.first_name;
            midName = data.mid_name;
            persInn = data.pers_inn;
            snils = data.snils;
            compInn = data.comp_inn;
            kpp = data.kpp;
            phone = data.phone;
            email = data.email;
             
        } else {
            console.error("asdf");
        }
    });

</script>

<div class='group-3'>
	<h3 class='text-cut text-c text-base text-box'>Данные дубликата</h3>
</div>

<div class="group-3">
	<div>
		<label class='label-close' for='AuthDuplSurName'>
			Фамилия
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={surName}
			type='text'
			id='AuthDuplSurName'
		>
	</div>

	<div>
		<label class='label-close' for='AuthDuplName'>
			Имя
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={firstName}
			type='text'
			id='AuthDuplName'
		>
	</div>

	<div>
		<label class='label-close' for='AuthDuplMidName'>
			Отчество
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={midName}
			type='text'
			id='AuthDuplMidName'
		>
	</div>
</div>

<div class='group-2'>
	<div>
		<label class='label-close' for='AuthDuplPersInn'>
			ИНН физлица
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={persInn}
			type='text'
			id='AuthDuplPersInn'
		>
	</div>

	<div>
		<label class='label-close' for='AuthDuplSnils'>
			СНИЛС
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={snils}
			type='text'
			id='AuthDuplSnils'
		>
	</div>
</div>


<div class='group-2'>
	<div>
		<label class='label-close' for='AuthDupCompInn'>
			ИНН организации
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={compInn}
			type='text'
			id='AuthDupCompInn'
		>
	</div>

	<div>
		<label class='label-close' for='AuthDupKpp'>
			КПП
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={kpp}
			type='text'
			id='AuthDupKpp'
		>
	</div>
</div>

<div class="group-2">
	<div>
		<label class='label-close' for='AuthDupPhone'>
			Телефон
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={phone}
			type='text'
			id='AuthDupPhone'
		>
	</div>

	<div>
		<label class='label-close' for='AuthDupEmail'>
			Email
		</label>
		<input
			class='input-ye'
			disabled={true}
			bind:value={email}
			type='text'
			id='AuthDupEmail'
		>
	</div>
</div>