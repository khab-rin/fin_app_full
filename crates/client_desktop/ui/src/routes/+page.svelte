<script lang="ts">
  import { goTo } from '$lib/rules/navigation';

  function loadPage(name: string) {
    console.log("Загрузка страницы:", name);
  }
</script>

<svelte:head>
  <link href="https://googleapis.com" rel="stylesheet">
</svelte:head>

<!-- ВЕРХНИЙ БЛОК: Быстрые кнопки -->
<section class="grid-section">
  <div class="quick-links">
    <button class="grid-item" id="bank-statement-order">
      <span class="btn-icon">📥</span>
      <span class="btn-label">Загрузить банковскую выписку</span>
    </button>

    <button class="grid-item" id="payment-order-btn">
      <span class="btn-icon">📄</span>
      <span class="btn-label">Сформировать платежное поручение</span>
    </button>

    <button class="grid-item" id="tax-report-btn">
      <span class="btn-icon">📊</span>
      <span class="btn-label">Выгрузить отчет для налоговой</span>
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
      <span class="btn-icon">🧾</span>
      <span class="btn-label">Налоговые отчеты</span>
    </button>
    
    <button class="grid-item" onclick={() => goTo('/ingest')}>
      <span class="btn-icon">📂</span>
      <span class="btn-label">Загрузка документов</span>
    </button>

    <button class="grid-item" onclick={() => loadPage('upload-docs')}>
      <span class="btn-icon">✍️</span>
      <span class="btn-label">Подписать документ</span>
    </button>

    <button class="grid-item" onclick={() => loadPage('upload-docs')}>
      <span class="btn-icon">📈</span>
      <span class="btn-label">Сформировать статистику</span>
    </button>
  </div>
</section>

<div id="status" class="status-message"></div>
