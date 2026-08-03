<script lang="ts">
  import { goTo } from '$lib/rules/navigation';

  import {pageManager} from '$lib/models/MainManager/MainManager.svelte';
  import { PageType } from "$lib/models/MainManager/PageValues";

  function loadPage(name: string) {
    console.log("Загрузка страницы:", name);
  }

  function goToOperation() {
    pageManager.Page = PageType.Operation;
  }


</script>

<!-- ВЕРХНИЙ БЛОК: Быстрые кнопки -->
<section class="grid-section">
  <div class="quick-links">
    <button class="grid-item" id="bank-statement-order">
      <span class="button-icon">📥</span>
      <span class="button-label">Загрузить банковскую выписку</span>
    </button>

    <button class="grid-item" id="payment-order-button">
      <span class="button-icon">📄</span>
      <span class="button-label">Сформировать платежное поручение</span>
    </button>

    <button class="grid-item" id="tax-report-button">
      <span class="button-icon">📊</span>
      <span class="button-label">Выгрузить отчет для налоговой</span>
    </button>

    <button class="grid-item empty">+</button>
    <button class="grid-item empty">+</button>
    <button class="grid-item empty">+</button>
  </div>
  <button class="small-settings-link">настроить быстрые кнопки</button>
</section>

<!-- НИЖНИЙ БЛОК: Основное меню -->
<section class="grid-section">
  <div class="grid-container">
    <button class="grid-item" onclick={() => goTo('/tax-reports')}>
      <span class="button-icon">🧾</span>
      <span class="button-label">Налоговые отчеты</span>
    </button>
    
    <button class="grid-item" onclick={() => goTo('/ingest')}>
      <span class="button-icon">📂</span>
      <span class="button-label">Загрузка документов</span>
    </button>

    <button class="grid-item" onclick={goToOperation}>
      <span class="button-icon">✍️</span>
      <span class="button-label">Работа с проводками</span>
    </button>

    <button class="grid-item" onclick={() => loadPage('upload-docs')}>
      <span class="button-icon">📈</span>
      <span class="button-label">Сформировать статистику</span>
    </button>
  </div>
</section>

<div id="status" class="status-message"></div>
