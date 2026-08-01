<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  async function handleBankImport() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'Выписка', extensions: ['csv', 'pdf', 'txt'] }]
      });

      if (selected) {
        await invoke('cmd_process_bank_statement', { path: selected });
        alert('Импорт завершен');
      }
    } catch (err) {
      alert('Ошибка импорта: ' + err);
    }
  }
</script>

<div class="ingest-container">
  <h2 class="section-title">Импорт данных</h2>
  
  <div class="ingest-list">
    <button class="ingest-row" onclick={handleBankImport}>
      <span class="label">Банковская выписка</span>
      <span class="action-button">Выбрать</span>
    </button>

    <button class="ingest-row">
      <span class="label">Отчет Яндекс Такси</span>
      <span class="action-button">Выбрать</span>
    </button>
  </div>
</div>

<style>
  .ingest-container { padding: 20px; font-family: sans-serif; }
  .section-title { margin-bottom: 20px; color: #333; }
  .ingest-list { display: flex; flex-direction: column; gap: 10px; }
  .ingest-row { 
    display: flex; 
    justify-content: space-between; 
    align-items: center;
    padding: 16px;
    background: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 8px;
    cursor: pointer;
    width: 100%;
    text-align: left;
  }
  .ingest-row:hover { background: #ebebeb; }
  .action-button { color: #0066cc; font-weight: 600; }
  .label { color: #444; }
</style>