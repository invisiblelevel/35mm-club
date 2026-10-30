# 35mm Club

**Your cinema. Your rules.** · **Твоя кинотека. Твои правила.** · **你的影院。你的规则。**

[![Version](https://img.shields.io/badge/version-1.3.0-blue.svg)](../../releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11%20%7C%20Android%208%2B-0078D6.svg)](#install)
[![Tauri](https://img.shields.io/badge/tauri-2.12-24C8DB.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev/)
[![Languages](https://img.shields.io/badge/languages-EN%20%7C%20RU%20%7C%20ZH-orange.svg)](#languages)

[English](#english) · [Русский](#русский) · [中文](#中文)

---

## English

35mm Club is a local-first tracker for movies, TV shows and anime with achievements, XP, ranks and a military-style ribbon rack. All data lives in a single SQLite file on your device — no clouds, no accounts, no data collection.

Add a title → track episodes → earn achievements → level up. Everything happens on your machine, nothing is sent anywhere except the public metadata APIs (TMDB, Shikimori, AniList, Bangumi).

**Available on Windows and Android.**

### What's new in 1.3.0

- **Roulette** — a new "What to watch?" wheel. Random movies, TV shows or anime from TMDB / Shikimori / Bangumi. 8 random titles spin on a 3D wheel with posters, sparks, sound and confetti. Add the winner to your library in one click.
- **Bangumi for China** — anime roulette on the Chinese locale now works via the Bangumi v0 API, even through Cloudflare, using a browser-style User-Agent.

### Features

- **Three content types** — movies, TV shows, anime
- **Roulette** — spin a wheel to pick what to watch tonight
- **Episode tracker** — seasons, episodes, watch progress
- **300+ achievements** — Common, Rare, Epic, Legendary, with custom icons and descriptions
- **Ranks and XP** — 10 military-style ranks from Private to General
- **Ribbon rack (kokarda)** — unlocks at level 20 (Major) and grows with level and achievements
- **Statistics** — overview, status donut, combined donut (ratings / months / weekdays), top-rated and most-rewatched
- **Wi-Fi sync** — one button on any device in the same network; syncs library, achievements, diary, notes and collections between your PC and phone. Pair once, then silent. No clouds, no accounts.
- **Rewatch Tracker** — log rewatches with rating and notes, see how many times you've watched each title. Compact list, no clutter.
- **Diary** — watch history with dates, ratings and notes, plus a dedicated **Rewatches** tab with all your rewatches across the library
- **Custom collections** — your own lists with posters preview
- **Ratings** — half-star precision (0.5–10.0) for movies, TV and anime
- **Anime progress** — episode counter with auto-complete detection
- **Backup & restore** — export the entire library, achievements, diary and collections to a single JSON file; import it on another device
- **Three languages** — English, Russian, Simplified Chinese, with system auto-detection
- **Dark / light theme** — dark by default, switchable in Settings
- **Interface zoom** — 100% / 125% / 150% / 200%
- **Fully offline** — works without internet after the first metadata fetch

### Tech Stack

Tauri 2.12 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · SQLite (bundled) · TMDB · Shikimori · AniList · Bangumi (zh-CN) · canvas-confetti · mDNS · Ed25519

### Install

**Windows:** Download **35mm Club Setup v1.3.0.exe** from [Releases](../../releases), run, install (choose English / Russian / Chinese). No dependencies needed — everything is bundled, including WebView2 runtime.

**Android:** Download **35mm Club v1.3.0.apk** from [Releases](../../releases), allow installation from unknown sources, install.

Windows 10 version 1903 or newer / Windows 11 (64-bit) · Android 8.0 or newer.

> **Note:** On first launch, the app needs internet access to fetch metadata from TMDB and Shikimori. After that, everything (library, achievements, diary) works fully offline.

### Build from source

Requirements:

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.75+
- Windows 10/11 with WebView2, or Android SDK + NDK

Steps:

1. `git clone https://github.com/invisiblelevel/35mm-club.git`
2. `cd 35mm-club`
3. `cp .env.example .env` and insert your TMDB API key
4. `npm install`
5. `npm run tauri dev` — dev mode (desktop)
6. `npm run tauri build` — release build (Windows)
7. `npx tauri android build --apk` — release build (Android APK)

The Windows installer will appear in `src-tauri/target/release/bundle/nsis/`. The Android APK will appear in `src-tauri/gen/android/app/build/outputs/apk/universal/release/`.

### Data location

- **Windows:** `%APPDATA%\com.invisiblelevel.thirtyfivemmclub\35mm_club.db`
- **Android:** internal app storage (use **Settings → Backup → Export data** to save).

Back it up before updating. Use **Settings → Backup → Export** for a portable JSON backup.

> **Android note:** Auto Backup is disabled (`android:allowBackup="false"`). Uninstalling removes the database.

### License

MIT — see [LICENSE](LICENSE).

### Author

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

---

## Русский

35mm Club — локальный трекер фильмов, сериалов и аниме с ачивками, XP, званиями и генеральской кокардой. Все данные лежат в одном файле SQLite на твоём устройстве — без облаков, аккаунтов и сбора данных.

Добавил тайтл → отметил серии → получил ачивку → поднял уровень. Всё происходит на твоей машине, наружу уходят только запросы к публичным API метаданных (TMDB, Shikimori, AniList, Bangumi).

**Доступно на Windows и Android.**

### Что нового в 1.3.0

- **Рулетка** — новый колесо-аттракцион «Что посмотреть?». Случайные фильмы, сериалы или аниме из TMDB / Shikimori / Bangumi. 8 случайных тайтлов крутятся на колесе с постерами, искрами, звуком и салютом. Победителя можно добавить в библиотеку в один клик.
- **Bangumi для Китая** — рулетка аниме на китайской локали теперь работает через Bangumi v0 API даже через Cloudflare, благодаря браузерному User-Agent.

### Что умеет

- **Три типа контента** — фильмы, сериалы, аниме
- **Рулетка** — крути колесо и выбирай, что смотреть сегодня
- **Трекер серий** — сезоны, эпизоды, прогресс просмотра
- **300+ ачивок** — Common, Rare, Epic, Legendary, с кастомными иконками и описаниями
- **Звания и XP** — 10 военных званий от Рядового до Генерала
- **Кокарда (планки)** — открывается на 20 уровне (Майор) и растёт вместе с уровнем и ачивками
- **Статистика** — обзор, круговая диаграмма статусов, комбинированный донат (рейтинги / месяцы / дни недели), топы по оценке и пересмотрам
- **Синхронизация по Wi-Fi** — одна кнопка на любом устройстве в одной сети; синкает библиотеку, ачивки, дневник, заметки и коллекции между ПК и телефоном. Один раз подтвердил — дальше молча. Без облаков и аккаунтов.
- **Пересмотры** — записывай пересмотры с оценкой и заметкой, смотри, сколько раз ты пересматривал каждый тайтл. Компактный список, без простыней.
- **Дневник** — история просмотров с датами, оценками и заметками + отдельная вкладка **«Пересмотры»** со всеми пересмотрами по библиотеке
- **Кастомные коллекции** — свои списки с превью постеров
- **Рейтинги** — с точностью до половины звезды (0.5–10.0) для фильмов, сериалов и аниме
- **Прогресс аниме** — счётчик серий с авто-определением завершения
- **Бэкап и восстановление** — экспорт всей библиотеки, ачивок, дневника и коллекций в один JSON-файл; импорт на другом устройстве
- **Три языка** — английский, русский, китайский (упрощённый), с авто-определением системного
- **Тёмная / светлая тема** — тёмная по умолчанию, переключается в настройках
- **Зум интерфейса** — 100% / 125% / 150% / 200%
- **Полностью оффлайн** — работает без интернета после первой загрузки метаданных

### Стек

Tauri 2.12 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · SQLite (bundled) · TMDB · Shikimori · AniList · Bangumi (zh-CN) · canvas-confetti · mDNS · Ed25519

### Установка

**Windows:** Скачай **35mm Club Setup v1.3.0.exe** из раздела [Releases](../../releases), запусти, установи (выбери язык: русский / английский / китайский). Зависимости не нужны — всё вшито в инсталлер, включая WebView2.

**Android:** Скачай **35mm Club v1.3.0.apk** из раздела [Releases](../../releases), разреши установку из неизвестных источников, установи.

Поддерживается Windows 10 (версия 1903+) / Windows 11 (64-bit) · Android 8.0+.

> **Важно:** При первом запуске нужен интернет, чтобы загрузить метаданные из TMDB и Shikimori. После этого всё (библиотека, ачивки, дневник) работает полностью оффлайн.

### Сборка из исходников

Требования:

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.75+
- Windows 10/11 с WebView2, или Android SDK + NDK

Шаги:

1. `git clone https://github.com/invisiblelevel/35mm-club.git`
2. `cd 35mm-club`
3. `cp .env.example .env` и вставь свой TMDB API-ключ
4. `npm install`
5. `npm run tauri dev` — dev-режим (десктоп)
6. `npm run tauri build` — релиз (Windows)
7. `npx tauri android build --apk` — релиз (Android APK)

Windows-установщик появится в `src-tauri/target/release/bundle/nsis/`. Android APK — в `src-tauri/gen/android/app/build/outputs/apk/universal/release/`.

### Где лежат данные

- **Windows:** `%APPDATA%\com.invisiblelevel.thirtyfivemmclub\35mm_club.db`
- **Android:** внутренняя папка приложения (используй **Настройки → Резервная копия → Экспорт данных**).

Сделай бэкап перед обновлением. Для переноса между устройствами — **Настройки → Резервная копия → Экспорт**.

> **Android:** Auto Backup отключён (`android:allowBackup="false"`). При удалении приложения база стирается.

### Лицензия

MIT — см. [LICENSE](LICENSE).

### Автор

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

---

## 中文

35mm Club 是一款本地优先的电影、剧集和动漫追踪器，带有成就、经验值、军衔和将军式勋表。所有数据存储在你设备上的单个 SQLite 文件中——无云端、无账号、无数据收集。

添加条目 → 记录剧集 → 获得成就 → 升级。所有操作都在本机完成，只有元数据请求会发送到公开 API（TMDB、Shikimori、AniList、Bangumi）。

**支持 Windows 和 Android。**

### 1.3.0 新功能

- **轮盘** — 全新的「今晚看什么？」转盘。从 TMDB / Shikimori / Bangumi 随机抽取电影、剧集或动漫。8 部随机作品在海报转盘上旋转，伴随火花、音效和彩带。一键将获胜者加入媒体库。
- **中国的 Bangumi** — 中文界面的动漫轮盘现在通过 Bangumi v0 API 工作，使用浏览器风格的 User-Agent 绕过 Cloudflare。

### 功能

- **三种内容类型** — 电影、剧集、动漫
- **轮盘** — 转动转盘，选择今晚看什么
- **剧集追踪** — 季、集、观看进度
- **300+ 个成就** — 普通、稀有、史诗、传奇，自定义图标和描述
- **军衔与经验值** — 10 个军事风格军衔，从列兵到将军
- **勋表（кокарда）** — 20 级（少校）解锁，随等级和成就成长
- **统计** — 概览、状态环形图、组合环形图（评分 / 月份 / 星期几）、最高评分与最多重看排行
- **Wi-Fi 同步** — 在同一网络的任意设备上按一个按钮；在 PC 和手机之间同步媒体库、成就、日记、笔记和收藏集。配对一次，之后静默运行。无云端、无账号。
- **重看追踪** — 记录重看评分和笔记，查看每部作品的重看次数。紧凑列表，不杂乱。
- **观影日记** — 带日期、评分和笔记的观看历史，另有独立的 **「重看」** 标签页
- **自定义收藏集** — 自己的列表，带海报预览
- **评分** — 半星精度（0.5–10.0），适用于电影、剧集和动漫
- **动漫进度** — 剧集计数器，自动检测完成
- **备份与恢复** — 将整个媒体库、成就、日记和收藏集导出为单个 JSON 文件；可在另一台设备上导入
- **三种语言** — 英语、俄语、简体中文，支持系统自动检测
- **深色 / 浅色主题** — 默认深色，可在设置中切换
- **界面缩放** — 100% / 125% / 150% / 200%
- **完全离线** — 首次获取元数据后无需网络即可运行

### 技术栈

Tauri 2.12 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · SQLite (bundled) · TMDB · Shikimori · AniList · Bangumi (zh-CN) · canvas-confetti · mDNS · Ed25519

### 安装

**Windows:** 从 [Releases](../../releases) 下载 **35mm Club Setup v1.3.0.exe**，运行并安装（可选英语 / 俄语 / 中文）。无需任何依赖 — 全部已打包，含 WebView2 运行时。

**Android:** 从 [Releases](../../releases) 下载 **35mm Club v1.3.0.apk**，允许从未知来源安装，然后安装。

支持 Windows 10（1903 或更新版本）/ Windows 11（64 位）· Android 8.0+。

> **重要：** 首次启动需要网络连接以从 TMDB 和 Shikimori 获取元数据。之后所有内容（媒体库、成就、日记）均可完全离线使用。

### 从源码构建

环境要求：

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.75+
- Windows 10/11 且已安装 WebView2，或 Android SDK + NDK

步骤：

1. `git clone https://github.com/invisiblelevel/35mm-club.git`
2. `cd 35mm-club`
3. `cp .env.example .env` 并填入你的 TMDB API 密钥
4. `npm install`
5. `npm run tauri dev` — 开发模式（桌面）
6. `npm run tauri build` — 发布构建（Windows）
7. `npx tauri android build --apk` — 发布构建（Android APK）

Windows 安装包将生成于 `src-tauri/target/release/bundle/nsis/`。Android APK 将生成于 `src-tauri/gen/android/app/build/outputs/apk/universal/release/`。

### 数据位置

- **Windows:** `%APPDATA%\com.invisiblelevel.thirtyfivemmclub\35mm_club.db`
- **Android:** 应用内部存储（使用 **设置 → 备份 → 导出数据**）。

更新前请备份。跨设备迁移 — **设置 → 备份 → 导出**。

> **Android:** 已禁用 Auto Backup（`android:allowBackup="false"`）。卸载应用会删除数据库。

### 许可证

MIT — 见 [LICENSE](LICENSE)。

### 作者

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)