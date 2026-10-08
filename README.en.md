# 🏦 B2B Fintech Platform: Monorepository for accounting and document flow

## 🚀 Implemented Business Features
### 1. Graphical Data Validation
* **Dynamic Input Control (UX/UI Validation):**
  * Real-time validation for INN, KPP, and dates right inside the user interface `[Client]`.
  * Interactive highlighting for fields with errors. Blocks form sending so user cannot input wrong data until they fix it `[Client]`.

### 2. Registration and Authentication Service
* **Step-by-step Enterprise Registration (Data Flow):**
  * Collects user and company data to create a single business profile. `[Client]`
  * Generates a registration application in printable view for user and in JSON format for next steps. `[Client]`
  * Receives detached Electronic Digital Signature (EDS) from user for the JSON file. `[Server]`
  * Extracts data from EDS public key, validates signature and checks it with source JSON data. `[Server]`
  * Hashes passwords using Argon2 algorithm. `[Server]`
  * Creates user and enterprise records in PostgreSQL database. `[Server]`
  * Generates login session and creates secure connection link `Token — DeviceId`. `[Server]`
* **Login and Access Recovery:**
  * Fast login using valid session token saved in secure device storage. `[Client / Server]`
  * Login via strict B2B fields: (User INN, Company INN, Company KPP) + password. `[Client / Server]`
  * 2FA access recovery via Flash Call (user enters last digits from incoming phone call). `[Client / Server]`

### 3. Machine-Readable Power of Attorney (MCHD) Service
* Simple UI and code interface for managing user access rights and roles. `[Client / Server]`
* Contains permissions for EDO (Electronic Document Interchange) with FNS RF and B2B EDO from official FNS list. `[Client / Server]`
* Supports custom (internal) permissions for flexible access inside this app. `[Client / Server]`
* **Creating new MCHD:**
  * Collects data of principal, representative, MCHD info, and selected rights `[Client]`.
  * Automatically generates MCHD in actual XML format approved by FNS RF `[Client]`.
  * Receives detached electronic signature of company director for the XML file `[Client]`.
  * Extracts data from director's EDS, validates signature owner and XML data. `[Server]`
  * Saves verified MCHD in server storage. `[Server]`
* Access control module that checks user rights in active MCHD database when user tries to do operations. `[Server]`
* Graphical interface to view current active rights of the user. `[Client]`

### 4. Accounting and Tax Reporting Service (RSBU / RAS Core)
* **Real-time Double-Entry Engine:**
  * Core logic that uses classic double-entry rule to show business operations across main accounts of Russian Accounting Standards (RSBU).
  * Supports active and passive accounts.
* **Automation and primary data entry:**
  * Screen for manual input and editing of complex accounting entries. `[Client]`
  * Smart bank statement parsing: automatically reads transactions, makes draft entries, and user can check or edit them before saving. `[Client]`
* **Tax Reporting Generation:**
  * Calculates tax base for small business using USN Income (УСН Доходы) and USN Income minus Expenses (УСН Доходы минус Расходы).
  * Generates official tax declarations and notifications in two formats: `[Client]`
    * **PDF format** — printable forms that exactly match official FNS layouts for paper delivery. `[Client]`
    * **XML format** — valid files for electronic document flow, ready to send to FNS via TKS operators. `[Client]`



## 🛠 Tech Stack and Key Libraries
The project is built on a modern asynchronous **Rust (Workspace)** stack.

* **Client Core:** **Tauri (v2.10)** — a lightweight framework for building native apps for PC and mobile platforms / TypeScript.
* **Server Core:** **Axum (v0.7)** — a high-performance async web framework for the backend REST API (`back_api`).
* **Custom `shared_lib` library:** Syncs common custom data structures.
  * **TS-RS:** Automatically generates TypeScript interfaces from Rust structures during compilation, giving 100% type safety on the frontend.
* **Async Runtime:** **Tokio & Futures** — runtime engine for efficient handling of concurrent async tasks.
### 🔹 Cryptography and Security
* **Argon2 & Password-Hash:** Modern crypto-strong hashing algorithm for secure user password storage.
* **Blake3:** Super-fast cryptographic hash function to check document file integrity.
* **Keyring:** Safe interface to work with OS system password storages (Secret Service / Keychain) on the client side.
* **Machine-UID:** Extracts a unique hardware ID to validate the `Token — DeviceId` binding.
### 🔹 Containerization and Infrastructure (Docker)
For local deployment and service isolation, we use **two Docker containers** orchestrated via Docker Compose:
* **PostgreSQL DBMS Container:** Storage for main system data, including company profiles, sessions, and RSBU journal entries.
* **Crypto-service Container (`crypto_service`):** Isolated environment for sensitive crypto operations and EDS validation, protecting the main app from infrastructure risks.
### 🔹 Specific Data Types
* **Rust_Decimal:** High-precision financial calculations in the accounting core (avoids float-type rounding errors).
* **Chrono:** Works with dates, time, and timezones (critical for MCHD and accounting periods).
* **Uuid:** Generates unique entity IDs.
* **Vec1:** Guarantees at least one element in collections at the system type level.
### 🔹 Databases
* **SQLx:** Async driver with compile-time SQL query validation and built-in migrations (PostgreSQL for the backend and SQLite for the clients' local cache).
### 🔹 Monitoring, Logging, and Utilities
* **Tracing (Appender, Subscriber):** Structured async logging for microservices with file rotation support.
* **Tower-Http:** Middleware for Axum (request logging, tracing, header validation).
* **Tauri Plugins (v2):** Official plugins from the Tauri ecosystem for secure access to OS resources (`dialog`, `shell`, `fs`, `log`).


## 🧩 Architecture Blocks (Crates Description)
### 📦 `shared_lib` (Core Shared Library)
The central component of the monorepository. It provides type safety across the whole app, lets us reuse business logic, and keeps data processing standards identical between the server and cross-platform clients.

Internal structure of the module:
* **`client` (Client business logic):** 
  * Contains a universal set of functions and helpers that remain the same for different client platforms (PC and Mobile) and different legal forms of enterprises.
  * Isolated from compilation inside the server crate `back_api`.
* **`err_models` (Unified error handling subsystem):**
  * Integration with logging layers: `tracing` for the server part and `log` for the client part.
  * Keeps Rust code clean and provides detailed logs to find root causes of crashes.
* **`primitives` (Domain-specific types / Value Objects):**
  * Custom newtype wrappers over basic types with built-in data validation (INN, KPP, SNILS).
  * Integration and seamless mapping into PostgreSQL (server) and SQLite (client) ensure data correctness and integrity, including inside DB tables.
* **`sql_models` (Data Mapping layer):**
  * Mirror structures that exactly match the database schemas. 
  * Contain helper data structures used in different parts of the application.
* **`static_data` (Formats and templates registry):**
  * Text templates, schemas, and metadata needed for document parsing and building bridges to strict XML structures of FNS RF MCHD.
* **`service` (DTO and Network contracts):**
  * Data Transfer Objects describing how program parts interact, grouped by functional modules.
  * Single registry of API routes that guarantees strict synchronization of request URLs on the client side and correct routing by keys on the Axum server side.

### 📦 `back_api` (Server Side)
Service for storing, accessing, and syncing global data: companies, individuals, users, connections, contracts, and accounting entries. The central component is a Docker container running PostgreSQL.

Internal structure of the module:
* **`migrations`:** SQL files for automatic database schema deployment and updates when the server starts.
* **`handlers`:** List of network request handlers (controllers) grouped by features. They accept and return data in JSON format.
* **`parser`:** Module for calling external APIs (used to automatically auto-fill company details via **DaData.ru**).
* **`service`:** Main service logic — processes client requests and builds response data structures. The structure mirrors the features (auth, MCHD creation, etc.).
* **`sql_queries`:** Database query functions, neatly separated by table names.

### 🔐 `crypto_service` (Cryptographic Microservice)
An isolated service located on the same physical server as the backend, ensuring maximum data exchange speed. Its only job is to receive a crypto signature file, extract owner data (Full Name, SNILS), and validate authenticity. The central component is a Docker container with CryptoPro CSP installed.

**Why it is separated into a microservice:**
* **Clean environment:** CryptoPro requires specific OS dependencies and the installation of official root certificates (`russian_trusted_root_ca.cer`). Moving this to a "sandbox" keeps the main `back_api` image clean.
* **Stability and security:** A crash during signature check or a failure in the CryptoPro utility will stay inside this container and won't affect the main app server.

**Key components of the crate:**
* `src/person_snils/` — logic for extracting and validating data from the EDS public key certificate.
* `DockerFile` + CryptoPro distribution package (`linux-amd64_deb.tgz`) — automatic build script for the isolated crypto environment.

### 💻📱 `lite_pc` & `lite_mobile` (Client Apps)
Built with **Tauri (Rust)** core + **Svelte 5 (TypeScript / Vite)**. Two separate crates describe the interface (HTML and CSS) for desktop screens and mobile phones. The rest of the logic is shared.

**1. Native Part (Rust / `src-tauri`):**
* **`commands`:** List of functions (commands) that the frontend calls from the UI. Platform-specific requests are handled inside the current crate, while universal requests (for different business ownership types) are handled by `shared_lib` functions.
* **`service`:** Internal application services (auth, local data storage in SQLite, device key verification) grouped by feature.

**2. UI Part (TypeScript / `ui/src`):**
* **`models/` (`.svelte.ts`):** Global application state manager and managers for separate microservices based on Svelte 5. Also stores automatically generated TypeScript bindings for data structures from `shared_lib`.
* **`service/` (`.svelte`):** A set of visual components and interfaces separated strictly by business tasks: auth forms, FNS MCHD constructor, accounting entry journals, and tax report download dashboards.
* **`routes/`:** Standard file-based routing (`+layout` and `+page` entry points). In practice, only the `+layout` file is used to dynamically render the needed modules and views from the `service` folder.
* **`style/`:** Modular CSS styles separated into individual files for every UI element (buttons, inputs, dialogs) for easy UI customization.


## 🏛 Key Architectural Decisions
### 1. State Management via Finite State Machines (Rust Enums)
* **Unified State Pattern:** Each functional service has a central data structure based on an `enum` (for example, `AuthStep`). Each element describes a specific step of business logic: `Loading`, `RegisterStep1`, `RegisterStep2`, `Success`, `TryLater`.
* **Strict and Predictable Data Flow:** The client clicks the "Submit" button and gets back a data structure with the processing result needed for the next step, or `TryLater` if something fails. This links the whole application chain *(Screen → Client Device → Server)* into a single, strict data exchange format. The compiler guarantees that the UI can never end up in an undefined state. All potential errors in root functions are safely handled by the system, switching the app into a secure and user-friendly standby mode (`TryLater`). The `ts-rs` library handles seamless processing of any Rust scenario results on the Svelte 5 frontend side.

### 2. End-to-End Compatibility and Safe Refactoring
* **Centralized Dependencies:** All libraries and their versions are declared only once in the root `Cargo.toml`. Crates just import them when needed. This guarantees full compatibility of libraries across the whole project, prevents version conflicts, and enables end-to-end code validation by the compiler.
* **Single Source of Truth:** All custom data structures are declared exclusively inside the `shared_lib` library. This allows safe code modifications and refactoring: changes in one place are instantly applied to the entire system.
* **Automatic Type Synchronization:** Using the `ts-rs` library to generate TypeScript interfaces directly from Rust structures guarantees full data compatibility between TypeScript and Rust.

### 3. Access Architecture Based on MCHD (Zero-Trust Concept)
* **Dynamic Permissions:** The system has absolutely no hardcoded, static roles for users (like "admin" or "accountant") inside their account profiles.
* **FNS Standards Compliance:** A user logs in as an individual using their personal EDS (electronic signature), and all access rights are calculated dynamically based on uploaded XML files of Machine-Readable Powers of Attorney (MCHD) signed by the company director.

### 4. Frontend Architectural Solutions (Svelte 5)
* **Single Entry Point:** The standard SvelteKit file-based router is used only in one place (`+layout.svelte`), while all other interfaces (services) are rendered dynamically as components. This removes UI freezes and minimizes RAM consumption by the device.
* **Separation of State Managers:** The architecture uses a Global Manager working with isolated managers for specific microservices. The Global Manager switches internal reactive variables to determine which service should be rendered right now. This ensures strict data isolation and prevents any cyclic dependencies in Svelte reactive variables.
