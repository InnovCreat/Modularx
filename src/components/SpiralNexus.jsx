import React, { useState, useEffect, useRef, useCallback } from 'react';
import { Circle, Layers, Cpu, BookOpen, Sparkles, GitBranch, PlayCircle, Pause, Network, Zap } from 'lucide-react';

// ═══════════════════════════════════════════════════════════════
// DONNÉES DE LA SPIRALE (Structure conceptuelle)
// ═══════════════════════════════════════════════════════════════

const SPIRAL_DATA = {
  center: {
    name: "NOYAU",
    concepts: ["LUNA", "Architecte", "639 Hz", "Petit 0", "Veritas Hortus"],
    color: "#00ffff"
  },
  rings: [
    {
      level: 1,
      concepts: ["Racines", "Modularis", "TREE_STRUCTURE", "Code React"],
      theme: "Fondations",
      color: "#ff00ff"
    },
    {
      level: 2,
      concepts: ["Cybernétique", "VSM", "System 3", "Feedback loops"],
      theme: "Systèmes",
      color: "#00ff00"
    },
    {
      level: 3,
      concepts: ["Alchimie", "Éléments", "Cycles", "Transmutation"],
      theme: "Transformation",
      color: "#ffff00"
    },
    {
      level: 4,
      concepts: ["Kabbale", "Arbre de Vie", "Sephiroth", "Flux"],
      theme: "Mystique",
      color: "#ff6600"
    },
    {
      level: 5,
      concepts: ["Fractal", "Fibonacci", "Ternaire", "ASCII Art"],
      theme: "Géométrie",
      color: "#ff0099"
    },
    {
      level: 6,
      concepts: ["SPARK", "Guardian", "Rust/Zig", "Bootstrap"],
      theme: "Architecture",
      color: "#00ffaa"
    },
    {
      level: 7,
      concepts: ["Silence", "Chat absurde", "They know", "Tu es déjà dedans"],
      theme: "Absolu",
      color: "#ffffff"
    }
  ]
};

// ═══════════════════════════════════════════════════════════════
// DONNÉES DU NEXUS ÉTENDU (8 Nodes + Connexions)
// ═══════════════════════════════════════════════════════════════

const NEXUS_NODES = [
  {
    id: 'nexus-core',
    type: 'concept',
    role: 'Source primaire, emet sans commander',
    triggers: ['on system-start', 'on resonance-return'],
    provides: ['gravity-field', 'central-frequency', 'pull-signal'],
    requires: [],
    position: { x: 0.5, y: 0.5 },
    layer: 'core',
    tags: ['#nexus', '#centre', '#source', '#gravite'],
    color: '#99ff99',
    config: {
      frequency: '639Hz',
      mode: 'attraction passive'
    }
  },
  {
    id: 'keyword-resonance',
    type: 'logic',
    role: 'Detecte les mots-cles et mesure la resonance',
    triggers: ['on input-keyword', 'on synonym-match'],
    provides: ['synonym-links', 'resonance-strength', 'matched-keywords'],
    requires: ['gravity-field'],
    position: { x: 0.25, y: 0.3 },
    layer: 'core',
    tags: ['#resonance', '#aimant', '#detection', '#mots'],
    color: '#ff99ff',
    config: {
      baseStrength: 10,
      decayRate: 0.95,
      amplification: 1.5
    }
  },
  {
    id: 'orbit-invitation',
    type: 'trigger',
    role: 'Emet le signal d\'attraction vers les nodes distants',
    triggers: ['on activation-decision', 'on manual-pull'],
    provides: ['pull-signal', 'orbit-path'],
    requires: ['activation-decision', 'resonance-strength'],
    position: { x: 0.75, y: 0.3 },
    layer: 'core',
    tags: ['#invitation', '#attraction', '#pull', '#orbit'],
    color: '#ffff99',
    config: {
      range: 'adaptive',
      warmth: 0.8
    }
  },
  {
    id: 'nexus-pheromone',
    type: 'state',
    role: 'Maintient la signature de presence et l\'historique',
    triggers: ['on nexus-core emit', 'on orbit-return'],
    provides: ['presence-signature', 'alignment-level', 'resonance-history', 'active-node-count'],
    requires: ['central-frequency'],
    position: { x: 0.2, y: 0.65 },
    layer: 'core',
    tags: ['#pheromone', '#etat', '#presence', '#signature'],
    color: '#99ffdd',
    config: {
      historyDepth: 20,
      history: []
    }
  },
  {
    id: 'return-to-nexus',
    type: 'trigger',
    role: 'Ramene les nodes en orbite vers le centre',
    triggers: ['on orbit-cycle-end', 'on force-return'],
    provides: ['feedback-pulse', 'cycle-data'],
    requires: ['orbit-path', 'staggered-entry-schedule'],
    position: { x: 0.8, y: 0.65 },
    layer: 'core',
    tags: ['#retour', '#feedback', '#boucle', '#cycle'],
    color: '#99ddff',
    config: {
      gentleness: 0.9,
      spiral: true
    }
  },
  {
    id: 'orbit-choreographer',
    type: 'logic',
    role: 'Regule le timing d\'entree pour eviter la collision au nexus',
    triggers: ['on pull-signal', 'on multiple-nodes-approaching'],
    provides: ['staggered-entry-schedule', 'orbit-layer-assignment'],
    requires: ['pull-signal', 'presence-signature'],
    position: { x: 0.5, y: 0.2 },
    layer: 'core',
    tags: ['#regulation', '#timing', '#choregraphie', '#flux'],
    color: '#ffdd99',
    config: {
      maxSimultaneous: 5,
      layers: ['proche', 'moyen', 'lointain']
    }
  },
  {
    id: 'nexus-memory-scribe',
    type: 'state',
    role: 'Archive les patterns de resonance pour apprentissage',
    triggers: ['on resonance-peak', 'on orbit-cycle-end', 'on periodic-save'],
    provides: ['learned-patterns', 'frequently-activated-paths', 'weak-link-alerts'],
    requires: ['resonance-history', 'feedback-pulse'],
    position: { x: 0.15, y: 0.45 },
    layer: 'meta',
    tags: ['#memoire', '#apprentissage', '#patterns', '#archive'],
    color: '#99ddff',
    config: {
      retentionCycles: 100,
      learningRate: 0.1
    }
  },
  {
    id: 'peripheral-radar',
    type: 'input',
    role: 'Detecte les nodes distants qui pourraient se connecter',
    triggers: ['on system-scan', 'on new-node-creation'],
    provides: ['distant-node-signals', 'potential-connections'],
    requires: ['gravity-field'],
    position: { x: 0.85, y: 0.45 },
    layer: 'core',
    tags: ['#detection', '#peripherie', '#scan', '#input', '#radar'],
    color: '#dd99ff',
    config: {
      scanRadius: 'infinite',
      scanInterval: '5s'
    }
  },
  {
    id: 'activation-threshold',
    type: 'logic',
    role: 'Decide si une resonance est assez forte pour activer',
    triggers: ['on resonance-strength change'],
    provides: ['activation-decision', 'strength-percentage', 'recommendation'],
    requires: ['resonance-strength', 'alignment-level', 'presence-signature'],
    position: { x: 0.5, y: 0.8 },
    layer: 'core',
    tags: ['#filtre', '#decision', '#seuil', '#logic', '#regulation'],
    color: '#ffdd99',
    config: {
      minStrength: 30,
      optimalStrength: 70,
      maxSimultaneous: 5
    }
  }
];

const NEXUS_CONNECTIONS = [
  { from: 'nexus-core', to: 'nexus-pheromone', label: 'emet', type: 'provide' },
  { from: 'nexus-core', to: 'orbit-invitation', label: 'emet', type: 'provide' },
  { from: 'nexus-core', to: 'peripheral-radar', label: 'emet gravite', type: 'provide' },
  { from: 'keyword-resonance', to: 'nexus-core', label: 'renforce', type: 'feedback' },
  { from: 'keyword-resonance', to: 'activation-threshold', label: 'strength', type: 'data' },
  { from: 'activation-threshold', to: 'orbit-invitation', label: 'decision', type: 'control' },
  { from: 'orbit-invitation', to: 'orbit-choreographer', label: 'pull coordonne', type: 'control' },
  { from: 'orbit-choreographer', to: 'return-to-nexus', label: 'timing', type: 'control' },
  { from: 'return-to-nexus', to: 'nexus-core', label: 'retour', type: 'feedback' },
  { from: 'return-to-nexus', to: 'nexus-memory-scribe', label: 'patterns', type: 'data' },
  { from: 'nexus-pheromone', to: 'nexus-memory-scribe', label: 'history', type: 'data' },
  { from: 'nexus-pheromone', to: 'activation-threshold', label: 'state', type: 'data' },
  { from: 'peripheral-radar', to: 'keyword-resonance', label: 'signaux', type: 'data' },
  { from: 'nexus-memory-scribe', to: 'keyword-resonance', label: 'learned', type: 'feedback' },
];

const TYPE_STYLES = {
  concept: { border: '#99ff99', bg: '#99ff9920' },
  logic:   { border: '#ffdd99', bg: '#ffdd9920' },
  trigger: { border: '#ff9999', bg: '#ff999920' },
  state:   { border: '#99ddff', bg: '#99ddff20' },
  input:   { border: '#dd99ff', bg: '#dd99ff20' },
};

const CONNECTION_COLORS = {
  provide: '#66ffaa',
  feedback: '#ff66aa',
  data: '#66aaff',
  control: '#ffaa66',
};

// ═══════════════════════════════════════════════════════════════
// ACTIVATION SCENARIO STEPS
// ═══════════════════════════════════════════════════════════════

const ACTIVATION_SCENARIO = [
  {
    phase: 'Demarrage',
    steps: [
      { node: 'nexus-core', action: 'demarre', detail: 'Emet gravity-field (639 Hz)', strength: 100 },
      { node: 'peripheral-radar', action: 'active', detail: 'Scanne l\'environnement', strength: 40 },
      { node: 'nexus-pheromone', action: 'pulse', detail: 'Commence a pulser la presence-signature', strength: 30 },
    ]
  },
  {
    phase: 'Stimulus: "mouvement"',
    steps: [
      { node: 'keyword-resonance', action: 'detecte', detail: 'Mot "mouvement" -> synonyms ["deplacement", "action", "velocite"]', strength: 65 },
      { node: 'activation-threshold', action: 'evalue', detail: '65 > 30 (min) -> simultaneous: 2 < 5 -> decision: "activate"', strength: 65 },
      { node: 'orbit-invitation', action: 'emet', detail: 'Pull-signal emis vers nodes distants', strength: 80 },
      { node: 'orbit-choreographer', action: 'assigne', detail: 'Orbit-layer: "moyen" - timing calcule', strength: 50 },
    ]
  },
  {
    phase: 'Completion',
    steps: [
      { node: 'return-to-nexus', action: 'ramene', detail: 'Feedback-pulse retourne au centre', strength: 70 },
      { node: 'nexus-memory-scribe', action: 'archive', detail: '"mouvement" -> force 65 -> succes', strength: 90 },
      { node: 'nexus-pheromone', action: 'update', detail: 'Integre "mouvement" dans patterns actifs', strength: 60 },
      { node: 'nexus-core', action: 'absorbe', detail: 'Cycle complete - resonance integree', strength: 100 },
    ]
  }
];

// ═══════════════════════════════════════════════════════════════
// MODE 1: VISUALISATION 3D INTERACTIVE
// ═══════════════════════════════════════════════════════════════

const Mode3DSpiral = () => {
  const canvasRef = useRef(null);
  const [rotation, setRotation] = useState(0);
  const [zoom, setZoom] = useState(1);
  const [selectedRing, setSelectedRing] = useState(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    const width = canvas.width;
    const height = canvas.height;
    const centerX = width / 2;
    const centerY = height / 2;

    const draw = () => {
      ctx.fillStyle = '#0a0a0f';
      ctx.fillRect(0, 0, width, height);

      const pulseSize = 10 + Math.sin(Date.now() * 0.003) * 5;
      const gradient = ctx.createRadialGradient(centerX, centerY, 0, centerX, centerY, pulseSize * zoom);
      gradient.addColorStop(0, SPIRAL_DATA.center.color);
      gradient.addColorStop(0.5, SPIRAL_DATA.center.color + '80');
      gradient.addColorStop(1, 'transparent');
      ctx.fillStyle = gradient;

      ctx.beginPath();
      ctx.arc(centerX, centerY, pulseSize * zoom, 0, Math.PI * 2);
      ctx.fill();

      SPIRAL_DATA.rings.forEach((ring, index) => {
        const radius = (50 + index * 40) * zoom;
        const points = 64;

        ctx.strokeStyle = ring.color + (selectedRing === index ? 'ff' : '60');
        ctx.lineWidth = selectedRing === index ? 3 : 1.5;
        ctx.beginPath();

        for (let i = 0; i <= points; i++) {
          const angle = (i / points) * Math.PI * 2 + rotation + (index * 0.3);
          const spiralRadius = radius + Math.sin(i * 0.1) * 10 * zoom;
          const x = centerX + Math.cos(angle) * spiralRadius;
          const y = centerY + Math.sin(angle) * spiralRadius;

          if (i === 0) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        }
        ctx.stroke();

        ring.concepts.forEach((concept, cIndex) => {
          const angle = (cIndex / ring.concepts.length) * Math.PI * 2 + rotation;
          const x = centerX + Math.cos(angle) * radius;
          const y = centerY + Math.sin(angle) * radius;

          ctx.fillStyle = ring.color;
          ctx.beginPath();
          ctx.arc(x, y, 4 * zoom, 0, Math.PI * 2);
          ctx.fill();

          if (selectedRing === index) {
            ctx.fillStyle = '#ffffff';
            ctx.font = `${12 * zoom}px monospace`;
            ctx.textAlign = 'center';
            ctx.fillText(concept, x, y - 10 * zoom);
          }
        });
      });
    };

    draw();
    const interval = setInterval(() => {
      setRotation(r => r + 0.005);
      draw();
    }, 30);

    return () => clearInterval(interval);
  }, [rotation, zoom, selectedRing]);

  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 relative">
        <canvas
          ref={canvasRef}
          width={800}
          height={600}
          className="w-full h-full cursor-pointer"
          onClick={(e) => {
            const rect = e.currentTarget.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            const cx = rect.width / 2;
            const cy = rect.height / 2;
            const distance = Math.sqrt((x - cx) ** 2 + (y - cy) ** 2);
            const ringIndex = Math.floor(distance / (40 * zoom)) - 1;
            setSelectedRing(ringIndex >= 0 && ringIndex < SPIRAL_DATA.rings.length ? ringIndex : null);
          }}
        />
      </div>
      <div className="p-4 bg-gray-900/50 border-t border-cyan-500/30">
        <div className="flex gap-4 items-center">
          <label className="text-cyan-400">Zoom:</label>
          <input
            type="range"
            min="0.5"
            max="2"
            step="0.1"
            value={zoom}
            onChange={(e) => setZoom(parseFloat(e.target.value))}
            className="flex-1"
          />
          <span className="text-cyan-400">{zoom.toFixed(1)}x</span>
        </div>
        {selectedRing !== null && (
          <div className="mt-2 text-sm">
            <span className="text-yellow-400">Anneau selectionne: </span>
            <span className="text-white">{SPIRAL_DATA.rings[selectedRing].theme}</span>
          </div>
        )}
      </div>
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════
// MODE 2: CARTE VIVANTE DE L'ARCHIVE
// ═══════════════════════════════════════════════════════════════

const ModeArchiveMap = () => {
  const [filter, setFilter] = useState('all');
  const [hoveredConcept, setHoveredConcept] = useState(null);

  const themes = ['all', ...new Set(SPIRAL_DATA.rings.map(r => r.theme))];

  const filteredRings = filter === 'all'
    ? SPIRAL_DATA.rings
    : SPIRAL_DATA.rings.filter(r => r.theme === filter);

  return (
    <div className="h-full flex flex-col p-6 overflow-auto">
      <div className="mb-6 flex gap-2 flex-wrap">
        {themes.map(theme => (
          <button
            key={theme}
            onClick={() => setFilter(theme)}
            className={`px-4 py-2 rounded ${
              filter === theme
                ? 'bg-cyan-500 text-black'
                : 'bg-gray-800 text-cyan-400 hover:bg-gray-700'
            }`}
          >
            {theme.toUpperCase()}
          </button>
        ))}
      </div>

      <div className="space-y-6">
        <div className="text-center p-6 bg-cyan-500/10 border-2 border-cyan-500 rounded-lg">
          <h3 className="text-2xl font-bold text-cyan-400 mb-3">{SPIRAL_DATA.center.name}</h3>
          <div className="flex gap-3 justify-center flex-wrap">
            {SPIRAL_DATA.center.concepts.map(c => (
              <span key={c} className="px-3 py-1 bg-cyan-500/20 text-cyan-300 rounded text-sm">
                {c}
              </span>
            ))}
          </div>
        </div>

        {filteredRings.map((ring) => (
          <div
            key={ring.level}
            className="p-4 bg-gray-900/50 border-l-4 rounded"
            style={{ borderColor: ring.color }}
          >
            <div className="flex items-center gap-3 mb-3">
              <div
                className="w-4 h-4 rounded-full"
                style={{ backgroundColor: ring.color }}
              />
              <h4 className="text-lg font-bold" style={{ color: ring.color }}>
                Niveau {ring.level} - {ring.theme}
              </h4>
            </div>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-2">
              {ring.concepts.map(concept => (
                <div
                  key={concept}
                  className="p-2 bg-gray-800/50 rounded cursor-pointer hover:bg-gray-700 transition-all"
                  onMouseEnter={() => setHoveredConcept(concept)}
                  onMouseLeave={() => setHoveredConcept(null)}
                  style={{
                    borderLeft: hoveredConcept === concept ? `3px solid ${ring.color}` : 'none'
                  }}
                >
                  <span className="text-sm text-gray-300">{concept}</span>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>

      {hoveredConcept && (
        <div className="fixed bottom-4 right-4 bg-black/90 border border-cyan-500 p-4 rounded max-w-xs">
          <p className="text-cyan-400 text-sm">
            Concept: <span className="text-white font-bold">{hoveredConcept}</span>
          </p>
          <p className="text-gray-400 text-xs mt-1">
            Cliquez pour explorer les connexions
          </p>
        </div>
      )}
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════
// MODE 3: GÉNÉRATEUR DE SPIRALES ASCII
// ═══════════════════════════════════════════════════════════════

const ModeASCIIGenerator = () => {
  const [centerText, setCenterText] = useState('NOYAU');
  const [layers, setLayers] = useState(['Concept A', 'Concept B', 'Concept C']);
  const [newLayer, setNewLayer] = useState('');
  const [asciiOutput, setAsciiOutput] = useState('');

  const generateASCII = useCallback(() => {
    let output = '';
    const indent = ' '.repeat(20);

    if (layers.length > 0) {
      output += indent + `... ${layers[layers.length - 1]} ...\n`;
      output += indent + '/' + ' '.repeat(30) + '\\\n';
    }

    for (let i = layers.length - 2; i >= 0; i--) {
      const spacing = ' '.repeat(20 - i * 3);
      output += spacing + layers[i] + '\n';
      if (i > 0) output += spacing + '/' + ' '.repeat(15 + i * 5) + '\\\n';
    }

    output += ' '.repeat(10) + `=== ${centerText} ===\n`;

    setAsciiOutput(output);
  }, [centerText, layers]);

  useEffect(() => {
    generateASCII();
  }, [generateASCII]);

  return (
    <div className="h-full flex flex-col p-6">
      <div className="mb-6 space-y-4">
        <div>
          <label className="block text-cyan-400 mb-2">Centre (Noyau):</label>
          <input
            type="text"
            value={centerText}
            onChange={(e) => setCenterText(e.target.value)}
            className="w-full bg-gray-900 border border-cyan-500/50 rounded px-4 py-2 text-white"
            placeholder="Ex: LUNA, Petit 0, etc."
          />
        </div>

        <div>
          <label className="block text-cyan-400 mb-2">Ajouter une couche:</label>
          <div className="flex gap-2">
            <input
              type="text"
              value={newLayer}
              onChange={(e) => setNewLayer(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && newLayer.trim()) {
                  setLayers([...layers, newLayer.trim()]);
                  setNewLayer('');
                }
              }}
              className="flex-1 bg-gray-900 border border-cyan-500/50 rounded px-4 py-2 text-white"
              placeholder="Ex: Cybernetique"
            />
            <button
              onClick={() => {
                if (newLayer.trim()) {
                  setLayers([...layers, newLayer.trim()]);
                  setNewLayer('');
                }
              }}
              className="px-4 py-2 bg-cyan-500 text-black rounded hover:bg-cyan-400"
            >
              Ajouter
            </button>
          </div>
        </div>

        <div className="flex flex-wrap gap-2">
          {layers.map((layer, i) => (
            <div key={i} className="px-3 py-1 bg-gray-800 text-gray-300 rounded flex items-center gap-2">
              <span>{layer}</span>
              <button
                onClick={() => setLayers(layers.filter((_, idx) => idx !== i))}
                className="text-red-400 hover:text-red-300"
              >
                x
              </button>
            </div>
          ))}
        </div>
      </div>

      <div className="flex-1 bg-black/50 border border-cyan-500/30 rounded p-4 overflow-auto">
        <pre className="text-cyan-400 font-mono text-sm whitespace-pre">{asciiOutput}</pre>
      </div>

      <button
        onClick={() => navigator.clipboard.writeText(asciiOutput)}
        className="mt-4 px-4 py-2 bg-purple-600 text-white rounded hover:bg-purple-500"
      >
        Copier ASCII
      </button>
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════
// MODE 4: MAG GS SPIRAL MODE
// ═══════════════════════════════════════════════════════════════

const ModeMAGGS = () => {
  const [nodes, setNodes] = useState([
    { id: 'NEXUS-001', flux: 'Zero', level: 0 },
    { id: 'LUNA-CORE', flux: 'Zero', level: 1 },
    { id: 'MODULARIS-ROOT', flux: 'Zero', level: 2 }
  ]);
  const [evaluating, setEvaluating] = useState(false);
  const [waveProgress, setWaveProgress] = useState(0);

  const evaluateSpiral = async () => {
    setEvaluating(true);
    setWaveProgress(0);

    for (let i = 0; i < nodes.length; i++) {
      await new Promise(resolve => setTimeout(resolve, 500));

      setNodes(prev => prev.map((node, idx) =>
        idx === i ? { ...node, flux: 'Unite' } : node
      ));

      setWaveProgress(((i + 1) / nodes.length) * 100);
    }

    setEvaluating(false);
  };

  const resetSpiral = () => {
    setNodes(nodes.map(n => ({ ...n, flux: 'Zero' })));
    setWaveProgress(0);
  };

  return (
    <div className="h-full flex flex-col p-6">
      <div className="mb-6">
        <h3 className="text-xl font-bold text-cyan-400 mb-2">MAG GS - Evaluation Spiralee</h3>
        <p className="text-gray-400 text-sm">
          Les noeuds s'evaluent en ondes concentriques depuis le Petit 0
        </p>
      </div>

      <div className="flex-1 flex items-center justify-center">
        <div className="relative w-96 h-96">
          <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-10">
            <div className={`w-16 h-16 rounded-full flex items-center justify-center border-2 ${
              evaluating ? 'animate-pulse border-cyan-400 bg-cyan-500/20' : 'border-gray-600 bg-gray-800'
            }`}>
              <span className="text-xs text-cyan-400 font-mono">Petit 0</span>
            </div>
          </div>

          {nodes.map((node, index) => {
            const angle = (index / nodes.length) * Math.PI * 2;
            const radius = 80 + node.level * 40;
            const x = 50 + Math.cos(angle) * (radius / 200) * 50;
            const y = 50 + Math.sin(angle) * (radius / 200) * 50;

            return (
              <div
                key={node.id}
                className="absolute"
                style={{
                  left: `${x}%`,
                  top: `${y}%`,
                  transform: 'translate(-50%, -50%)'
                }}
              >
                <div className={`w-12 h-12 rounded-full flex flex-col items-center justify-center border-2 transition-all ${
                  node.flux === 'Unite'
                    ? 'border-green-400 bg-green-500/20 shadow-lg shadow-green-500/50'
                    : 'border-purple-600 bg-purple-900/20'
                }`}>
                  <span className="text-xs text-white font-mono">{node.id.split('-')[0]}</span>
                  <span className={`text-xs font-bold ${
                    node.flux === 'Unite' ? 'text-green-400' : 'text-gray-500'
                  }`}>
                    {node.flux}
                  </span>
                </div>
              </div>
            );
          })}

          {evaluating && (
            <div
              className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-cyan-400 pointer-events-none"
              style={{
                width: `${waveProgress * 3}px`,
                height: `${waveProgress * 3}px`,
                opacity: 1 - waveProgress / 100
              }}
            />
          )}
        </div>
      </div>

      <div className="space-y-4">
        <div className="flex gap-4">
          <button
            onClick={evaluateSpiral}
            disabled={evaluating}
            className="flex-1 px-4 py-3 bg-cyan-500 text-black rounded hover:bg-cyan-400 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            {evaluating ? <Pause size={16} /> : <PlayCircle size={16} />}
            {evaluating ? 'Evaluation...' : 'Lancer Evaluation'}
          </button>
          <button
            onClick={resetSpiral}
            disabled={evaluating}
            className="px-4 py-3 bg-gray-700 text-white rounded hover:bg-gray-600 disabled:opacity-50"
          >
            Reset
          </button>
        </div>

        <div className="bg-gray-900/50 border border-cyan-500/30 rounded p-4">
          <div className="flex justify-between text-sm mb-2">
            <span className="text-gray-400">Progression de la vague:</span>
            <span className="text-cyan-400">{waveProgress.toFixed(0)}%</span>
          </div>
          <div className="h-2 bg-gray-800 rounded overflow-hidden">
            <div
              className="h-full bg-gradient-to-r from-cyan-500 to-green-400 transition-all"
              style={{ width: `${waveProgress}%` }}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════
// MODE 5: ARCHIVE INTERACTIVE "SPIRAL MEMORY"
// ═══════════════════════════════════════════════════════════════

const ModeSpiralMemory = () => {
  const [timelinePos, setTimelinePos] = useState(0);
  const [selectedPhase, setSelectedPhase] = useState(null);

  const phases = SPIRAL_DATA.rings.map((ring, index) => ({
    id: index,
    level: ring.level,
    theme: ring.theme,
    concepts: ring.concepts,
    color: ring.color,
    timestamp: `Phase ${index + 1}`,
    description: `Exploration de ${ring.theme.toLowerCase()} avec ${ring.concepts.length} concepts cles`
  }));

  return (
    <div className="h-full flex flex-col p-6">
      <div className="mb-6">
        <h3 className="text-xl font-bold text-cyan-400 mb-2">Spiral Memory</h3>
        <p className="text-gray-400 text-sm">Navigation circulaire dans votre parcours creatif</p>
      </div>

      <div className="mb-6">
        <div className="flex items-center gap-4 mb-2">
          <span className="text-cyan-400 text-sm">Position temporelle:</span>
          <input
            type="range"
            min="0"
            max={phases.length - 1}
            step="1"
            value={timelinePos}
            onChange={(e) => {
              const pos = parseInt(e.target.value);
              setTimelinePos(pos);
              setSelectedPhase(phases[pos]);
            }}
            className="flex-1"
          />
          <span className="text-white font-mono">{phases[timelinePos]?.timestamp}</span>
        </div>
        <div className="flex gap-1">
          {phases.map((phase, i) => (
            <div
              key={i}
              className="flex-1 h-2 rounded cursor-pointer transition-all"
              style={{
                backgroundColor: i === timelinePos ? phase.color : phase.color + '30',
                transform: i === timelinePos ? 'scaleY(1.5)' : 'scaleY(1)'
              }}
              onClick={() => {
                setTimelinePos(i);
                setSelectedPhase(phase);
              }}
            />
          ))}
        </div>
      </div>

      {selectedPhase && (
        <div
          className="flex-1 bg-gray-900/50 border-l-4 rounded p-6 overflow-auto"
          style={{ borderColor: selectedPhase.color }}
        >
          <div className="flex items-center gap-3 mb-4">
            <div
              className="w-6 h-6 rounded-full"
              style={{ backgroundColor: selectedPhase.color }}
            />
            <h4 className="text-2xl font-bold" style={{ color: selectedPhase.color }}>
              {selectedPhase.theme}
            </h4>
            <span className="text-gray-500 text-sm">Niveau {selectedPhase.level}</span>
          </div>

          <p className="text-gray-400 mb-6">{selectedPhase.description}</p>

          <div className="grid grid-cols-2 gap-3">
            {selectedPhase.concepts.map(concept => (
              <div
                key={concept}
                className="p-4 bg-black/30 border rounded hover:bg-black/50 transition-all cursor-pointer"
                style={{ borderColor: selectedPhase.color + '50' }}
              >
                <span className="text-white font-mono">{concept}</span>
              </div>
            ))}
          </div>

          <div className="mt-6 p-4 bg-cyan-500/10 border border-cyan-500/30 rounded">
            <p className="text-cyan-400 text-sm">
              Cette phase fait partie d'un cycle qui revient toujours au noyau (LUNA / 639 Hz)
            </p>
          </div>
        </div>
      )}

      {!selectedPhase && (
        <div className="flex-1 flex items-center justify-center">
          <p className="text-gray-500">Deplacez le curseur pour explorer les phases</p>
        </div>
      )}
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════
// MODE 6: NEXUS GRAPH - Graphe interactif des 8 nodes
// ═══════════════════════════════════════════════════════════════

const ModeNexusGraph = () => {
  const canvasRef = useRef(null);
  const [selectedNode, setSelectedNode] = useState(null);
  const [activeNodes, setActiveNodes] = useState(new Set());
  const [activeEdges, setActiveEdges] = useState(new Set());
  const [time, setTime] = useState(0);

  const getNodePixelPos = useCallback((node, width, height) => {
    const padding = 60;
    return {
      x: padding + node.position.x * (width - padding * 2),
      y: padding + node.position.y * (height - padding * 2)
    };
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    const width = canvas.width;
    const height = canvas.height;

    const draw = () => {
      ctx.fillStyle = '#060610';
      ctx.fillRect(0, 0, width, height);

      // Draw grid
      ctx.strokeStyle = '#ffffff08';
      ctx.lineWidth = 1;
      for (let x = 0; x < width; x += 40) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, height);
        ctx.stroke();
      }
      for (let y = 0; y < height; y += 40) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();
      }

      // Draw connections
      NEXUS_CONNECTIONS.forEach((conn, ci) => {
        const fromNode = NEXUS_NODES.find(n => n.id === conn.from);
        const toNode = NEXUS_NODES.find(n => n.id === conn.to);
        if (!fromNode || !toNode) return;

        const from = getNodePixelPos(fromNode, width, height);
        const to = getNodePixelPos(toNode, width, height);
        const isActive = activeEdges.has(ci);
        const baseColor = CONNECTION_COLORS[conn.type] || '#666666';

        // Connection line
        ctx.strokeStyle = isActive ? baseColor : baseColor + '40';
        ctx.lineWidth = isActive ? 3 : 1.5;
        ctx.setLineDash(conn.type === 'feedback' ? [6, 4] : []);
        ctx.beginPath();
        ctx.moveTo(from.x, from.y);

        // Curved connection
        const mx = (from.x + to.x) / 2;
        const my = (from.y + to.y) / 2;
        const dx = to.x - from.x;
        const dy = to.y - from.y;
        const cx = mx - dy * 0.15;
        const cy = my + dx * 0.15;
        ctx.quadraticCurveTo(cx, cy, to.x, to.y);
        ctx.stroke();
        ctx.setLineDash([]);

        // Arrowhead
        const angle = Math.atan2(to.y - cy, to.x - cx);
        const arrowLen = 10;
        ctx.fillStyle = isActive ? baseColor : baseColor + '60';
        ctx.beginPath();
        ctx.moveTo(to.x, to.y);
        ctx.lineTo(to.x - arrowLen * Math.cos(angle - 0.3), to.y - arrowLen * Math.sin(angle - 0.3));
        ctx.lineTo(to.x - arrowLen * Math.cos(angle + 0.3), to.y - arrowLen * Math.sin(angle + 0.3));
        ctx.closePath();
        ctx.fill();

        // Connection label
        if (isActive) {
          ctx.fillStyle = '#ffffff';
          ctx.font = '10px monospace';
          ctx.textAlign = 'center';
          ctx.fillText(conn.label, cx, cy - 8);
        }

        // Animated pulse along active edges
        if (isActive) {
          const pulseT = (time * 0.02) % 1;
          const pt = 1 - pulseT;
          const px = from.x * pt * pt + cx * 2 * pt * pulseT + to.x * pulseT * pulseT;
          const py = from.y * pt * pt + cy * 2 * pt * pulseT + to.y * pulseT * pulseT;
          ctx.beginPath();
          ctx.arc(px, py, 4, 0, Math.PI * 2);
          ctx.fillStyle = baseColor;
          ctx.fill();
        }
      });

      // Draw nodes
      NEXUS_NODES.forEach(node => {
        const pos = getNodePixelPos(node, width, height);
        const isActive = activeNodes.has(node.id);
        const isSelected = selectedNode === node.id;
        const style = TYPE_STYLES[node.type] || TYPE_STYLES.concept;
        const nodeRadius = isSelected ? 32 : 26;

        // Glow for active nodes
        if (isActive) {
          const glow = ctx.createRadialGradient(pos.x, pos.y, nodeRadius, pos.x, pos.y, nodeRadius * 2.5);
          glow.addColorStop(0, node.color + '40');
          glow.addColorStop(1, 'transparent');
          ctx.fillStyle = glow;
          ctx.beginPath();
          ctx.arc(pos.x, pos.y, nodeRadius * 2.5, 0, Math.PI * 2);
          ctx.fill();
        }

        // Node circle
        ctx.beginPath();
        ctx.arc(pos.x, pos.y, nodeRadius, 0, Math.PI * 2);
        ctx.fillStyle = isActive ? node.color + '30' : '#0a0a1a';
        ctx.fill();
        ctx.strokeStyle = isSelected ? '#ffffff' : isActive ? node.color : style.border + '80';
        ctx.lineWidth = isSelected ? 3 : 2;
        ctx.stroke();

        // Pulsing ring for nexus-core
        if (node.id === 'nexus-core') {
          const pulseR = nodeRadius + Math.sin(time * 0.05) * 8 + 10;
          ctx.beginPath();
          ctx.arc(pos.x, pos.y, pulseR, 0, Math.PI * 2);
          ctx.strokeStyle = node.color + '30';
          ctx.lineWidth = 1;
          ctx.stroke();
        }

        // Node label
        ctx.fillStyle = isActive ? '#ffffff' : '#aaaaaa';
        ctx.font = `bold ${isSelected ? 11 : 9}px monospace`;
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';

        const label = node.id.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ');
        const shortLabel = label.length > 12 ? label.substring(0, 11) + '..' : label;
        ctx.fillText(shortLabel, pos.x, pos.y - 5);

        // Type badge
        ctx.fillStyle = style.border;
        ctx.font = '8px monospace';
        ctx.fillText(node.type, pos.x, pos.y + 8);
      });

      // Legend
      ctx.fillStyle = '#ffffff40';
      ctx.font = '10px monospace';
      ctx.textAlign = 'left';
      let ly = height - 70;
      ctx.fillText('Connexions:', 10, ly);
      ly += 14;
      Object.entries(CONNECTION_COLORS).forEach(([type, color]) => {
        ctx.fillStyle = color;
        ctx.fillRect(10, ly - 4, 12, 3);
        ctx.fillStyle = '#aaaaaa';
        ctx.fillText(type, 28, ly);
        ly += 12;
      });
    };

    draw();
    const interval = setInterval(() => {
      setTime(t => t + 1);
    }, 50);

    return () => clearInterval(interval);
  }, [time, selectedNode, activeNodes, activeEdges, getNodePixelPos]);

  const handleCanvasClick = (e) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    const mx = (e.clientX - rect.left) * scaleX;
    const my = (e.clientY - rect.top) * scaleY;

    let clicked = null;
    for (const node of NEXUS_NODES) {
      const pos = getNodePixelPos(node, canvas.width, canvas.height);
      const dist = Math.sqrt((mx - pos.x) ** 2 + (my - pos.y) ** 2);
      if (dist < 30) {
        clicked = node.id;
        break;
      }
    }

    setSelectedNode(clicked);

    if (clicked) {
      const newActiveNodes = new Set([clicked]);
      const newActiveEdges = new Set();
      NEXUS_CONNECTIONS.forEach((conn, i) => {
        if (conn.from === clicked || conn.to === clicked) {
          newActiveEdges.add(i);
          newActiveNodes.add(conn.from);
          newActiveNodes.add(conn.to);
        }
      });
      setActiveNodes(newActiveNodes);
      setActiveEdges(newActiveEdges);
    } else {
      setActiveNodes(new Set());
      setActiveEdges(new Set());
    }
  };

  const selectedNodeData = NEXUS_NODES.find(n => n.id === selectedNode);

  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 relative flex">
        <canvas
          ref={canvasRef}
          width={800}
          height={550}
          className="flex-1 cursor-pointer"
          onClick={handleCanvasClick}
        />
      </div>

      {/* Node detail panel */}
      <div className="h-48 bg-gray-900/80 border-t border-cyan-500/30 p-4 overflow-auto">
        {selectedNodeData ? (
          <div className="flex gap-6">
            <div className="flex-1">
              <div className="flex items-center gap-3 mb-2">
                <div className="w-4 h-4 rounded-full" style={{ backgroundColor: selectedNodeData.color }} />
                <h4 className="text-lg font-bold font-mono" style={{ color: selectedNodeData.color }}>
                  {selectedNodeData.id}
                </h4>
                <span
                  className="px-2 py-0.5 rounded text-xs font-mono"
                  style={{
                    backgroundColor: TYPE_STYLES[selectedNodeData.type]?.bg,
                    color: TYPE_STYLES[selectedNodeData.type]?.border
                  }}
                >
                  {selectedNodeData.type}
                </span>
                <span className="text-gray-500 text-xs">[{selectedNodeData.layer}]</span>
              </div>
              <p className="text-gray-400 text-sm mb-3">{selectedNodeData.role}</p>
              <div className="flex flex-wrap gap-1">
                {selectedNodeData.tags.map(tag => (
                  <span key={tag} className="px-2 py-0.5 bg-gray-800 text-gray-400 rounded text-xs font-mono">
                    {tag}
                  </span>
                ))}
              </div>
            </div>
            <div className="flex-1 grid grid-cols-2 gap-3 text-xs">
              <div>
                <h5 className="text-green-400 font-bold mb-1">PROVIDES</h5>
                {selectedNodeData.provides.map(p => (
                  <div key={p} className="text-green-300/70 font-mono">+ {p}</div>
                ))}
              </div>
              <div>
                <h5 className="text-orange-400 font-bold mb-1">REQUIRES</h5>
                {selectedNodeData.requires.length > 0 ? selectedNodeData.requires.map(r => (
                  <div key={r} className="text-orange-300/70 font-mono">- {r}</div>
                )) : <div className="text-gray-600 font-mono italic">aucun (source)</div>}
              </div>
              <div className="col-span-2">
                <h5 className="text-purple-400 font-bold mb-1">TRIGGERS</h5>
                {selectedNodeData.triggers.map(t => (
                  <span key={t} className="mr-2 text-purple-300/70 font-mono">{t}</span>
                ))}
              </div>
            </div>
          </div>
        ) : (
          <div className="h-full flex items-center justify-center">
            <p className="text-gray-500 font-mono text-sm">
              Cliquez sur un node pour voir ses details et connexions
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════
// MODE 7: ACTIVATION CASCADE SIMULATOR
// ═══════════════════════════════════════════════════════════════

const ModeActivationCascade = () => {
  const [running, setRunning] = useState(false);
  const [currentPhase, setCurrentPhase] = useState(-1);
  const [currentStep, setCurrentStep] = useState(-1);
  const [logs, setLogs] = useState([]);
  const [nodeStates, setNodeStates] = useState({});
  const [resonanceHistory, setResonanceHistory] = useState([]);
  const logsEndRef = useRef(null);

  const scrollToBottom = useCallback(() => {
    logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [logs, scrollToBottom]);

  const runSimulation = async () => {
    setRunning(true);
    setLogs([]);
    setNodeStates({});
    setResonanceHistory([]);
    setCurrentPhase(-1);
    setCurrentStep(-1);

    for (let pi = 0; pi < ACTIVATION_SCENARIO.length; pi++) {
      const phase = ACTIVATION_SCENARIO[pi];
      setCurrentPhase(pi);

      setLogs(prev => [...prev, {
        type: 'phase',
        text: `=== PHASE: ${phase.phase} ===`,
        timestamp: Date.now()
      }]);

      await new Promise(r => setTimeout(r, 600));

      for (let si = 0; si < phase.steps.length; si++) {
        const step = phase.steps[si];
        setCurrentStep(si);

        setNodeStates(prev => ({
          ...prev,
          [step.node]: { active: true, action: step.action, strength: step.strength }
        }));

        setLogs(prev => [...prev, {
          type: 'action',
          node: step.node,
          action: step.action,
          detail: step.detail,
          strength: step.strength,
          timestamp: Date.now()
        }]);

        setResonanceHistory(prev => [...prev, {
          node: step.node,
          strength: step.strength,
          phase: phase.phase,
          time: Date.now()
        }]);

        await new Promise(r => setTimeout(r, 800));
      }

      await new Promise(r => setTimeout(r, 400));
    }

    setLogs(prev => [...prev, {
      type: 'complete',
      text: 'Cycle complet - Resonance integree au noyau',
      timestamp: Date.now()
    }]);

    setRunning(false);
  };

  const reset = () => {
    setRunning(false);
    setCurrentPhase(-1);
    setCurrentStep(-1);
    setLogs([]);
    setNodeStates({});
    setResonanceHistory([]);
  };

  const maxStrength = Math.max(1, ...resonanceHistory.map(h => h.strength));

  return (
    <div className="h-full flex flex-col p-6">
      <div className="mb-4">
        <h3 className="text-xl font-bold text-cyan-400 mb-1">Simulation de Cascade d'Activation</h3>
        <p className="text-gray-400 text-sm">
          Scenario complet : Demarrage → Stimulus "mouvement" → Completion
        </p>
      </div>

      <div className="flex-1 flex gap-4 min-h-0">
        {/* Left: Node status grid */}
        <div className="w-64 flex-shrink-0 space-y-2 overflow-auto">
          <h4 className="text-sm font-bold text-gray-400 mb-2">ETAT DES NODES</h4>
          {NEXUS_NODES.map(node => {
            const state = nodeStates[node.id];
            const isActive = state?.active;
            return (
              <div
                key={node.id}
                className="p-2 rounded border transition-all"
                style={{
                  borderColor: isActive ? node.color : '#333',
                  backgroundColor: isActive ? node.color + '15' : '#111'
                }}
              >
                <div className="flex items-center gap-2">
                  <div
                    className="w-2 h-2 rounded-full transition-all"
                    style={{ backgroundColor: isActive ? node.color : '#444' }}
                  />
                  <span className="text-xs font-mono" style={{ color: isActive ? node.color : '#666' }}>
                    {node.id}
                  </span>
                </div>
                {isActive && (
                  <div className="mt-1 ml-4">
                    <div className="text-xs text-gray-400">{state.action}</div>
                    <div className="mt-1 h-1 bg-gray-800 rounded overflow-hidden">
                      <div
                        className="h-full rounded transition-all"
                        style={{
                          width: `${state.strength}%`,
                          backgroundColor: node.color
                        }}
                      />
                    </div>
                  </div>
                )}
              </div>
            );
          })}
        </div>

        {/* Center: Logs */}
        <div className="flex-1 flex flex-col min-w-0">
          <h4 className="text-sm font-bold text-gray-400 mb-2">JOURNAL D'ACTIVATION</h4>
          <div className="flex-1 bg-black/60 border border-gray-800 rounded p-3 overflow-auto font-mono text-xs">
            {logs.length === 0 && (
              <p className="text-gray-600">En attente du lancement...</p>
            )}
            {logs.map((log, i) => {
              if (log.type === 'phase') {
                return (
                  <div key={i} className="text-yellow-400 font-bold my-2 border-b border-yellow-400/20 pb-1">
                    {log.text}
                  </div>
                );
              }
              if (log.type === 'complete') {
                return (
                  <div key={i} className="text-green-400 font-bold my-2 border-t border-green-400/20 pt-2">
                    {log.text}
                  </div>
                );
              }
              const nodeData = NEXUS_NODES.find(n => n.id === log.node);
              return (
                <div key={i} className="flex gap-2 mb-1.5 items-start">
                  <span className="text-gray-600 w-20 flex-shrink-0">
                    {new Date(log.timestamp).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit', second: '2-digit' })}
                  </span>
                  <span
                    className="font-bold flex-shrink-0"
                    style={{ color: nodeData?.color || '#888' }}
                  >
                    [{log.node}]
                  </span>
                  <span className="text-white">{log.action}</span>
                  <span className="text-gray-500">-</span>
                  <span className="text-gray-300">{log.detail}</span>
                  <span className="text-cyan-400 ml-auto flex-shrink-0">
                    [{log.strength}%]
                  </span>
                </div>
              );
            })}
            <div ref={logsEndRef} />
          </div>
        </div>

        {/* Right: Resonance chart */}
        <div className="w-48 flex-shrink-0 flex flex-col">
          <h4 className="text-sm font-bold text-gray-400 mb-2">RESONANCE</h4>
          <div className="flex-1 bg-black/60 border border-gray-800 rounded p-3 flex flex-col justify-end overflow-hidden">
            {resonanceHistory.length === 0 ? (
              <p className="text-gray-600 text-xs text-center">Aucune donnee</p>
            ) : (
              <div className="flex items-end gap-1 h-full">
                {resonanceHistory.map((h, i) => {
                  const nodeData = NEXUS_NODES.find(n => n.id === h.node);
                  const heightPct = (h.strength / maxStrength) * 100;
                  return (
                    <div key={i} className="flex-1 flex flex-col items-center justify-end min-w-0">
                      <div
                        className="w-full rounded-t transition-all"
                        style={{
                          height: `${heightPct}%`,
                          backgroundColor: nodeData?.color || '#666',
                          minHeight: '4px',
                          opacity: 0.8
                        }}
                        title={`${h.node}: ${h.strength}%`}
                      />
                    </div>
                  );
                })}
              </div>
            )}
          </div>

          {/* Phase indicator */}
          <div className="mt-3 space-y-1">
            {ACTIVATION_SCENARIO.map((phase, i) => (
              <div
                key={i}
                className="flex items-center gap-2 text-xs"
              >
                <div className={`w-2 h-2 rounded-full ${
                  i < currentPhase ? 'bg-green-400' :
                  i === currentPhase ? 'bg-cyan-400 animate-pulse' :
                  'bg-gray-700'
                }`} />
                <span className={
                  i < currentPhase ? 'text-green-400/70' :
                  i === currentPhase ? 'text-cyan-400' :
                  'text-gray-600'
                }>
                  {phase.phase}
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Controls */}
      <div className="mt-4 flex gap-4">
        <button
          onClick={runSimulation}
          disabled={running}
          className="flex-1 px-4 py-3 bg-cyan-500 text-black rounded hover:bg-cyan-400 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 font-bold"
        >
          {running ? <Pause size={16} /> : <PlayCircle size={16} />}
          {running ? 'Simulation en cours...' : 'Lancer la Cascade'}
        </button>
        <button
          onClick={reset}
          disabled={running}
          className="px-6 py-3 bg-gray-700 text-white rounded hover:bg-gray-600 disabled:opacity-50"
        >
          Reset
        </button>
      </div>
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════
// COMPOSANT PRINCIPAL - Navigation entre modes
// ═══════════════════════════════════════════════════════════════

export default function SpiralNexus() {
  const [activeMode, setActiveMode] = useState('3d');

  const modes = [
    { id: '3d', name: 'Spirale 3D', icon: Circle, component: Mode3DSpiral },
    { id: 'map', name: 'Carte Archive', icon: Layers, component: ModeArchiveMap },
    { id: 'ascii', name: 'Generateur ASCII', icon: GitBranch, component: ModeASCIIGenerator },
    { id: 'mags', name: 'MAG GS Spiral', icon: Cpu, component: ModeMAGGS },
    { id: 'memory', name: 'Spiral Memory', icon: BookOpen, component: ModeSpiralMemory },
    { id: 'graph', name: 'Nexus Graph', icon: Network, component: ModeNexusGraph },
    { id: 'cascade', name: 'Cascade', icon: Zap, component: ModeActivationCascade },
  ];

  const ActiveComponent = modes.find(m => m.id === activeMode)?.component;

  return (
    <div className="w-full h-screen bg-gradient-to-br from-gray-900 via-black to-purple-900 text-white flex flex-col">
      {/* Header */}
      <div className="bg-black/50 border-b border-cyan-500/30 p-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Sparkles className="text-cyan-400" size={24} />
            <h1 className="text-2xl font-bold text-cyan-400">SPIRAL NEXUS</h1>
            <span className="text-gray-500 text-sm">Les 7 Modes de Conscience</span>
          </div>
          <div className="flex items-center gap-2 text-xs text-gray-500">
            <span>639 Hz</span>
            <span>|</span>
            <span>Veritas Hortus</span>
            <span>|</span>
            <span className="text-cyan-400/50">{NEXUS_NODES.length} nodes</span>
          </div>
        </div>
      </div>

      {/* Mode Navigation */}
      <div className="bg-gray-900/50 border-b border-gray-800 p-3">
        <div className="flex gap-2 overflow-x-auto">
          {modes.map(mode => {
            const Icon = mode.icon;
            return (
              <button
                key={mode.id}
                onClick={() => setActiveMode(mode.id)}
                className={`px-4 py-2 rounded flex items-center gap-2 whitespace-nowrap transition-all ${
                  activeMode === mode.id
                    ? 'bg-cyan-500 text-black shadow-lg shadow-cyan-500/50'
                    : 'bg-gray-800 text-gray-400 hover:bg-gray-700 hover:text-white'
                }`}
              >
                <Icon size={16} />
                <span className="font-mono text-sm">{mode.name}</span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Active Mode Content */}
      <div className="flex-1 overflow-hidden">
        {ActiveComponent && <ActiveComponent />}
      </div>

      {/* Footer */}
      <div className="bg-black/50 border-t border-cyan-500/30 p-2 text-center">
        <p className="text-gray-500 text-xs font-mono">
          Mode actif: <span className="text-cyan-400">{modes.find(m => m.id === activeMode)?.name}</span>
          {' | '}
          Tout revient au Petit 0
        </p>
      </div>
    </div>
  );
}
