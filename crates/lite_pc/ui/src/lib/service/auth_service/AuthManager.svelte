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


<div class="w-100">
	<h4 class='w-g100 x-self-c x-ce t-fill'>
		{currAuthStep.currentText}
	</h4>
</div>



{#if currAuthStep.getPage}
    {@const ChosenPage = currAuthStep.getPage}
    <ChosenPage />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}


<div class='w-40 x-self-l'>
	<h5 class='w-g100 t-fill x-ce'>
		Навигация по авторизации 
	</h5>
</div>



<section class="w-40">
	<div  class='w-g50'>
		<button
			class="but-pu"
			type="button"
			hidden={isPassword}
			onclick={goToPassword}
		>	
			<span class='w-g100 x-ce'>
				Вход по паролю
			</span>
			
		</button>
	</div>
	
	<div  class='w-g50'>
		<button
			class="but-pu"
			type="button"
			hidden={isRegistrerStep1}
			onclick={goToRegisterStep1}
		>	
			<span class='w-g100 x-ce'>
				Регистрация шаг 1
			</span>
		</button>
	</div>
</section>

<section class="w-40">
	<div class='w-v50'>
		<button
			class="but-pu"
			type="button"
			hidden={isRegistrerStep2}
			onclick={goToRegisterStep2}
		>
			<span class='w-g100 x-ce'>
				Регистрация шаг 2
			</span>
		</button>
	</div>
	
	<div class='w-v50'>
		<button
			class="but-pu"
			type="button"
			hidden={isNickName}
			onclick={goToNickName}
		>
			<span class='w-g100 x-ce'>
				Войти как
			</span>
		</button>
	</div>
	
</section>

<!-- <section class='w-g100'>
	<div class='w-v50'>
		<div class='row-20'>
			<button 
				class='but-gr'
				type='button'
			>
				<span class='span-fill span-l span-u'>
					example
				</span>

			</button>
		</div>
		<div class='row-20'>
			<button 
				class='but-gr'
				type='button'
			>
				<span class='span-fill span-l span-u'>
					example
				</span>

			</button>
		</div>
		<div class='row-20'>
			<button 
				class='but-gr'
				type='button'
			>
				<span class='span-fill span-l span-u'>
					example
				</span>

			</button>
		</div>
		<div class='row-20'>
			<button 
				class='but-gr'
				type='button'
			>
				<span class='span-fill span-l span-u'>
					example
				</span>

			</button>
		</div>
		<div class='row-20'>
			<button 
				class='but-gr'
				type='button'
			>
				<span class='span-fill span-l span-u'>
					example
				</span>

			</button>
		</div>
	</div>

	<div class='w-v50'>
		<div class='w-v50'>
			<button 
				class='but-gr'
				type='button'
			>
				<span class='span-fill span-l span-u'>
					example
				</span>

			</button>
		</div>
		<div class='w-v50'>
			<button 
				class='but-gr'
				type='button'
			>
				<span class='span-fill span-l span-u'>
					example
				</span>

			</button>
		</div>
	
	</div>
</section> -->
