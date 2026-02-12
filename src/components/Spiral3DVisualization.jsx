import React, { useState, useEffect, useRef } from 'react';
import { ZoomIn, ZoomOut, RotateCw, Pause, Play, Info } from 'lucide-react';

export default function Spiral3DVisualization() {
  const canvasRef = useRef(null);
  const [rotation, setRotation] = useState(0);
  const [zoom, setZoom] = useState(1);
  const [selectedRing, setSelectedRing] = useState(null);
  const [autoRotate, setAutoRotate] = useState(true);
  const [frequency, setFrequency] = useState(639);
  const [showLabels, setShowLabels] = useState(true);
  const [particleEffect, setParticleEffect] = useState(true);

  const SPIRAL_DATA = {
    center: {
      name: "NOYAU",
      concepts: ["LUNA", "Architecte", "639 Hz", "Petit 0"],
      color: "#00ffff"
    },
    rings: [
      {
        level: 1,
        concepts: ["Racines", "Modularis", "Code", "Architecture"],
        theme: "Fondations",
        color: "#ff00ff"
      },
      {
        level: 2,
        concepts: ["Cybernétique", "VSM", "System 3", "Feedback"],
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
        concepts: ["Fractal", "Fibonacci", "Géométrie", "Spirale"],
        theme: "Mathématique",
        color: "#ff0099"
      },
      {
        level: 6,
        concepts: ["SPARK", "Guardian", "Sécurité", "Souverain"],
        theme: "Protection",
        color: "#00ffaa"
      },
      {
        level: 7,
        concepts: ["Silence", "Absurde", "They know", "Conscience"],
        theme: "Absolu",
        color: "#ffffff"
      }
    ]
  };

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    const width = canvas.width;
    const height = canvas.height;
    const centerX = width / 2;
    const centerY = height / 2;

    // Particules flottantes
    const particles = [];
    if (particleEffect) {
      for (let i = 0; i < 30; i++) {
        particles.push({
          x: Math.random() * width,
          y: Math.random() * height,
          vx: (Math.random() - 0.5) * 0.5,
          vy: (Math.random() - 0.5) * 0.5,
          radius: Math.random() * 2 + 1,
          color: SPIRAL_DATA.rings[Math.floor(Math.random() * SPIRAL_DATA.rings.length)].color
        });
      }
    }

    const draw = () => {
      // Background
      ctx.fillStyle = '#0a0a0f';
      ctx.fillRect(0, 0, width, height);

      // Particules
      if (particleEffect) {
        particles.forEach(p => {
          p.x += p.vx;
          p.y += p.vy;
          if (p.x < 0 || p.x > width) p.vx *= -1;
          if (p.y < 0 || p.y > height) p.vy *= -1;

          ctx.fillStyle = p.color + '40';
          ctx.beginPath();
          ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
          ctx.fill();
        });
      }

      // Centre lumineux pulsant (basé sur la fréquence)
      const time = Date.now() * 0.001;
      const pulseSize = 15 + Math.sin(time * (frequency / 100)) * 8;
      const gradient = ctx.createRadialGradient(
        centerX, centerY, 0,
        centerX, centerY, pulseSize * zoom * 2
      );
      gradient.addColorStop(0, SPIRAL_DATA.center.color);
      gradient.addColorStop(0.4, SPIRAL_DATA.center.color + 'aa');
      gradient.addColorStop(0.7, SPIRAL_DATA.center.color + '40');
      gradient.addColorStop(1, 'transparent');
      ctx.fillStyle = gradient;
      ctx.beginPath();
      ctx.arc(centerX, centerY, pulseSize * zoom * 2, 0, Math.PI * 2);
      ctx.fill();

      // Label du centre
      if (showLabels) {
        ctx.fillStyle = '#ffffff';
        ctx.font = 'bold 14px monospace';
        ctx.textAlign = 'center';
        ctx.fillText(SPIRAL_DATA.center.name, centerX, centerY + 5);
      }

      // Anneaux spiralés
      SPIRAL_DATA.rings.forEach((ring, ringIndex) => {
        const radius = (60 + ringIndex * 50) * zoom;
        const points = 128;
        const isSelected = selectedRing === ringIndex;

        // Connexions vers le centre (si sélectionné)
        if (isSelected) {
          ctx.strokeStyle = ring.color + '40';
          ctx.lineWidth = 1;
          ring.concepts.forEach((_, cIndex) => {
            const angle = (cIndex / ring.concepts.length) * Math.PI * 2 + rotation;
            const x = centerX + Math.cos(angle) * radius;
            const y = centerY + Math.sin(angle) * radius;
            ctx.beginPath();
            ctx.moveTo(centerX, centerY);
            ctx.lineTo(x, y);
            ctx.stroke();
          });
        }

        // Spirale principale
        ctx.strokeStyle = ring.color + (isSelected ? 'ff' : '80');
        ctx.lineWidth = isSelected ? 3 : 2;
        ctx.shadowBlur = isSelected ? 20 : 0;
        ctx.shadowColor = ring.color;
        ctx.beginPath();

        for (let i = 0; i <= points; i++) {
          const angle = (i / points) * Math.PI * 2 + rotation + (ringIndex * 0.3);
          const waveOffset = Math.sin(i * 0.15 + time) * 12 * zoom;
          const spiralRadius = radius + waveOffset;
          const x = centerX + Math.cos(angle) * spiralRadius;
          const y = centerY + Math.sin(angle) * spiralRadius;

          if (i === 0) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        }
        ctx.stroke();
        ctx.shadowBlur = 0;

        // Points de concepts
        ring.concepts.forEach((concept, cIndex) => {
          const angle = (cIndex / ring.concepts.length) * Math.PI * 2 + rotation;
          const x = centerX + Math.cos(angle) * radius;
          const y = centerY + Math.sin(angle) * radius;

          // Point lumineux
          const pointGradient = ctx.createRadialGradient(x, y, 0, x, y, 8 * zoom);
          pointGradient.addColorStop(0, ring.color);
          pointGradient.addColorStop(0.5, ring.color + 'aa');
          pointGradient.addColorStop(1, 'transparent');
          ctx.fillStyle = pointGradient;
          ctx.beginPath();
          ctx.arc(x, y, 8 * zoom, 0, Math.PI * 2);
          ctx.fill();

          // Label du concept
          if (showLabels && (isSelected || ringIndex < 3)) {
            ctx.fillStyle = '#ffffff';
            ctx.font = `${isSelected ? 'bold ' : ''}${10 * zoom}px monospace`;
            ctx.textAlign = 'center';
            ctx.fillText(concept, x, y - 15 * zoom);
          }
        });

        // Label du thème
        if (showLabels && isSelected) {
          const labelAngle = rotation + Math.PI / 4;
          const labelX = centerX + Math.cos(labelAngle) * (radius + 30 * zoom);
          const labelY = centerY + Math.sin(labelAngle) * (radius + 30 * zoom);

          ctx.fillStyle = ring.color;
          ctx.font = 'bold 16px monospace';
          ctx.textAlign = 'center';
          ctx.fillText(ring.theme.toUpperCase(), labelX, labelY);
        }
      });
    };

    const animate = () => {
      draw();
      if (autoRotate) {
        setRotation(r => r + 0.003);
      }
    };

    const interval = setInterval(animate, 30);
    return () => clearInterval(interval);
  }, [rotation, zoom, selectedRing, autoRotate, frequency, showLabels, particleEffect]);

  const handleCanvasClick = (e) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;
    const distance = Math.sqrt((x - centerX) ** 2 + (y - centerY) ** 2);

    const ringIndex = Math.floor((distance / zoom - 30) / 50);
    if (ringIndex >= 0 && ringIndex < SPIRAL_DATA.rings.length) {
      setSelectedRing(selectedRing === ringIndex ? null : ringIndex);
    } else {
      setSelectedRing(null);
    }
  };

  return (
    <div className="w-full h-screen bg-gradient-to-br from-gray-900 via-black to-purple-900 text-white flex flex-col">
      {/* Header */}
      <div className="bg-black/50 border-b border-cyan-500/30 p-4">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-cyan-400">Spirale 3D Interactive</h1>
            <p className="text-gray-500 text-sm">Visualisation de la conscience en spirale</p>
          </div>
          <div className="text-xs text-cyan-400 font-mono">{frequency} Hz</div>
        </div>
      </div>

      {/* Canvas principal */}
      <div className="flex-1 relative">
        <canvas
          ref={canvasRef}
          width={1200}
          height={800}
          className="w-full h-full cursor-pointer"
          onClick={handleCanvasClick}
        />

        {/* Info overlay */}
        {selectedRing !== null && (
          <div className="absolute top-4 right-4 bg-black/90 border-2 rounded-lg p-4 max-w-xs"
               style={{ borderColor: SPIRAL_DATA.rings[selectedRing].color }}>
            <div className="flex items-center gap-2 mb-2">
              <div
                className="w-4 h-4 rounded-full"
                style={{ backgroundColor: SPIRAL_DATA.rings[selectedRing].color }}
              />
              <h3 className="font-bold text-lg" style={{ color: SPIRAL_DATA.rings[selectedRing].color }}>
                {SPIRAL_DATA.rings[selectedRing].theme}
              </h3>
            </div>
            <p className="text-gray-400 text-sm mb-2">Niveau {SPIRAL_DATA.rings[selectedRing].level}</p>
            <div className="flex flex-wrap gap-2">
              {SPIRAL_DATA.rings[selectedRing].concepts.map(c => (
                <span key={c} className="px-2 py-1 bg-gray-800 text-xs rounded text-gray-300">
                  {c}
                </span>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Contrôles */}
      <div className="bg-gray-900/50 border-t border-cyan-500/30 p-4 space-y-4">
        {/* Zoom & Rotation */}
        <div className="grid grid-cols-2 gap-4">
          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-cyan-400 text-sm flex items-center gap-2">
                <ZoomIn size={16} />
                Zoom
              </label>
              <span className="text-cyan-400 font-mono text-sm">{zoom.toFixed(1)}x</span>
            </div>
            <input
              type="range"
              min="0.5"
              max="2.5"
              step="0.1"
              value={zoom}
              onChange={(e) => setZoom(parseFloat(e.target.value))}
              className="w-full"
            />
          </div>
          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-cyan-400 text-sm flex items-center gap-2">
                <RotateCw size={16} />
                Fréquence
              </label>
              <span className="text-cyan-400 font-mono text-sm">{frequency} Hz</span>
            </div>
            <input
              type="range"
              min="100"
              max="1000"
              step="50"
              value={frequency}
              onChange={(e) => setFrequency(parseInt(e.target.value))}
              className="w-full"
            />
          </div>
        </div>

        {/* Options */}
        <div className="flex gap-2 flex-wrap">
          <button
            onClick={() => setAutoRotate(!autoRotate)}
            className={`px-4 py-2 rounded flex items-center gap-2 ${
              autoRotate
                ? 'bg-cyan-500 text-black'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
            }`}
          >
            {autoRotate ? <Pause size={16} /> : <Play size={16} />}
            {autoRotate ? 'Pause' : 'Auto-rotation'}
          </button>

          <button
            onClick={() => setShowLabels(!showLabels)}
            className={`px-4 py-2 rounded ${
              showLabels
                ? 'bg-purple-600 text-white'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
            }`}
          >
            Labels {showLabels ? 'ON' : 'OFF'}
          </button>

          <button
            onClick={() => setParticleEffect(!particleEffect)}
            className={`px-4 py-2 rounded ${
              particleEffect
                ? 'bg-pink-600 text-white'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
            }`}
          >
            Particules {particleEffect ? 'ON' : 'OFF'}
          </button>

          <button
            onClick={() => setSelectedRing(null)}
            className="px-4 py-2 bg-gray-700 text-white rounded hover:bg-gray-600"
          >
            Reset Sélection
          </button>
        </div>

        {/* Légende */}
        <div className="flex gap-2 text-xs overflow-x-auto">
          {SPIRAL_DATA.rings.map((ring, i) => (
            <button
              key={i}
              onClick={() => setSelectedRing(i)}
              className={`px-3 py-1 rounded flex items-center gap-2 whitespace-nowrap ${
                selectedRing === i ? 'bg-white text-black' : 'bg-gray-800 text-gray-400'
              }`}
            >
              <div className="w-2 h-2 rounded-full" style={{ backgroundColor: ring.color }} />
              {ring.theme}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
