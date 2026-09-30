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

<p class="text-small">
    {currAuthStep.currentText}
</p>

{#if currAuthStep.getPage}
    {@const ChosenPage = currAuthStep.getPage}
    <ChosenPage />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}

<h6> Навигация по авторизации </h6>
<section class="group-two">

	<div>
		<button
			class="purple-button"
			type="button"
			hidden={isPassword}
			onclick={goToPassword}
		>
			Вход по паролю
		</button>
	</div>
	
	<div>
		<button
			class="purple-button"
			type="button"
			hidden={isRegistrerStep1}
			onclick={goToRegisterStep1}
		>
			Регистрация шаг 1
		</button>
	</div>
	
	<div>
		<button
			class="purple-button"
			type="button"
			hidden={isRegistrerStep2}
			onclick={goToRegisterStep2}
		>
			Регистрация шаг 2
		</button>
	</div>
	
	<div>
		<button
			class="purple-button"
			type="button"
			hidden={isNickName}
			onclick={goToNickName}
		>
			Войти как
		</button>
	</div>
	
</section>
