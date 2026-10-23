# 35mm Club

**Your cinema. Your rules.** · **Твоя кинотека. Твои правила.** · **你的影院。你的规则。**

[![Version](https://img.shields.io/badge/version-1.2.0-blue.svg)](../../releases)
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

### What's new in 1.2.0

- **Statistics** — a dedicated tab with overview cards (movies / TV / anime / average rating / time / episodes / rewatches), status donut chart, combined donut (ratings + monthly activity + weekdays), and top-rated / most-rewatched posters.
- **Chinese anime search (zh-CN)** — now via Bangumi, with Chinese titles. Achievements work for Chinese search too.
- **XP sync fix** — XP no longer rolls back when syncing; takes MAX. XP for achievements now goes into sync immediately.
- **Search bar fix** — the top search bar now reacts to every query, not just the first one.
- **Firefly achievement fix** — `tv_firefly_browncoat` used a wrong `tmdb_id` (1433 instead of 1437) and never triggered.
- **Achievements fix** — a missing `if satisfied` block in `check_achievements_internal` made all achievements silently dead in 1.1.1. Fixed.
- **Cleaner logs** — debug output only in dev builds.

### Features

- **Three content types** — movies, TV shows, anime
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

### Achievement categories

| Category | Count | Examples |
|---|---|---|
| Movies (count) | 8 | Popcorn Lover, Sofa Critic, Architect of Matrix |
| TV (count) | 8 | Pilot Dinner, Live Netflix, Master of Binge-Watching |
| Anime (count) | 8 | TV Tourist, Hardened Otaku, Anime Deity |
| Movie franchises | 50+ | Potterhead, Fast Family, Marvel Fatigue |
| TV shows | 90+ | Breaking Bad Cook, GOT Coffee Cup, Friends Pivot |
| Anime titles | 100+ | Naruto Legend, Drug Cat, Alpha Pathos |
| Humor / memes | 12 | Not the Bees, It's Morbin' Time, Sweet Bread |
| Director / actor | 6 | Nolan Approves, Tarantino Feet, Cage Fever |
| Time / runtime | 5 | Night Owl, Ancient Fossil, Marathon Endurance |
| Rewatch | 4 | Rewatcher, Movie Masochist, Eternal Loop, Memory Collector |

### Wi-Fi Sync

Sync your library between PC and phone over local Wi-Fi. No cloud, no accounts, no data leaves your network.

1. Make sure both devices are on the **same Wi-Fi network**.
2. Open **Settings → Wi-Fi Sync** on either device.
3. Press **Sync**.
4. On the other device, confirm the connection once (a popup will appear).
5. Done. Next time just press **Sync** — it works silently.

**What gets synced:** library (movies, TV, anime), ratings, notes, watch progress, achievements, diary, collections, rewatches. Resolves conflicts by most-recent change. XP takes the maximum, never rolls back.

**Not synced:** settings (theme, language), episode metadata (each device fetches its own).

> **Note for Windows users:** On first sync, Windows Firewall may ask for network access — click **Allow**. If you missed it, the app will show an error with instructions. The installer now adds a firewall rule automatically.

### Tech Stack

Tauri 2.12 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · SQLite (bundled) · TMDB · Shikimori · AniList · Bangumi (zh-CN) · mDNS · Ed25519

### Install

**Windows:** Download **35mm Club Setup v1.2.0.exe** from [Releases](../../releases), run, install (choose English / Russian / Chinese). No dependencies needed — everything is bundled, including WebView2 runtime.

**Android:** Download **35mm Club v1.2.0.apk** from [Releases](../../releases), allow installation from unknown sources, install.

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
3. `npm install`
4. `npm run tauri dev` — dev mode (desktop)
5. `npm run tauri build` — release build (Windows)
6. `npx tauri android build --apk` — release build (Android APK)

The Windows installer will appear in `src-tauri/target/release/bundle/nsis/`. The Android APK will appear in `src-tauri/gen/android/app/build/outputs/apk/universal/release/`.

### Data location

- **Windows:** `%APPDATA%\com.invisiblelevel.thirtyfivemmclub\35mm_club.db`
- **Android:** internal app storage (use **Settings → Backup → Export data** to save).

Back it up before updating. Use **Settings → Backup → Export** for a portable JSON backup.

> **Android note:** Auto Backup is disabled (`android:allowBackup="false"`). Uninstalling removes the database. If you had a backup from an older version, remove it in **Google → Backup** so a fresh install starts clean.

### License

MIT — see [LICENSE](LICENSE).

### Author

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

---

## Русский

35mm Club — локальный трекер фильмов, сериалов и аниме с ачивками, XP, званиями и генеральской кокардой. Все данные лежат в одном файле SQLite на твоём устройстве — без облаков, аккаунтов и сбора данных.

Добавил тайтл → отметил серии → получил ачивку → поднял уровень. Всё происходит на твоей машине, наружу уходят только запросы к публичным API метаданных (TMDB, Shikimori, AniList, Bangumi).

**Доступно на Windows и Android.**

### Что нового в 1.2.0

- **Статистика** — отдельная вкладка: карточки (фильмы / сериалы / аниме / средняя оценка / время / эпизоды / пересмотры), круговая диаграмма статусов, комбинированный донат (рейтинги + активность по месяцам + дни недели), топы по оценке и пересмотрам.
- **Китайский поиск аниме (zh-CN)** — через Bangumi, с китайскими названиями. Ачивки для китайского поиска тоже работают.
- **Синк XP** — опыт больше не откатывается при синхронизации, берётся по максимуму. XP за ачивки уходит в синк сразу.
- **Фикс строки поиска** — верхняя строка теперь реагирует на каждый запрос, а не только на первый.
- **Фикс ачивки «Светлячок»** — `tv_firefly_browncoat` использовала неверный `tmdb_id` (1433 вместо 1437) и никогда не выпадала.
- **Фикс ачивок** — пропавший блок `if satisfied` в `check_achievements_internal` полностью убивал все ачивки в 1.1.1. Исправлено.
- **Чистые логи** — отладочный вывод только в dev-сборке.

### Что умеет

- **Три типа контента** — фильмы, сериалы, аниме
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

### Категории ачивок

| Категория | Количество | Примеры |
|---|---|---|
| Фильмы (счётчик) | 8 | Любитель попкорна, Диванный кинокритик, Архитектор матриц |
| Сериалы (счётчик) | 8 | Ужин под пилот, Живой Netflix, Мастер запойного просмотра |
| Аниме (счётчик) | 8 | Турист из телевизора, Прожжённый отаку, Аниме-божество |
| Франшизы фильмов | 50+ | Главный Гриффиндорщик, За семью и 1/4 мили, Жертва конвейера |
| Сериалы | 90+ | Настоящий химик, Стаканчик из Старбакса, ДРУЗЬЯ: ВЕЧНЫЙ ПОВТОР |
| Аниме-тайтлы | 100+ | Легенда скрытого листа, Кот-наркоман, Альфа-пафос |
| Юмор / мемы | 12 | Пчёлы! Только не пчёлы!, It's Morbin' Time!, Сладкий хлеб |
| Режиссёры / актёры | 6 | Нолан одобряет, Ценитель женских стоп, Клетка переполнена |
| Время / хронометраж | 5 | Полуночный сеанс, Археолог кинематографа, Попа из стали |
| Пересмотры | 4 | Пересмотрщик, Киноман-мазохист, Вечный цикл, Коллекционер воспоминаний |

### Синхронизация по Wi-Fi

Синкает библиотеку между ПК и телефоном по локальной Wi-Fi. Без облака, без аккаунтов, без выхода в интернет — данные не покидают твою сеть.

1. Убедись, что оба устройства в **одной Wi-Fi сети**.
2. Открой **Настройки → Синхронизация** на любом устройстве.
3. Нажми **Синхронизировать**.
4. На втором устройстве подтверди подключение один раз (всплывёт окно).
5. Готово. В следующий раз просто жми **Синхронизировать** — работает молча.

**Что синкается:** библиотека (фильмы, сериалы, аниме), рейтинги, заметки, прогресс просмотра, ачивки, дневник, коллекции, пересмотры. Конфликты решаются по самому свежему изменению. XP берётся по максимуму, не откатывается.

**Что не синкается:** настройки (тема, язык), метаданные серий (каждое устройство тянет само).

> **Для Windows:** при первой синхронизации Windows Firewall может спросить доступ к сети — нажми **Разрешить**. Инсталлятор теперь добавляет правило автоматически. Если всё равно не работает — приложение покажет ошибку с инструкцией.

### Стек

Tauri 2.12 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · SQLite (bundled) · TMDB · Shikimori · AniList · Bangumi (zh-CN) · mDNS · Ed25519

### Установка

**Windows:** Скачай **35mm Club Setup v1.2.0.exe** из раздела [Releases](../../releases), запусти, установи (выбери язык: русский / английский / китайский). Зависимости не нужны — всё вшито в инсталлер, включая WebView2.

**Android:** Скачай **35mm Club v1.2.0.apk** из раздела [Releases](../../releases), разреши установку из неизвестных источников, установи.

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
3. `npm install`
4. `npm run tauri dev` — dev-режим (десктоп)
5. `npm run tauri build` — релиз (Windows)
6. `npx tauri android build --apk` — релиз (Android APK)

Windows-установщик появится в `src-tauri/target/release/bundle/nsis/`. Android APK — в `src-tauri/gen/android/app/build/outputs/apk/universal/release/`.

### Где лежат данные

- **Windows:** `%APPDATA%\com.invisiblelevel.thirtyfivemmclub\35mm_club.db`
- **Android:** внутренняя папка приложения (используй **Настройки → Резервная копия → Экспорт данных**).

Сделай бэкап перед обновлением. Для переноса между устройствами — **Настройки → Резервная копия → Экспорт**.

> **Android:** Auto Backup отключён (`android:allowBackup="false"`). При удалении приложения база стирается. Если остался бэкап от старой версии — удали его в **Google → Резервное копирование**, чтобы свежая установка началась с чистого листа.

### Лицензия

MIT — см. [LICENSE](LICENSE).

### Автор

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

---

## 中文

35mm Club 是一款本地优先的电影、剧集和动漫追踪器，带有成就、经验值、军衔和将军式勋表。所有数据存储在你设备上的单个 SQLite 文件中——无云端、无账号、无数据收集。

添加条目 → 记录剧集 → 获得成就 → 升级。所有操作都在本机完成，只有元数据请求会发送到公开 API（TMDB、Shikimori、AniList、Bangumi）。

**支持 Windows 和 Android。**

### 1.2.0 新功能

- **统计** — 独立标签页：卡片（电影 / 剧集 / 动漫 / 平均评分 / 时长 / 剧集数 / 重看次数）、状态环形图、组合环形图（评分 + 每月活动 + 星期几）、最高评分与最多重看排行。
- **中文动漫搜索 (zh-CN)** — 通过 Bangumi，带中文标题。中文搜索同样支持成就。
- **XP 同步修复** — 同步时经验值不再回退，取最大值。成就获得的 XP 立即进入同步。
- **搜索栏修复** — 顶部搜索栏现在响应每次查询，而不仅是第一次。
- **《萤火虫》成就修复** — `tv_firefly_browncoat` 使用了错误的 `tmdb_id`（1433 而非 1437），从未触发。
- **成就修复** — `check_achievements_internal` 中缺失的 `if satisfied` 代码块导致 1.1.1 中所有成就静默失效。已修复。
- **更干净的日志** — 调试输出仅在开发版本中显示。

### 功能

- **三种内容类型** — 电影、剧集、动漫
- **剧集追踪** — 季、集、观看进度
- **300+ 个成就** — 普通、稀有、史诗、传奇，自定义图标和描述
- **军衔与经验值** — 10 个军事风格军衔，从列兵到将军
- **勋表（кокарда）** — 20 级（少校）解锁，随等级和成就成长
- **统计** — 概览、状态环形图、组合环形图（评分 / 月份 / 星期几）、最高评分与最多重看排行
- **Wi-Fi 同步** — 在同一网络的任意设备上按一个按钮；在 PC 和手机之间同步媒体库、成就、日记、笔记和收藏集。配对一次，之后静默运行。无云端、无账号。
- **重看追踪** — 记录重看评分和笔记，查看每部作品的重看次数。紧凑列表，不杂乱。
- **观影日记** — 带日期、评分和笔记的观看历史，另有独立的 **「重看」** 标签页，显示整个媒体库的所有重看记录
- **自定义收藏集** — 自己的列表，带海报预览
- **评分** — 半星精度（0.5–10.0），适用于电影、剧集和动漫
- **动漫进度** — 剧集计数器，自动检测完成
- **备份与恢复** — 将整个媒体库、成就、日记和收藏集导出为单个 JSON 文件；可在另一台设备上导入
- **三种语言** — 英语、俄语、简体中文，支持系统自动检测
- **深色 / 浅色主题** — 默认深色，可在设置中切换
- **界面缩放** — 100% / 125% / 150% / 200%
- **完全离线** — 首次获取元数据后无需网络即可运行

### 成就类别

| 类别 | 数量 | 示例 |
|---|---|---|
| 电影（计数） | 8 | 爆米花爱好者、沙发影评人、矩阵架构师 |
| 剧集（计数） | 8 | 试播集晚餐、活体 Netflix、刷剧大师 |
| 动漫（计数） | 8 | 电视游客、老油条宅、动漫神明 |
| 电影系列 | 50+ | 首席格兰芬多、为了家人和四分之一英里、流水线受害者 |
| 剧集系列 | 90+ | 真正的化学家、星巴克杯子、老友记：永恒重播 |
| 动漫标题 | 100+ | 木叶传说、吸毒猫、阿尔法·帕福斯 |
| 幽默 / 迷因 | 12 | 蜜蜂！不要蜜蜂！、It's Morbin' Time!、甜面包 |
| 导演 / 演员 | 6 | 诺兰认可、女性脚部鉴赏家、凯奇爆满 |
| 时间 / 时长 | 5 | 午夜场、电影考古学家、钢铁屁股 |
| 重看 | 4 | 重看者、电影受虐狂、永恒循环、回忆收藏家 |

### Wi-Fi 同步

在本地 Wi-Fi 之间同步 PC 和手机上的媒体库。无云端、无账号、数据不离开你的网络。

1. 确保两台设备在**同一 Wi-Fi 网络**下。
2. 在任意一台设备上打开 **设置 → Wi-Fi 同步**。
3. 按下 **同步**。
4. 在另一台设备上确认连接一次（会弹出窗口）。
5. 完成。下次只需按 **同步** — 会静默运行。

**同步内容：** 媒体库（电影、剧集、动漫）、评分、笔记、观看进度、成就、日记、收藏集、重看记录。冲突按最新更改解决。XP 取最大值，不会回退。

**不同步：** 设置（主题、语言）、剧集元数据（每台设备自行获取）。

> **Windows 用户注意：** 首次同步时，Windows 防火墙可能会询问网络访问 — 点击 **允许**。安装程序现在会自动添加规则。如果仍然不行，应用会显示错误并附上说明。

### 技术栈

Tauri 2.12 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · SQLite (bundled) · TMDB · Shikimori · AniList · Bangumi (zh-CN) · mDNS · Ed25519

### 安装

**Windows:** 从 [Releases](../../releases) 下载 **35mm Club Setup v1.2.0.exe**，运行并安装（可选英语 / 俄语 / 中文）。无需任何依赖 — 全部已打包，含 WebView2 运行时。

**Android:** 从 [Releases](../../releases) 下载 **35mm Club v1.2.0.apk**，允许从未知来源安装，然后安装。

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
3. `npm install`
4. `npm run tauri dev` — 开发模式（桌面）
5. `npm run tauri build` — 发布构建（Windows）
6. `npx tauri android build --apk` — 发布构建（Android APK）

Windows 安装包将生成于 `src-tauri/target/release/bundle/nsis/`。Android APK 将生成于 `src-tauri/gen/android/app/build/outputs/apk/universal/release/`。

### 数据位置

- **Windows:** `%APPDATA%\com.invisiblelevel.thirtyfivemmclub\35mm_club.db`
- **Android:** 应用内部存储（使用 **设置 → 备份 → 导出数据**）。

更新前请备份。跨设备迁移 — **设置 → 备份 → 导出**。

> **Android:** 已禁用 Auto Backup（`android:allowBackup="false"`）。卸载应用会删除数据库。如果旧版本留下了备份 — 请在 **Google → 备份** 中删除它，以便全新安装从干净状态开始。

### 许可证

MIT — 见 [LICENSE](LICENSE)。

### 作者

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)