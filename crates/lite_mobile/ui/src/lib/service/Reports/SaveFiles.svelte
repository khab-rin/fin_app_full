<script lang='ts'>
    import { save } from "@tauri-apps/plugin-dialog";
    import { writeFile } from "@tauri-apps/plugin-fs";
	import {onMount} from 'svelte';

    import {reportManager} from '$lib/models/Reports/ReportsManager.svelte'
	import {ReportType} from '$lib/models/Reports/ReportValues';
	import type {ReportStep} from '$lib/models/rustModels/ReportStep';


	let xmlName = $state<string | null>(null); 
	let xmlFile = $state<Uint8Array | null>(null);
	let pdfName = $state<string | null>(null); 
	let pdfFile = $state<Uint8Array | null>(null);



    let xmlPathSaved = $state("");
    let pdfPathSaved = $state("");

    let xmlSavePushed = $state(false)
    let pdfSavePushed = $state(false)

    async function save_xml_file() {
		if (xmlSavePushed) {return;}
		if (xmlFile == null || xmlName == null) {return;}
        try {
            xmlSavePushed = true;
            const docPath = await save({
                title: "Сохранить файл",
                defaultPath: xmlName,
                filters: [{name: "Документы xml", extensions: ["xml"]}]
            });

            if (docPath) {
                await writeFile(docPath, xmlFile);

                console.log("Документ успешно сохранен по пути:", docPath);

                xmlPathSaved = docPath;
                xmlSavePushed = false;

            } else {
                xmlSavePushed = false;
            }
        } catch(err) {
            console.error("XML FILE SAVING ERROR: ", err);
            xmlSavePushed = false;
            const nextStep: ReportStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            reportManager.step = nextStep;
        }
    }

    async function save_pdf_file() {
		if (pdfSavePushed) {return;}
		if (pdfFile == null || pdfName == null) {return;}
        try {
            pdfSavePushed = true;
            const docPath = await save({
                title: "Сохранить файл",
                defaultPath: pdfName,
                filters: [{name: "Документы pdf", extensions: ["pdf"]}]
            });

            if (docPath) {
                await writeFile(docPath, pdfFile);

                console.log("Документ успешно сохранен по пути:", docPath);
                pdfSavePushed = false;
				pdfPathSaved = docPath;

            } else {
                xmlSavePushed = false;
            }
        } catch(err) {
            console.error("XML FILE SAVING ERROR: ", err);
            xmlSavePushed = false;
            const nextStep: ReportStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            reportManager.step = nextStep;
        }
    }

	onMount(() => {
		if (ReportType.SaveFiles in reportManager.step) {
			xmlName = reportManager.step.SaveFiles.xml_name;
			xmlFile = new Uint8Array(reportManager.step.SaveFiles.xml_file);
			pdfName = reportManager.step.SaveFiles.pdf_name;
			pdfFile = new Uint8Array(reportManager.step.SaveFiles.pdf_file);
		} else {
			const nextStep: ReportStep = {TryLater: {text:'Критическая ошибка на устройстве...'}};
			console.error("LogicError in Reports save files");
			reportManager.step = nextStep;
		}
	}) 
</script>



<section class="group-one">
    <div>
        <label class="green-field-label" for="xlsFile">
            Сохранить XML файл
        </label>
  
        <input
            id="xlsFile"
            type="text"
            value={xmlPathSaved}
            class="green-field"
        />
        <button
            type="button"
            class="green-button"
            onclick={save_xml_file}
            disabled={xmlSavePushed}
            >
            Сохранить
        </button>

    </div>

    <div>
        <label class="green-field-label" for="xlsFile">
            Сохранить PDF файл
        </label>
  
        <input
            id="xlsFile"
            type="text"
            value={pdfPathSaved}
            class="green-field"
        />
        <button
            type="button"
            class="green-button"
            onclick={save_pdf_file}
            disabled={pdfSavePushed}
            >
            Сохранить
        </button>
    </div>
</section>
