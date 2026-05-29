<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

    // Массив для "чипсов" (быстрых кнопок)
    let savedNicks = $state(['Admin', 'Manager', 'Analyst']);

    // Поля для простого входа
    let nickname = $state('');

    // Состояние процесса и ошибок
    let isProcessing = $state(false);
    let errorMessage = $state('');

    // Поля регистрации — структура в точности повторяет SvelteRegistrationData из Rust!
    let reg = $state({
        // Данные физлица сгруппированы вложенным объектом
        person: {
            surname: '',
            name: '',
            last_name: '',
            inn: '',
            snils: ''
        },
        comp_inn: '',
        kpp: '',
        password: '',
        phone: '',
        email: '',
        // Эти пути заполнятся на этапе выгрузки/подписания, пока ставим пустые строки
        document_path: '',  
        signature_path: ''  
    });

    // Дополнительное поле для никнейма, которое мы пишем в стейт Tauri
    let reg_nick = $state('');

    // Функция входа (для чипсов и ручного ввода)
    async function handleLogin(targetNick: string) {
        if (!targetNick) return;
        isProcessing = true;
        errorMessage = '';
        try {
            // Вызов команды Rust для авторизации
            await invoke('auth_login', { nickname: targetNick });
            console.log('Вход выполнен:', targetNick);
        } catch (e) {
            errorMessage = 'Ошибка авторизации: ' + e;
        } finally {
            isProcessing = false;
        }
    }

    // Функция регистрации
    async function handleRegister(e: SubmitEvent) {
        e.preventDefault();
        isProcessing = true;
        errorMessage = '';
        try {
            // ВАЖНО: Передаем 'data' (структура) и 'nick' (строка) отдельно, 
            // как ожидает ваша функция в Rust
            await invoke('auth_register_user', { data: reg, nick: reg_nick });
            alert('Регистрация прошла успешно!');
        } catch (e) {
            errorMessage = 'Ошибка регистрации: ' + e;
        } finally {
            isProcessing = false;
        }
    }
</script>

<div class="auth-container">
    <!-- СЕКЦИЯ ВХОДА -->
    <section>
        <h2>Вход в систему</h2>
        <p>Быстрый выбор:</p>
        <div class="chips">
            {#each savedNicks as saved (saved)}
                <button 
                    type="button" 
                    onclick={() => handleLogin(saved)}
                    disabled={isProcessing}
                >
                    {saved}
                </button>
            {/each}
        </div>

        <form onsubmit={(e) => { e.preventDefault(); handleLogin(nickname); }}>
            <p>Или введите никнейм вручную:</p>
            <input 
                type="text" 
                bind:value={nickname} 
                placeholder="Никнейм..." 
                disabled={isProcessing}
                required 
            />
            
            <button type="submit" disabled={isProcessing || !nickname}>
                {isProcessing ? 'Проверка...' : 'Войти'}
            </button>
        </form>
    </section>

    <hr />

    <!-- СЕКЦИЯ РЕГИСТРАЦИИ -->
    <section>
        <h2>Регистрация нового пользователя</h2>
        <form onsubmit={handleRegister}>
            <div>
                <p><b>Аккаунт:</b></p>
                <!-- Привязываем никнейм к отдельной переменной reg_nick -->
                <input type="text" bind:value={reg_nick} placeholder="Придумайте никнейм" required />
                <input type="password" bind:value={reg.password} placeholder="Пароль" required />
            </div>

            <div>
                <p><b>Личные данные:</b></p>
                <!-- Обратите внимание на bind:value через точку: reg.person... -->
                <input type="text" bind:value={reg.person.surname} placeholder="Фамилия" required />
                <input type="text" bind:value={reg.person.name} placeholder="Имя" required />
                <input type="text" bind:value={reg.person.last_name} placeholder="Отчество" />
            </div>

            <div>
                <p><b>Документы:</b></p>
                <input type="text" bind:value={reg.person.inn} placeholder="Ваш ИНН" required />
                <input type="text" bind:value={reg.person.snils} placeholder="Ваш СНИЛС" required />
            </div>

            <div>
                <p><b>Организация:</b></p>
                <input type="text" bind:value={reg.comp_inn} placeholder="ИНН Организации" required />
                <input type="text" bind:value={reg.kpp} placeholder="КПП" required />
            </div>

            <div>
                <p><b>Связь:</b></p>
                <input type="tel" bind:value={reg.phone} placeholder="Телефон" required />
                <input type="email" bind:value={reg.email} placeholder="E-mail" required />
            </div>

            <br />
            <button type="submit" disabled={isProcessing}>
                {isProcessing ? 'Регистрация...' : 'Зарегистрировать профиль'}
            </button>
        </form>
    </section>

    <!-- ОШИБКИ -->
    {#if errorMessage}
        <p style="color: red; font-weight: bold; margin-top: 20px;">
            {errorMessage}
        </p>
    {/if}
</div>
