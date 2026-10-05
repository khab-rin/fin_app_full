<script lang='ts'>
    import {open as openFileDialog} from '@tauri-apps/plugin-dialog';
    import {invoke} from '@tauri-apps/api/core';
	import {pageManager} from '$lib/models/MainManager/MainManager.svelte';
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
                title: "Выберите отсоединенный файл подписи",
                filters: [{name: 'документ подписи', extensions: ['sig', 'p7s']}]
            });

            if (selected) {
                if (typeof selected === 'string') {
                    signFilePath = selected
                } else {
                    isLoadSignFile = false;
                }
            } else {
                isLoadSignFile = false;
            }

        } catch (err) {
            console.error("GET FILE PATH FAILED, ERROR = ", err);
            const nextStep: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
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
			pageManager.compName = await invoke<string>('cmd_get_comp_name', {});
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



<section class="w-40 x-self-l">
    <div class='w-v100'>
        <label class="label-close" for="AuthStep2jsonFilePath">
            Загрузите путь до json файла
        </label>
        <input
            type="text"
            id="AuthStep2jsonFilePath"
            value={jsonFilePath}
            class="input-gr"
			disabled={true}
        />
        <button
            type="button"
            id="AuthStep2xmlFileButton"
            class="but-gr"
            onclick={getJsonFilePath}
            disabled={isLoadJsonFile}
        >
			<span class="x-self-c">
				Загрузите json файл
			</span>
            
        </button>

    </div>
</section>

<section class='w-40 x-self-l'>
    <div class='w-v100'>
        <label class="label-close" for="AuthStep2signFilePath">
            Загрузите путь до файла ЭЦП
        </label>
  
        <input
            type="text"
            id="AuthStep2signFilePath"
            value={signFilePath}
            class="input-gr"
			disabled={true}
        />
        <button
            type="button"
            id="AuthStep2sigFileButton"
            class="but-gr"
            onclick={getSignFilePath}
            disabled={isLoadSignFile}
        >
			<span class="x-self-c">
				Загрузите файл подписи
			</span> 
        </button>

    </div>
</section>

<section class='w-40 x-self-l'>
	<div class='w-v100'>
		<button
			type="button"
			id='AuthButRegister'
			class='but-gr'
			disabled={isPushedRegister || IsDataReady}
			onclick={register}

		>
			<span class='x-self-c'>Отправить файлы на регистрацию</span>
		</button>
	</div>
</section>

