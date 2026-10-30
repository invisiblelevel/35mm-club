<script lang="ts">
  import { Dices } from 'lucide-svelte';
  import { onMount } from 'svelte';

  export type WheelItem = {
    id: string;
    poster_url: string | null;
    title: string;
  };

  interface Props {
    items: WheelItem[];
    winnerIndex: number;
    accent: string;
    showButton?: boolean;
    onSpinEnd?: () => void;
  }

  let { items, winnerIndex, accent, showButton = true, onSpinEnd }: Props = $props();

  const SECTORS = 8;
  const SECTOR_ANGLE = 360 / SECTORS;
  const SPIN_DURATION_MS = 4000;

  const SIZE = 460; 
  const CENTER = SIZE / 2;
  const RADIUS = CENTER - 16;
  const INNER_RADIUS = 75;

  let rotation = $state(0);
  let isAnimating = $state(false); 

  let audioCtx: AudioContext | null = null;
  let tickTimeouts: ReturnType<typeof setTimeout>[] = [];

  function initAudio() {
    if (audioCtx) return;
    try { audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)(); } catch {}
  }

  function playTick(volume: number) {
    if (!audioCtx) return;
    try {
      const osc = audioCtx.createOscillator();
      const gain = audioCtx.createGain();
      osc.type = 'square';
      osc.frequency.value = 1200;
      gain.gain.setValueAtTime(volume, audioCtx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.0001, audioCtx.currentTime + 0.035);
      osc.connect(gain);
      gain.connect(audioCtx.destination);
      osc.start();
      osc.stop(audioCtx.currentTime + 0.04);
    } catch {}
  }

  function scheduleTicks() {
    tickTimeouts.forEach(clearTimeout);
    tickTimeouts = [];
    const ticksCount = 32;
    for (let i = 0; i < ticksCount; i++) {
      const t = i / ticksCount;
      const delay = SPIN_DURATION_MS * Math.pow(t, 1.8);
      const volume = 0.09 * (1 - t * 0.75);
      if (volume <= 0.005) continue;
      tickTimeouts.push(setTimeout(() => playTick(volume), delay));
    }
  }

  export function triggerSpin() {
    if (isAnimating) return;
    isAnimating = true;
    initAudio();
    if (audioCtx && audioCtx.state === 'suspended') audioCtx.resume();

    const center = winnerIndex * SECTOR_ANGLE + SECTOR_ANGLE / 2;
    const jitter = (Math.random() - 0.5) * (SECTOR_ANGLE * 0.6);
    const desiredMod = ((360 - center + jitter) % 360 + 360) % 360;
    const currentMod = ((rotation % 360) + 360) % 360;
    let diff = desiredMod - currentMod;
    if (diff < 0) diff += 360;
    
    rotation += 6 * 360 + diff;

    requestAnimationFrame(() => scheduleTicks());

    setTimeout(() => {
      isAnimating = false;
      onSpinEnd?.();
    }, SPIN_DURATION_MS + 100);
  }

  export function resetRotation() {
    tickTimeouts.forEach(clearTimeout);
    tickTimeouts = [];
    isAnimating = false;
    rotation = 0;
  }

  onMount(() => {
    return () => {
      tickTimeouts.forEach(clearTimeout);
      audioCtx?.close().catch(() => {});
    };
  });

  function polarToCartesian(cx: number, cy: number, r: number, angleDeg: number) {
    const angleRad = ((angleDeg - 90) * Math.PI) / 180;
    return { x: cx + r * Math.cos(angleRad), y: cy + r * Math.sin(angleRad) };
  }

  function sectorPath(startAngle: number, endAngle: number) {
    const start = polarToCartesian(CENTER, CENTER, RADIUS, startAngle);
    const end = polarToCartesian(CENTER, CENTER, RADIUS, endAngle);
    const largeArc = endAngle - startAngle > 180 ? 1 : 0;
    return `M ${CENTER} ${CENTER} L ${start.x} ${start.y} A ${RADIUS} ${RADIUS} 0 ${largeArc} 1 ${end.x} ${end.y} Z`;
  }

  function labelPosition(index: number) {
    const center = index * SECTOR_ANGLE + SECTOR_ANGLE / 2;
    return polarToCartesian(CENTER, CENTER, RADIUS * 0.62, center);
  }

  const wheelItems = $derived(Array.from({ length: SECTORS }, (_, i) => items[i] ?? null));

  const sectorColors = $derived([
    `color-mix(in srgb, ${accent} 28%, #1a1a1a)`,
    `color-mix(in srgb, ${accent} 14%, #0f0f0f)`,
  ]);
</script>

<div class="relative flex flex-col items-center">
  <!-- Стрелка -->
  <div class="relative z-20 -mb-3">
    <div class="w-0 h-0 border-l-[18px] border-r-[18px] border-t-[30px] border-l-transparent border-r-transparent"
         style="border-top-color: {accent}; filter: drop-shadow(0 0 10px {accent});"></div>
  </div>

  <div class="relative">
    <div class="absolute inset-0 rounded-full blur-2xl opacity-40 transition-all duration-500"
         style="background: radial-gradient(circle, {accent}66 0%, transparent 70%);"></div>

    <div class="relative w-[min(460px,90vw)] h-[min(460px,90vw)] rounded-full"
         style="transform: rotate({rotation}deg); transition: transform {isAnimating ? SPIN_DURATION_MS : 300}ms cubic-bezier(0.15, 0.85, 0.15, 1);">
      <svg viewBox="0 0 {SIZE} {SIZE}" class="w-full h-full drop-shadow-2xl">
        <!-- Внешнее кольцо -->
        <circle cx={CENTER} cy={CENTER} r={RADIUS + 4} fill="none" stroke={accent} stroke-width="2" opacity="0.5" />

        <!-- Сектора -->
        {#each wheelItems as item, i}
          {@const startAngle = i * SECTOR_ANGLE}
          <path d={sectorPath(startAngle, startAngle + SECTOR_ANGLE)} fill={sectorColors[i % 2]} stroke="#0a0a0a" stroke-width="3" />
        {/each}

        <!-- Постеры через foreignObject (WebView2 не рендерит data: в <image>) -->
        {#each wheelItems as item, i}
          {@const pos = labelPosition(i)}
          {#if item?.poster_url}
            <foreignObject x={pos.x - 42} y={pos.y - 42} width="84" height="84">
              <div style="width: 84px; height: 84px; border-radius: 50%; overflow: hidden; border: 2px solid {accent}; box-sizing: border-box;">
                <img
                  src={item.poster_url}
                  alt={item.title}
                  referrerpolicy="no-referrer"
                  style="width: 100%; height: 100%; object-fit: cover; display: block;"
                  onerror={() => {
                    if (wheelItems[i]) wheelItems[i] = { ...wheelItems[i], poster_url: null };
                  }}
                />
              </div>
            </foreignObject>
          {:else}
            <circle cx={pos.x} cy={pos.y} r="40" fill="#1f1f1f" stroke="#333" stroke-width="2" />
            <g transform="translate({pos.x - 16}, {pos.y - 16})" opacity="0.4">
              <Dices size={32} color="#666" />
            </g>
          {/if}
        {/each}

        <!-- Линии разделения -->
        {#each Array.from({ length: SECTORS }) as _, i}
          {@const angle = i * SECTOR_ANGLE}
          {@const p1 = polarToCartesian(CENTER, CENTER, INNER_RADIUS, angle)}
          {@const p2 = polarToCartesian(CENTER, CENTER, RADIUS, angle)}
          <line x1={p1.x} y1={p1.y} x2={p2.x} y2={p2.y} stroke="#0a0a0a" stroke-width="3" />
        {/each}

        <!-- Центральный круг -->
        <circle cx={CENTER} cy={CENTER} r={INNER_RADIUS} fill="#0a0a0a" stroke={accent} stroke-width="3" opacity="0.95" />
      </svg>

      <!-- Иконка в центре -->
      <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
        <div class="w-16 h-16 rounded-full flex items-center justify-center {isAnimating ? 'animate-spin' : ''}" style="color: {accent};">
          <Dices size={36} />
        </div>
      </div>
    </div>
  </div>
</div>