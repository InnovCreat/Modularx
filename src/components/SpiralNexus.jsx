import React, { useState, useEffect, useRef, useCallback } from 'react';
import { Circle, Layers, Cpu, BookOpen, Sparkles, GitBranch, PlayCircle, Pause } from 'lucide-react';

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

      // Centre lumineux pulsant
      const pulseSize = 10 + Math.sin(Date.now() * 0.003) * 5;
      const gradient = ctx.createRadialGradient(centerX, centerY, 0, centerX, centerY, pulseSize * zoom);
      gradient.addColorStop(0, SPIRAL_DATA.center.color);
      gradient.addColorStop(0.5, SPIRAL_DATA.center.color + '80');
      gradient.addColorStop(1, 'transparent');
      ctx.fillStyle = gradient;

      ctx.beginPath();
      ctx.arc(centerX, centerY, pulseSize * zoom, 0, Math.PI * 2);
      ctx.fill();

      // Anneaux spiralés
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

        // Points de concepts
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
        {/* Centre */}
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

        {/* Anneaux */}
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

    // Dernière couche (périphérie)
    if (layers.length > 0) {
      output += indent + `... ${layers[layers.length - 1]} ...\n`;
      output += indent + '/' + ' '.repeat(30) + '\\\n';
    }

    // Couches intermédiaires
    for (let i = layers.length - 2; i >= 0; i--) {
      const spacing = ' '.repeat(20 - i * 3);
      output += spacing + layers[i] + '\n';
      if (i > 0) output += spacing + '/' + ' '.repeat(15 + i * 5) + '\\\n';
    }

    // Centre
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
          {/* Centre - Petit 0 */}
          <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-10">
            <div className={`w-16 h-16 rounded-full flex items-center justify-center border-2 ${
              evaluating ? 'animate-pulse border-cyan-400 bg-cyan-500/20' : 'border-gray-600 bg-gray-800'
            }`}>
              <span className="text-xs text-cyan-400 font-mono">Petit 0</span>
            </div>
          </div>

          {/* Noeuds en spirale */}
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

          {/* Vague de propagation */}
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

      {/* Timeline circulaire */}
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

      {/* Details de la phase */}
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
// COMPOSANT PRINCIPAL - Navigation entre modes
// ═══════════════════════════════════════════════════════════════

export default function SpiralNexus() {
  const [activeMode, setActiveMode] = useState('3d');

  const modes = [
    { id: '3d', name: 'Spirale 3D', icon: Circle, component: Mode3DSpiral },
    { id: 'map', name: 'Carte Archive', icon: Layers, component: ModeArchiveMap },
    { id: 'ascii', name: 'Generateur ASCII', icon: GitBranch, component: ModeASCIIGenerator },
    { id: 'mags', name: 'MAG GS Spiral', icon: Cpu, component: ModeMAGGS },
    { id: 'memory', name: 'Spiral Memory', icon: BookOpen, component: ModeSpiralMemory }
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
            <span className="text-gray-500 text-sm">Les 5 Modes de Conscience</span>
          </div>
          <div className="flex items-center gap-2 text-xs text-gray-500">
            <span>639 Hz</span>
            <span>|</span>
            <span>Veritas Hortus</span>
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
