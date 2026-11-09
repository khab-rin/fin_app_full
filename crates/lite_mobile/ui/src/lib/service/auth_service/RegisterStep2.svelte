<script lang='ts'>
    import {open as openFileDialog} from '@tauri-apps/plugin-dialog';
    import {invoke} from '@tauri-apps/api/core';
    import {currAuthStep} from '$lib/models/Auth/AuthStep.svelte';

    import type {AuthStep} from '$lib/models/rustModels/AuthStep';
    import type {RegFilesPathData} from '$lib/models/rustModels/RegFilesPathData';


    let isPushedRegister = $state(false);
    let isLoadJsonFile = $state(false);
    let isLoadSignFile = $state(false);

    let jsonFilePath = $state('');
    let signFilePath = $state('');

    let IsDataReady = $derived(
        jsonFilePath.length == 0 ||
        signFilePath.length == 0
    );


    async function getJsonFilePath() {
        if (isLoadJsonFile) return;
        isLoadJsonFile = true;
        try {
            const selected = await openFileDialog({
                multiple: false,
                directory: false,
                title: "Выберите ранее созданный json файл регистрации",
                filters: [{name: 'документ json', extensions: ['json']}]
            });

            if (selected) {
                if (typeof selected === 'string') {
                    jsonFilePath = selected
                } else {
                    isLoadJsonFile = false;
                }
            } else {
                isLoadJsonFile = false;
            }

        } catch (err) {
            console.error("GET FILE PATH FAILED, ERROR = ", err);
            const nextStep: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isLoadJsonFile = false;
            currAuthStep.step = nextStep;
        }
    }

    async function getSignFilePath() {
		if (isLoadSignFile) return;

		isLoadSignFile = true;

		try {
			const selected = await openFileDialog({
				multiple: false,
				directory: false,
				title: "Выберите отсоединенный файл подписи"
			});

			if (selected) {
				if (typeof selected === 'string') {
					signFilePath = selected;
					isLoadSignFile = false;
				} else {
					isLoadSignFile = false;
				}
			} else {
				isLoadSignFile = false;
			}

		} catch (err) {
			console.error("GET FILE PATH FAILED, ERROR = ", err);

			const nextStep: AuthStep = {
				TryLater: {
					text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"
				}
			};

			isLoadSignFile = false;
			currAuthStep.step = nextStep;
		}
	}

    async function register() {
        if (isPushedRegister) return;
        isPushedRegister = true;
        let data: RegFilesPathData = {
            jsonPath: jsonFilePath,
            signPath: signFilePath
        };
        try {
            const nextStep: AuthStep = await invoke<AuthStep>("cmd_register_step2", {
                data: data
            });

            isPushedRegister = false;
            currAuthStep.step = nextStep;
        } catch (err) {
            console.error("cmd_register FAILED, ERROR = ", err);
            const nextStep: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isPushedRegister = false;
            currAuthStep.step = nextStep;
        }
    }
</script>



<div class="w-100">
    <div class='w-v100'>
        <label class="label-close" for="jsonFilePath">
            Загрузите путь до json файла
        </label>
        <input
            type="text"
            id="jsonFilePath"
            value={jsonFilePath}
            class="input-gr"
			disabled={true}
        />
        <button
            type="button"
            id="xmlFileButton"
            class="but-gr"
            onclick={getJsonFilePath}
            disabled={isLoadJsonFile}
        >
            Загрузите xml файл
        </button>

    </div>
</div>

<div class="w-100">
    <div class='w-v100'>
        <label class="label-close" for="signFilePath">
            Загрузите путь до файла ЭЦП
        </label>
  
        <input
            type="text"
            id="signFilePath"
            value={signFilePath}
            class="input-gr"
			disabled={true}
        />
        <button
            type="button"
            id="sigFileButton"
            class="but-gr"
            onclick={getSignFilePath}
            disabled={isLoadSignFile}
        >
            Загрузите файл подписи
        </button>

    </div>
</div>


<div class="w-100">
    <div class='w-v100'>
		<button
			type="button"
			id='Register'
			class='but-gr'
			disabled={isPushedRegister || IsDataReady}
			onclick={register}
		>	
			{#if isPushedRegister}
				<span class="loading-content">
					<span class="loading-spinner"></span>
					Регистрация...
				</span>
			{:else}
				<span class='but-gr-span'>Отправить файлы на регистрацию</span>
			{/if}
		</button>
	</div>
</div>

