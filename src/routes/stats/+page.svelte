<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import {
    BarChart3, Film, Tv, Sparkles, Star, Clock, RotateCcw,
    TrendingUp, Award, CalendarDays, Activity
  } from 'lucide-svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';

  type Overview = {
    total_titles: number; total_movies: number; total_tv: number; total_anime: number;
    total_completed: number; total_watching: number; total_planned: number;
    total_on_hold: number; total_dropped: number;
    total_minutes: number; total_episodes: number; total_rewatches: number;
    average_rating: number | null; rated_count: number;
  };
  type RatingBucket = { bucket: string; count: number };
  type Monthly = { month: string; count: number };
  type Weekday = { weekday: string; count: number };
  type TopTitle = { title: string; poster_url: string | null; type_field: string; tmdb_id: number | null; rating: number | null; rewatch_count: number };
  type StatsFull = {
    overview: Overview;
    ratings: RatingBucket[]; monthly: Monthly[]; weekdays: Weekday[];
    top_rated: TopTitle[]; most_rewatched: TopTitle[];
  };

  let stats: StatsFull | null = $state(null);
  let loading = $state(true);
  let activeTab = $state<'overview' | 'activity' | 'top'>('overview');
  let mounted = $state(false);
  let hoveredRing = $state<string | null>(null);
  let hoveredSeg = $state<{ ring: string; label: string; value: number } | null>(null);

  onMount(async () => {
    try {
      stats = await invoke<StatsFull>('get_stats');
      setTimeout(() => { mounted = true; }, 100);
    } catch (e) {
      console.error('stats failed:', e);
    } finally {
      loading = false;
    }
  });

  function formatHours(minutes: number): string {
    const h = Math.floor(minutes / 60);
    const d = Math.floor(h / 24);
    if (d > 0) return `${d} ${$t('stats.days')} ${h % 24} ${$t('stats.hours')}`;
    return `${h} ${$t('stats.hours')}`;
  }

  function monthLabel(m: string): string {
    const [y, mo] = m.split('-');
    const names = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];
    return `${names[parseInt(mo) - 1]} ${y.slice(2)}`;
  }

  // -------- СТАТУСЫ (кольцо для «Обзор») --------
  const statusSegments = $derived.by(() => {
    if (!stats) return [];
    const items = [
      { key: 'completed', count: stats.overview.total_completed, color: '#22c55e' },
      { key: 'watching',  count: stats.overview.total_watching,  color: '#3b82f6' },
      { key: 'planned',   count: stats.overview.total_planned,   color: '#ff9000' },
      { key: 'on_hold',   count: stats.overview.total_on_hold,   color: '#eab308' },
      { key: 'dropped',   count: stats.overview.total_dropped,   color: '#ef4444' },
    ];
    const total = items.reduce((s, i) => s + i.count, 0) || 1;
    let angle = 0;
    return items.map(item => {
      const percent = (item.count / total) * 100;
      const sweep = (item.count / total) * 360;
      const start = angle;
      angle += sweep;
      return { ...item, percent, start, sweep, total };
    });
  });

  // -------- КОМБИНИРОВАННЫЙ КРУГ (Активность) --------
  // 3 кольца: внутреннее (рейтинги), среднее (месяцы, последние 12),
  // внешнее (дни недели). Донат: у каждого сектора свой радиус от/до.
  const RATINGS_PALETTE = ['#ef4444','#f97316','#eab308','#84cc16','#22c55e','#10b981','#14b8a6','#06b6d4','#3b82f6','#ff9000'];
  const MONTHS_PALETTE = ['#ff9000','#ffa733','#ffbe66','#ffd699','#ffe6cc','#e6b47f','#cc9f66','#b38a4d','#997533','#80601a','#664d00','#4d3a00'];
  const WEEKDAYS_PALETTE = ['#ff9000','#ffa733','#ffbe66','#ffd699','#c9a77e','#b38a4d','#997533'];

  type RingSeg = {
    ring: 'ratings' | 'months' | 'weekdays';
    label: string;
    value: number;
    color: string;
    startAngle: number;
    endAngle: number;
  };

  function donutPath(cx: number, cy: number, rInner: number, rOuter: number, startDeg: number, endDeg: number): string {
    const sweep = endDeg - startDeg;
    if (sweep <= 0.01) return '';
    if (sweep >= 359.99) {
      // полный круг
      const rad = 2 * Math.PI;
      const outer1 = { x: cx + rOuter, y: cy };
      const inner1 = { x: cx + rInner, y: cy };
      return `M ${outer1.x} ${outer1.y}
              A ${rOuter} ${rOuter} 0 1 1 ${cx - rOuter} ${cy}
              A ${rOuter} ${rOuter} 0 1 1 ${outer1.x} ${outer1.y}
              L ${inner1.x} ${inner1.y}
              A ${rInner} ${rInner} 0 1 0 ${cx - rInner} ${cy}
              A ${rInner} ${rInner} 0 1 0 ${inner1.x} ${inner1.y} Z`;
    }
    const startRad = (startDeg - 90) * Math.PI / 180;
    const endRad = (endDeg - 90) * Math.PI / 180;
    const ox1 = cx + rOuter * Math.cos(startRad);
    const oy1 = cy + rOuter * Math.sin(startRad);
    const ox2 = cx + rOuter * Math.cos(endRad);
    const oy2 = cy + rOuter * Math.sin(endRad);
    const ix1 = cx + rInner * Math.cos(endRad);
    const iy1 = cy + rInner * Math.sin(endRad);
    const ix2 = cx + rInner * Math.cos(startRad);
    const iy2 = cy + rInner * Math.sin(startRad);
    const largeArc = sweep > 180 ? 1 : 0;
    return `M ${ox1} ${oy1}
            A ${rOuter} ${rOuter} 0 ${largeArc} 1 ${ox2} ${oy2}
            L ${ix1} ${iy1}
            A ${rInner} ${rInner} 0 ${largeArc} 0 ${ix2} ${iy2} Z`;
  }

  // Строим три кольца
  const rings = $derived.by(() => {
    if (!stats) return [] as RingSeg[];
    const result: RingSeg[] = [];

    // --- Рейтинги (внутреннее кольцо) ---
    const ratingCounts = stats.ratings.map(r => r.count);
    const ratingsTotal = ratingCounts.reduce((s, n) => s + n, 0);
    if (ratingsTotal > 0) {
      let angle = 0;
      for (let i = 0; i < stats.ratings.length; i++) {
        const r = stats.ratings[i];
        if (r.count === 0) continue;
        const sweep = (r.count / ratingsTotal) * 360;
        result.push({
          ring: 'ratings',
          label: `${$t('stats.rating_short')} ${r.bucket}`,
          value: r.count,
          color: RATINGS_PALETTE[i % RATINGS_PALETTE.length],
          startAngle: angle,
          endAngle: angle + sweep,
        });
        angle += sweep;
      }
    }

    // --- Месяцы (среднее кольцо) — последние 12 ---
    const months = stats.monthly.slice(-12);
    const monthsTotal = months.reduce((s, m) => s + m.count, 0);
    if (monthsTotal > 0) {
      let angle = 0;
      for (let i = 0; i < months.length; i++) {
        const m = months[i];
        const sweep = (m.count / monthsTotal) * 360;
        result.push({
          ring: 'months',
          label: monthLabel(m.month),
          value: m.count,
          color: MONTHS_PALETTE[i % MONTHS_PALETTE.length],
          startAngle: angle,
          endAngle: angle + sweep,
        });
        angle += sweep;
      }
    }

    // --- Дни недели (внешнее кольцо) ---
    // Порядок Пн..Вс: 1..6,0
    const wdMap = new Map<string, number>();
    for (const w of stats.weekdays) wdMap.set(w.weekday, w.count);
    const wdOrder = [1,2,3,4,5,6,0];
    const wdLabels = ['Пн','Вт','Ср','Чт','Пт','Сб','Вс'];
    const wdCounts = wdOrder.map(i => wdMap.get(String(i)) ?? 0);
    const wdTotal = wdCounts.reduce((s, n) => s + n, 0);
    if (wdTotal > 0) {
      let angle = 0;
      for (let i = 0; i < wdCounts.length; i++) {
        const c = wdCounts[i];
        if (c === 0) continue;
        const sweep = (c / wdTotal) * 360;
        result.push({
          ring: 'weekdays',
          label: wdLabels[i],
          value: c,
          color: WEEKDAYS_PALETTE[i % WEEKDAYS_PALETTE.length],
          startAngle: angle,
          endAngle: angle + sweep,
        });
        angle += sweep;
      }
    }

    return result;
  });

  const ringsTotalForCenter = $derived.by(() => {
    if (!stats) return 0;
    // центр показывает суммарное число просмотров (записей в дневнике)
    return stats.overview.total_completed + stats.overview.total_rewatches;
  });

  function isDimmed(seg: RingSeg): boolean {
    if (hoveredRing === null) return false;
    return seg.ring !== hoveredRing;
  }

  // -------- СТАТУСЫ: круглый путь (используется только в «Обзор») --------
  function arcPath(cx: number, cy: number, r: number, startDeg: number, sweepDeg: number): string {
    if (sweepDeg >= 359.99) {
      return `M ${cx} ${cy - r} A ${r} ${r} 0 1 1 ${cx - 0.01} ${cy - r} Z`;
    }
    const startRad = (startDeg - 90) * Math.PI / 180;
    const endRad = (startDeg + sweepDeg - 90) * Math.PI / 180;
    const x1 = cx + r * Math.cos(startRad);
    const y1 = cy + r * Math.sin(startRad);
    const x2 = cx + r * Math.cos(endRad);
    const y2 = cy + r * Math.sin(endRad);
    const largeArc = sweepDeg > 180 ? 1 : 0;
    return `M ${cx} ${cy} L ${x1} ${y1} A ${r} ${r} 0 ${largeArc} 1 ${x2} ${y2} Z`;
  }
</script>

<div class="space-y-6">
  <!-- ХЕДЕР -->
  <div class="relative overflow-hidden rounded-3xl bg-gradient-to-br from-gold/15 via-surface to-surface border border-gold/20 p-6 md:p-8">
    <div class="absolute -right-20 -top-20 w-64 h-64 rounded-full bg-gold/10 blur-3xl pointer-events-none"></div>
    <div class="absolute -left-10 -bottom-20 w-48 h-48 rounded-full bg-gold/5 blur-3xl pointer-events-none"></div>
    <div class="relative">
      <div class="flex items-center gap-3 mb-2">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-gold to-gold-dark flex items-center justify-center shadow-lg shadow-gold/20">
          <BarChart3 size={20} class="text-ink" />
        </div>
        <h1 class="font-display text-2xl md:text-3xl font-bold">{$t('stats.title')}</h1>
      </div>
      {#if stats && stats.overview.total_titles > 0}
        <p class="text-sm md:text-base text-zinc-400 mt-3 max-w-2xl">
          {$t('stats.hero_summary', {
            values: {
              titles: stats.overview.total_titles,
              hours: Math.floor(stats.overview.total_minutes / 60),
            }
          })}
        </p>
      {/if}
    </div>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-20">
      <div class="w-10 h-10 rounded-full border-2 border-gold border-t-transparent animate-spin"></div>
    </div>
  {:else if !stats || stats.overview.total_titles === 0}
    <EmptyState icon={BarChart3} title={$t('stats.empty_title')} description={$t('stats.empty_desc')} />
  {:else}
    <!-- Табы -->
    <div class="flex gap-1.5 p-1.5 bg-surface border border-border rounded-2xl w-fit">
      {#each [
        { id: 'overview', label: $t('stats.tab_overview') },
        { id: 'activity', label: $t('stats.tab_activity') },
        { id: 'top', label: $t('stats.tab_top') },
      ] as tab}
        <button
          class="px-4 py-2 rounded-xl text-sm font-medium transition-all duration-300
                 {activeTab === tab.id
                   ? 'bg-gradient-to-b from-gold to-gold-dark text-ink shadow-lg shadow-gold/20'
                   : 'text-zinc-400 hover:text-zinc-100'}"
          onclick={() => activeTab = tab.id}
        >{tab.label}</button>
      {/each}
    </div>

    {#if activeTab === 'overview'}
      <!-- 4 КАРТОЧКИ -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3 md:gap-4">
        {#each [
          { label: $t('stats.movies'), value: stats.overview.total_movies, icon: Film, accent: 'from-blue-500/20 to-blue-500/5', iconColor: 'text-blue-400' },
          { label: $t('stats.tv'), value: stats.overview.total_tv, icon: Tv, accent: 'from-purple-500/20 to-purple-500/5', iconColor: 'text-purple-400' },
          { label: $t('stats.anime'), value: stats.overview.total_anime, icon: Sparkles, accent: 'from-pink-500/20 to-pink-500/5', iconColor: 'text-pink-400' },
          { label: $t('stats.avg_rating'), value: stats.overview.average_rating?.toFixed(1) ?? '—', icon: Star, accent: 'from-gold/20 to-gold/5', iconColor: 'text-gold' },
        ] as card}
          <div class="group relative overflow-hidden rounded-2xl bg-gradient-to-br {card.accent} bg-surface border border-border p-4 md:p-5 transition-all duration-500 hover:border-gold/40 hover:-translate-y-0.5 hover:shadow-xl hover:shadow-black/30">
            <div class="w-10 h-10 rounded-xl bg-ink/50 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
              <card.icon size={20} class={card.iconColor} />
            </div>
            <div class="text-3xl md:text-4xl font-display font-bold leading-none">{card.value}</div>
            <div class="text-xs md:text-sm text-zinc-500 mt-2">{card.label}</div>
          </div>
        {/each}
      </div>

      <!-- 3 КАРТОЧКИ -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-3 md:gap-4">
        {#each [
          { label: $t('stats.total_time'), value: formatHours(stats.overview.total_minutes), icon: Clock },
          { label: $t('stats.episodes'), value: String(stats.overview.total_episodes), icon: TrendingUp },
          { label: $t('stats.rewatches'), value: String(stats.overview.total_rewatches), icon: RotateCcw },
        ] as card}
          <div class="group relative overflow-hidden rounded-2xl bg-surface border border-border p-4 md:p-5 transition-all duration-300 hover:border-gold/40">
            <div class="flex items-center justify-between mb-3">
              <card.icon size={18} class="text-gold" />
              <div class="w-1.5 h-1.5 rounded-full bg-gold/40 group-hover:bg-gold transition-colors"></div>
            </div>
            <div class="text-xl md:text-2xl font-display font-bold">{card.value}</div>
            <div class="text-xs md:text-sm text-zinc-500 mt-1">{card.label}</div>
          </div>
        {/each}
      </div>

      <!-- СТАТУСЫ -->
      <div class="rounded-3xl bg-surface border border-border p-5 md:p-6">
        <h3 class="font-display font-semibold mb-5 flex items-center gap-2">
          <BarChart3 size={18} class="text-gold" />
          {$t('stats.statuses')}
        </h3>
        <div class="flex flex-col md:flex-row items-center gap-6 md:gap-10">
          <div class="relative shrink-0">
            <svg width="180" height="180" viewBox="0 0 180 180">
              {#each statusSegments as seg}
                {#if seg.count > 0}
                  <path
                    d={arcPath(90, 90, 80, seg.start, seg.sweep)}
                    fill={seg.color}
                    class="transition-all duration-700"
                    style="opacity: {mounted ? 1 : 0}"
                  />
                {/if}
              {/each}
              <circle cx="90" cy="90" r="50" class="fill-surface" />
            </svg>
            <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
              <div class="text-3xl font-display font-bold">{stats.overview.total_titles}</div>
              <div class="text-[10px] text-zinc-500 uppercase tracking-wider">{$t('stats.total_short')}</div>
            </div>
          </div>
          <div class="flex-1 w-full space-y-2">
            {#each statusSegments as seg}
              <div class="flex items-center gap-3 p-2 rounded-lg hover:bg-ink-light transition-colors">
                <div class="w-3 h-3 rounded-full shrink-0" style="background: {seg.color}"></div>
                <div class="flex-1 text-sm">{$t('status.' + seg.key)}</div>
                <div class="text-sm font-medium">{seg.count}</div>
                <div class="w-12 text-right text-xs text-zinc-500">{seg.percent.toFixed(1)}%</div>
              </div>
            {/each}
          </div>
        </div>
      </div>

    {:else if activeTab === 'activity'}
      <!-- БОЛЬШОЙ КОМБИНИРОВАННЫЙ КРУГ -->
      <div class="rounded-3xl bg-surface border border-border p-5 md:p-8">
        <div class="flex items-start justify-between mb-2 flex-wrap gap-2">
          <h3 class="font-display font-semibold flex items-center gap-2">
            <Activity size={18} class="text-gold" />
            {$t('stats.activity_rings')}
          </h3>
          {#if hoveredSeg}
            <span class="text-xs text-zinc-400">
              <span class="inline-block w-2 h-2 rounded-full mr-1" style="background: {hoveredSeg.value > 0 ? '#ff9000' : '#555'}"></span>
              {hoveredSeg.label}: <span class="text-zinc-100 font-medium">{hoveredSeg.value}</span>
            </span>
          {:else}
            <span class="text-xs text-zinc-500">{$t('stats.hover_hint')}</span>
          {/if}
        </div>

        <div class="flex flex-col lg:flex-row items-center justify-center gap-8 lg:gap-12 py-4">
          <!-- Сам круг -->
          <div class="relative shrink-0" style="width: 340px; height: 340px;">
            <svg width="340" height="340" viewBox="0 0 340 340">
              <!-- Фоновые кольца -->
              <circle cx="170" cy="170" r="155" fill="none" stroke="currentColor" class="text-border" stroke-width="1" opacity="0.3" />
              <circle cx="170" cy="170" r="120" fill="none" stroke="currentColor" class="text-border" stroke-width="1" opacity="0.3" />
              <circle cx="170" cy="170" r="85" fill="none" stroke="currentColor" class="text-border" stroke-width="1" opacity="0.3" />

              <!-- Секторы -->
              {#each rings as seg}
                {@const isHovered = hoveredSeg?.ring === seg.ring && hoveredSeg?.label === seg.label}
                <path
                  d={donutPath(
                    170, 170,
                    seg.ring === 'ratings' ? 60 : seg.ring === 'months' ? 95 : 130,
                    seg.ring === 'ratings' ? 85 : seg.ring === 'months' ? 120 : 155,
                    seg.startAngle, seg.endAngle
                  )}
                  fill={seg.color}
                  opacity={mounted ? (isDimmed(seg) ? 0.15 : 1) : 0}
                  class="transition-all duration-300 cursor-pointer"
                  style="transform-origin: 170px 170px; transform: scale({isHovered ? 1.02 : 1});"
                  onmouseenter={() => {
                    hoveredRing = seg.ring;
                    hoveredSeg = { ring: seg.ring, label: seg.label, value: seg.value };
                  }}
                  onmouseleave={() => {
                    hoveredRing = null;
                    hoveredSeg = null;
                  }}
                />
              {/each}

              <!-- Центр -->
              <circle cx="170" cy="170" r="55" class="fill-surface" />
              <text x="170" y="162" text-anchor="middle" class="fill-zinc-100" style="font-family: 'Geist', sans-serif; font-weight: 700; font-size: 28px;">
                {ringsTotalForCenter}
              </text>
              <text x="170" y="182" text-anchor="middle" class="fill-zinc-500" style="font-size: 10px; text-transform: uppercase; letter-spacing: 1px;">
                {$t('stats.total_short')}
              </text>
            </svg>
          </div>

          <!-- Легенда -->
          <div class="flex flex-col gap-5 w-full lg:w-80">
            <!-- Рейтинги -->
            <div
              class="p-3 rounded-2xl border border-border transition-all cursor-pointer {hoveredRing === 'ratings' ? 'border-gold/60 bg-gold/5' : 'bg-ink-light/30'}"
              onmouseenter={() => { hoveredRing = 'ratings'; hoveredSeg = null; }}
              onmouseleave={() => { hoveredRing = null; }}
            >
              <div class="flex items-center gap-2 mb-2">
                <div class="w-3 h-3 rounded-full bg-gradient-to-br from-red-500 to-gold"></div>
                <div class="text-sm font-medium">{$t('stats.ratings_dist')}</div>
                <div class="ml-auto text-xs text-zinc-500">
                  {stats.ratings.reduce((s, r) => s + r.count, 0)}
                </div>
              </div>
              <div class="flex flex-wrap gap-1.5">
                {#each stats.ratings as r, i}
                  {#if r.count > 0}
                    <div class="flex items-center gap-1 text-[11px] text-zinc-400">
                      <span class="w-2 h-2 rounded-sm" style="background: {RATINGS_PALETTE[i % RATINGS_PALETTE.length]}"></span>
                      {r.bucket.split('-')[0]}
                    </div>
                  {/if}
                {/each}
              </div>
            </div>

            <!-- Месяцы -->
            <div
              class="p-3 rounded-2xl border border-border transition-all cursor-pointer {hoveredRing === 'months' ? 'border-gold/60 bg-gold/5' : 'bg-ink-light/30'}"
              onmouseenter={() => { hoveredRing = 'months'; hoveredSeg = null; }}
              onmouseleave={() => { hoveredRing = null; }}
            >
              <div class="flex items-center gap-2 mb-2">
                <div class="w-3 h-3 rounded-full bg-gradient-to-br from-gold-dark to-gold-light"></div>
                <div class="text-sm font-medium">{$t('stats.monthly')}</div>
                <div class="ml-auto text-xs text-zinc-500">
                  {stats.monthly.slice(-12).reduce((s, m) => s + m.count, 0)}
                </div>
              </div>
              <div class="text-[11px] text-zinc-500">
                {$t('stats.last_12_months')}
              </div>
            </div>

            <!-- Дни недели -->
            <div
              class="p-3 rounded-2xl border border-border transition-all cursor-pointer {hoveredRing === 'weekdays' ? 'border-gold/60 bg-gold/5' : 'bg-ink-light/30'}"
              onmouseenter={() => { hoveredRing = 'weekdays'; hoveredSeg = null; }}
              onmouseleave={() => { hoveredRing = null; }}
            >
              <div class="flex items-center gap-2 mb-2">
                <div class="w-3 h-3 rounded-full bg-gradient-to-br from-gold to-yellow-900"></div>
                <div class="text-sm font-medium">{$t('stats.weekdays')}</div>
                <div class="ml-auto text-xs text-zinc-500">
                  {stats.weekdays.reduce((s, w) => s + w.count, 0)}
                </div>
              </div>
              <div class="flex flex-wrap gap-1.5">
                {#each ['Пн','Вт','Ср','Чт','Пт','Сб','Вс'] as d, i}
                  <div class="flex items-center gap-1 text-[11px] text-zinc-400">
                    <span class="w-2 h-2 rounded-sm" style="background: {WEEKDAYS_PALETTE[i % WEEKDAYS_PALETTE.length]}"></span>
                    {d}
                  </div>
                {/each}
              </div>
            </div>
          </div>
        </div>
      </div>

    {:else if activeTab === 'top'}
      <!-- ТОП ПО РЕЙТИНГУ -->
      <div class="rounded-3xl bg-surface border border-border p-5 md:p-6">
        <h3 class="font-display font-semibold mb-5 flex items-center gap-2">
          <Award size={18} class="text-gold" />
          {$t('stats.top_rated')}
        </h3>
        {#if stats.top_rated.length === 0}
          <p class="text-sm text-zinc-500">{$t('stats.no_ratings')}</p>
        {:else}
          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3 md:gap-4">
            {#each stats.top_rated as item, i}
              <a
                href={item.type_field === 'anime' ? `/anime/${item.tmdb_id}` : `/title/${item.tmdb_id}?type=${item.type_field}`}
                class="group relative rounded-2xl overflow-hidden bg-ink-light border border-border hover:border-gold/60 transition-all duration-300 hover:-translate-y-1 hover:shadow-xl hover:shadow-black/40"
              >
                <div class="aspect-[2/3] overflow-hidden relative">
                  {#if item.poster_url}
                    <img src={item.poster_url} alt={item.title} loading="lazy" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-700" />
                  {:else}
                    <div class="w-full h-full flex items-center justify-center text-zinc-600"><Film size={32} /></div>
                  {/if}
                  <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent"></div>
                  <div class="absolute top-2 left-2 w-7 h-7 rounded-full bg-gradient-to-br from-gold to-gold-dark text-ink text-xs font-bold flex items-center justify-center shadow-lg">
                    {i + 1}
                  </div>
                  <div class="absolute bottom-2 left-2 right-2 flex items-center gap-1 text-gold text-sm font-semibold drop-shadow-lg">
                    <Star size={12} fill="currentColor" /> {item.rating?.toFixed(1)}
                  </div>
                </div>
                <div class="p-2.5">
                  <p class="text-xs font-medium truncate">{item.title}</p>
                </div>
              </a>
            {/each}
          </div>
        {/if}
      </div>

      <!-- ТОП ПО ПЕРЕСМОТРАМ -->
      <div class="rounded-3xl bg-surface border border-border p-5 md:p-6">
        <h3 class="font-display font-semibold mb-5 flex items-center gap-2">
          <RotateCcw size={18} class="text-gold" />
          {$t('stats.most_rewatched')}
        </h3>
        {#if stats.most_rewatched.length === 0}
          <p class="text-sm text-zinc-500">{$t('stats.no_rewatches')}</p>
        {:else}
          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3 md:gap-4">
            {#each stats.most_rewatched as item, i}
              <a
                href={item.type_field === 'anime' ? `/anime/${item.tmdb_id}` : `/title/${item.tmdb_id}?type=${item.type_field}`}
                class="group relative rounded-2xl overflow-hidden bg-ink-light border border-border hover:border-gold/60 transition-all duration-300 hover:-translate-y-1 hover:shadow-xl hover:shadow-black/40"
              >
                <div class="aspect-[2/3] overflow-hidden relative">
                  {#if item.poster_url}
                    <img src={item.poster_url} alt={item.title} loading="lazy" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-700" />
                  {:else}
                    <div class="w-full h-full flex items-center justify-center text-zinc-600"><Film size={32} /></div>
                  {/if}
                  <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent"></div>
                  <div class="absolute top-2 right-2 px-2.5 py-1 rounded-full bg-gradient-to-br from-gold to-gold-dark text-ink text-xs font-bold shadow-lg">
                    ×{item.rewatch_count}
                  </div>
                </div>
                <div class="p-2.5">
                  <p class="text-xs font-medium truncate">{item.title}</p>
                </div>
              </a>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</div>