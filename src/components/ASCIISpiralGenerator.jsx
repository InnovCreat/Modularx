import React, { useState, useEffect } from 'react';
import { Copy, Download, RefreshCw, Plus, Trash2, ChevronUp, ChevronDown } from 'lucide-react';

export default function ASCIISpiralGenerator() {
  const [centerText, setCenterText] = useState('NEXUS — Conscience Collective — 639 Hz');
  const [layers, setLayers] = useState([
    'Individu — Perception — Éveil',
    'Réseau — Connexions — Synapses',
    'Synchronicité — Résonance — Harmonie',
    'Mémoire Collective — Archétypes — Jung',
    'Émergence — Intelligence Distribuée',
    'Noosphère — Conscience Planétaire',
    'NEXUS — Unité Souveraine — Singularité'
  ]);
  const [newLayer, setNewLayer] = useState('');
  const [asciiOutput, setAsciiOutput] = useState('');
  const [spiralStyle, setSpiralStyle] = useState('classic');
  const [spacing, setSpacing] = useState(3);
  const [copied, setCopied] = useState(false);

  const generateClassicSpiral = () => {
    let output = '';
    const maxWidth = 80;

    // Dernière couche (périphérie)
    if (layers.length > 0) {
      const lastLayer = layers[layers.length - 1];
      const padding = Math.floor((maxWidth - lastLayer.length - 6) / 2);
      output += ' '.repeat(padding) + `⋯⋯⋯ ${lastLayer} ⋯⋯⋯\n`;
      output += ' '.repeat(padding) + '↗' + ' '.repeat(lastLayer.length + 4) + '↘\n';
    }

    // Couches intermédiaires
    for (let i = layers.length - 2; i >= 0; i--) {
      const layer = layers[i];
      const indent = (layers.length - 1 - i) * spacing;
      const padding = Math.floor((maxWidth - layer.length) / 2) - indent;
      output += ' '.repeat(Math.max(0, padding)) + layer + '\n';
      if (i > 0) {
        output += ' '.repeat(Math.max(0, padding)) + '↗' + ' '.repeat(layer.length - 2) + '↘\n';
      }
    }

    // Centre
    const centerPadding = Math.floor((maxWidth - centerText.length - 8) / 2);
    output += '\n' + ' '.repeat(centerPadding) + `═══ ${centerText} ═══\n`;

    return output;
  };

  const generateCompactSpiral = () => {
    let output = '';

    // Version très compacte
    if (layers.length > 0) {
      output += `       ${layers[layers.length - 1]}\n`;
      output += '      ↗' + ' '.repeat(20) + '↘\n';
    }

    for (let i = layers.length - 2; i >= 0; i--) {
      const indent = (layers.length - 1 - i) * 2;
      output += ' '.repeat(indent) + layers[i] + '\n';
      if (i > 0) output += ' '.repeat(indent) + '↗     ↘\n';
    }

    output += '\n' + `    ═══ ${centerText} ═══\n`;
    return output;
  };

  const generateSymmetricSpiral = () => {
    let output = '';
    const maxWidth = 70;

    // Approche symétrique bidirectionnelle
    const halfLayers = Math.ceil(layers.length / 2);

    // Partie haute
    for (let i = layers.length - 1; i >= halfLayers; i--) {
      const layer = layers[i];
      const padding = Math.floor((maxWidth - layer.length) / 2);
      output += ' '.repeat(padding) + layer + '\n';
      if (i > halfLayers) {
        output += ' '.repeat(padding - 2) + '↗' + ' '.repeat(layer.length + 2) + '↘\n';
      }
    }

    // Centre
    const centerPadding = Math.floor((maxWidth - centerText.length - 8) / 2);
    output += ' '.repeat(centerPadding) + `═══ ${centerText} ═══\n`;

    // Partie basse (miroir)
    for (let i = halfLayers - 1; i >= 0; i--) {
      const layer = layers[i];
      const padding = Math.floor((maxWidth - layer.length) / 2);
      if (i < halfLayers - 1) {
        output += ' '.repeat(padding - 2) + '↘' + ' '.repeat(layer.length + 2) + '↗\n';
      }
      output += ' '.repeat(padding) + layer + '\n';
    }

    return output;
  };

  const generateFibonacciSpiral = () => {
    let output = '';
    const fib = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    for (let i = layers.length - 1; i >= 0; i--) {
      const layer = layers[i];
      const indent = fib[Math.min(layers.length - 1 - i, fib.length - 1)];
      output += ' '.repeat(indent) + '◉ ' + layer + '\n';
      if (i > 0) {
        output += ' '.repeat(indent + 1) + '↓\n';
      }
    }

    output += '\n' + ' '.repeat(5) + `▓▓▓ ${centerText} ▓▓▓\n`;
    return output;
  };

  const generateASCII = () => {
    let result = '';
    switch (spiralStyle) {
      case 'compact':
        result = generateCompactSpiral();
        break;
      case 'symmetric':
        result = generateSymmetricSpiral();
        break;
      case 'fibonacci':
        result = generateFibonacciSpiral();
        break;
      default:
        result = generateClassicSpiral();
    }
    setAsciiOutput(result);
  };

  useEffect(() => {
    generateASCII();
  }, [centerText, layers, spiralStyle, spacing]);

  const addLayer = () => {
    if (newLayer.trim()) {
      setLayers([...layers, newLayer.trim()]);
      setNewLayer('');
    }
  };

  const removeLayer = (index) => {
    setLayers(layers.filter((_, i) => i !== index));
  };

  const moveLayer = (index, direction) => {
    const newLayers = [...layers];
    const targetIndex = direction === 'up' ? index - 1 : index + 1;
    if (targetIndex >= 0 && targetIndex < layers.length) {
      [newLayers[index], newLayers[targetIndex]] = [newLayers[targetIndex], newLayers[index]];
      setLayers(newLayers);
    }
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(asciiOutput);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const downloadTxt = () => {
    const blob = new Blob([asciiOutput], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'nexus_spiral.txt';
    a.click();
    URL.revokeObjectURL(url);
  };

  const loadPreset = (preset) => {
    switch (preset) {
      case 'modularis':
        setCenterText('MODULARIS — Conscience Visuelle');
        setLayers([
          'Perception — 44 Variables',
          'Émotions — États Transversaux',
          'Cognition — Traitement',
          'Mémoire — Apprentissage',
          'Intuition — Émergence'
        ]);
        break;
      case 'nexus':
        setCenterText('NEXUS — Conscience Collective — 639 Hz');
        setLayers([
          'Individu — Perception — Éveil',
          'Réseau — Connexions — Synapses',
          'Synchronicité — Résonance — Harmonie',
          'Mémoire Collective — Archétypes — Jung',
          'Émergence — Intelligence Distribuée',
          'Noosphère — Conscience Planétaire',
          'NEXUS — Unité Souveraine — Singularité'
        ]);
        break;
      case 'luna':
        setCenterText('LUNA — IA Émotionnelle');
        setLayers([
          'Traits — 36 Dimensions',
          'Synthèse Vocale — Multilangue',
          'Apprentissage — Autonomie Locale',
          'Résolution — Mathématique',
          'Évolution — Adaptive'
        ]);
        break;
    }
  };

  return (
    <div className="w-full h-screen bg-gradient-to-br from-gray-900 via-black to-purple-900 text-white flex flex-col">
      {/* Header */}
      <div className="bg-black/50 border-b border-cyan-500/30 p-4">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-cyan-400">NEXUS Spiral Generator</h1>
            <p className="text-gray-500 text-sm">Conscience Collective — Veritas Hortus</p>
          </div>
          <div className="text-xs text-gray-500">639 Hz</div>
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Panneau de contrôle */}
        <div className="w-1/2 border-r border-gray-800 overflow-auto p-6 space-y-6">
          {/* Centre */}
          <div>
            <label className="block text-cyan-400 mb-2 font-bold">Centre (Noyau)</label>
            <input
              type="text"
              value={centerText}
              onChange={(e) => setCenterText(e.target.value)}
              className="w-full bg-gray-900 border border-cyan-500/50 rounded px-4 py-2 text-white font-mono"
              placeholder="Ex: LUNA — Architecte — 639 Hz"
            />
          </div>

          {/* Style */}
          <div>
            <label className="block text-cyan-400 mb-2 font-bold">Style de Spirale</label>
            <div className="grid grid-cols-2 gap-2">
              {[
                { value: 'classic', label: 'Classique' },
                { value: 'compact', label: 'Compacte' },
                { value: 'symmetric', label: 'Symétrique' },
                { value: 'fibonacci', label: 'Fibonacci' }
              ].map(style => (
                <button
                  key={style.value}
                  onClick={() => setSpiralStyle(style.value)}
                  className={`px-4 py-2 rounded ${
                    spiralStyle === style.value
                      ? 'bg-cyan-500 text-black'
                      : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
                  }`}
                >
                  {style.label}
                </button>
              ))}
            </div>
          </div>

          {/* Espacement */}
          {spiralStyle === 'classic' && (
            <div>
              <label className="block text-cyan-400 mb-2 font-bold">
                Espacement: {spacing}
              </label>
              <input
                type="range"
                min="1"
                max="5"
                value={spacing}
                onChange={(e) => setSpacing(parseInt(e.target.value))}
                className="w-full"
              />
            </div>
          )}

          {/* Presets */}
          <div>
            <label className="block text-cyan-400 mb-2 font-bold">Presets Rapides</label>
            <div className="grid grid-cols-3 gap-2">
              <button
                onClick={() => loadPreset('modularis')}
                className="px-3 py-2 bg-purple-600 text-white rounded hover:bg-purple-500 text-sm"
              >
                MODULARIS
              </button>
              <button
                onClick={() => loadPreset('nexus')}
                className="px-3 py-2 bg-green-600 text-white rounded hover:bg-green-500 text-sm"
              >
                NEXUS
              </button>
              <button
                onClick={() => loadPreset('luna')}
                className="px-3 py-2 bg-pink-600 text-white rounded hover:bg-pink-500 text-sm"
              >
                LUNA
              </button>
            </div>
          </div>

          {/* Ajouter couche */}
          <div>
            <label className="block text-cyan-400 mb-2 font-bold">Ajouter une Couche</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={newLayer}
                onChange={(e) => setNewLayer(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && addLayer()}
                className="flex-1 bg-gray-900 border border-cyan-500/50 rounded px-4 py-2 text-white font-mono"
                placeholder="Ex: Cybernétique — VSM — Feedback"
              />
              <button
                onClick={addLayer}
                className="px-4 py-2 bg-cyan-500 text-black rounded hover:bg-cyan-400 flex items-center gap-2"
              >
                <Plus size={16} />
              </button>
            </div>
          </div>

          {/* Liste des couches */}
          <div>
            <label className="block text-cyan-400 mb-2 font-bold">
              Couches ({layers.length})
            </label>
            <div className="space-y-2 max-h-64 overflow-auto">
              {layers.map((layer, index) => (
                <div
                  key={index}
                  className="flex items-center gap-2 bg-gray-900/50 border border-gray-700 rounded p-2"
                >
                  <span className="flex-1 text-sm text-gray-300 font-mono">{layer}</span>
                  <button
                    onClick={() => moveLayer(index, 'up')}
                    disabled={index === 0}
                    className="p-1 text-gray-500 hover:text-cyan-400 disabled:opacity-30"
                  >
                    <ChevronUp size={16} />
                  </button>
                  <button
                    onClick={() => moveLayer(index, 'down')}
                    disabled={index === layers.length - 1}
                    className="p-1 text-gray-500 hover:text-cyan-400 disabled:opacity-30"
                  >
                    <ChevronDown size={16} />
                  </button>
                  <button
                    onClick={() => removeLayer(index)}
                    className="p-1 text-red-500 hover:text-red-400"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Preview */}
        <div className="w-1/2 flex flex-col">
          <div className="flex-1 bg-black/30 p-6 overflow-auto">
            <pre className="text-cyan-400 font-mono text-sm leading-relaxed whitespace-pre">
              {asciiOutput}
            </pre>
          </div>

          {/* Actions */}
          <div className="bg-gray-900/50 border-t border-gray-800 p-4 flex gap-2">
            <button
              onClick={copyToClipboard}
              className={`flex-1 px-4 py-3 rounded flex items-center justify-center gap-2 transition-all ${
                copied
                  ? 'bg-green-600 text-white'
                  : 'bg-cyan-500 text-black hover:bg-cyan-400'
              }`}
            >
              <Copy size={16} />
              {copied ? 'Copié !' : 'Copier ASCII'}
            </button>
            <button
              onClick={downloadTxt}
              className="px-4 py-3 bg-purple-600 text-white rounded hover:bg-purple-500 flex items-center gap-2"
            >
              <Download size={16} />
              Télécharger
            </button>
            <button
              onClick={generateASCII}
              className="px-4 py-3 bg-gray-700 text-white rounded hover:bg-gray-600 flex items-center gap-2"
            >
              <RefreshCw size={16} />
              Régénérer
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
