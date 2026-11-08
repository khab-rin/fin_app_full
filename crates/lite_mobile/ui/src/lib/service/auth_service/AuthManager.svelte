<script lang='ts'>
    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import {AuthStepType} from "$lib/models/Auth/AuthValues";

    import type { AuthStep } from "$lib/models/rustModels/AuthStep";

    let isPassword = $derived(AuthStepType.Password in currAuthStep.step);
    let isRegistrerStep1 = $derived(AuthStepType.RegisterStep1 in currAuthStep.step);
    let isRegistrerStep2 = $derived(AuthStepType.RegisterStep2 in currAuthStep.step);
    let isNickName = $derived(AuthStepType.NickName in currAuthStep.step);

    function goToPassword() {
        let nextStep: AuthStep = { Password: { text: "" } };
        currAuthStep.step = nextStep;
    }

    function goToRegisterStep1() {
        let nextStep: AuthStep = { RegisterStep1: { text: "Заполните поля регистрации строго как в документах" } };
        currAuthStep.step = nextStep;
    }

    function goToRegisterStep2() {
        let nextStep: AuthStep = { RegisterStep2: { text: "Укажите путь до xml файла заявления и путь до файла открепленной подписи. Подпись должна быть для указанного файла xml" } };
        currAuthStep.step = nextStep;
    }

    function goToNickName() {
        let nextStep: AuthStep = { NickName: { text: "" } };
        currAuthStep.step = nextStep;
    }
</script>

<div class='w-100'>
	<p class="x-cont-c with-info">
		{currAuthStep.currentText}
	</p>
</div>


{#if currAuthStep.getPage}
    {@const ChosenPage = currAuthStep.getPage}
    <ChosenPage />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}


<div class='w-100'>
	<h6 class="x-self-c x-cont-c"> Навигация по авторизации </h6>
</div>


<section class="w-100">
	<div class="w-v50">
		<button
			class="but-pu"
			type="button"
			hidden={isPassword}
			onclick={goToPassword}
		>
			<span class='t-carry-cut'>
				Вход по паролю
			</span>
		</button>
	</div>
	
	<div class="w-v50">
		<button
			class="but-pu"
			type="button"
			hidden={isRegistrerStep1}
			onclick={goToRegisterStep1}
		>	
			<span class='t-carry-cut'>
				Регистрация шаг 1
			</span>
		</button>
	</div>
</section>

<section class="w-100">
	<div class="w-v50">
		<button
			class="but-pu"
			type="button"
			hidden={isRegistrerStep2}
			onclick={goToRegisterStep2}
		>
			<span class='t-carry-cut'>
				Регистрация шаг 2
			</span>
			
		</button>
	</div>
	
	<div class="w-v50">
		<button
			class="but-pu"
			type="button"
			hidden={isNickName}
			onclick={goToNickName}
		>
			<span class='t-carry-cut'>
				Войти как
			</span>
		</button>
	</div>
	
</section>
