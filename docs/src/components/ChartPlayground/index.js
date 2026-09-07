import React, { useState, useMemo } from 'react';
import styles from './styles.module.css';
import CodeBlock from '@theme/CodeBlock';

export default function ChartPlayground() {
  const [dataPointsCount, setDataPointsCount] = useState(5);
  const [colorStart, setColorStart] = useState('#8a2be2'); // hsl(250, 100%, 70%) approx
  const [colorEnd, setColorEnd] = useState('#00bfff');   // hsl(190, 100%, 50%) approx
  const [lineWidth, setLineWidth] = useState(4);
  const [tension, setTension] = useState(0); // 0 = straight lines, >0 = curved (optional feature)

  // Generate mock data based on count
  const data = useMemo(() => {
    const pts = [];
    const step = 600 / Math.max(1, dataPointsCount - 1);
    for (let i = 0; i < dataPointsCount; i++) {
      pts.push({
        x: i * step,
        y: 200 - (Math.random() * 150 + 20) // Random Y between 20 and 170
      });
    }
    return pts;
  }, [dataPointsCount]);

  // Generate SVG Path
  const pathD = useMemo(() => {
    if (data.length === 0) return '';
    let d = `M ${data[0].x} ${data[0].y}`;
    
    if (tension > 0) {
      // Basic smooth curve for demonstration
      for (let i = 0; i < data.length - 1; i++) {
        const curr = data[i];
        const next = data[i + 1];
        const cpX = (curr.x + next.x) / 2;
        d += ` C ${cpX} ${curr.y}, ${cpX} ${next.y}, ${next.x} ${next.y}`;
      }
    } else {
      for (let i = 1; i < data.length; i++) {
        d += ` L ${data[i].x} ${data[i].y}`;
      }
    }
    return d;
  }, [data, tension]);

  // Generated Rust Code snippet
  const rustCode = `use ferrox_front_charts::{line_chart, Point};

let data = vec![
${data.slice(0, 5).map(p => `    Point { x: ${p.x.toFixed(1)}, y: ${p.y.toFixed(1)} },`).join('\n')}${data.length > 5 ? '\n    // ... more points' : ''}
];

// In the real Wasm engine, styling is applied via Ferrox CSS definitions
let chart = line_chart(data, 600, 200);`;

  return (
    <div className={styles.playgroundContainer}>
      <div className={styles.playgroundMain}>
        
        {/* Chart Area */}
        <div className={styles.chartArea}>
          <div className={styles.chartContent}>
            <svg 
              className="ferrox-svg-chart" 
              width="100%" 
              height="100%" 
              viewBox="-20 -20 640 240"
              style={{ overflow: 'visible' }}
            >
              <defs>
                <linearGradient id="chartGradientLive" x1="0%" y1="0%" x2="100%" y2="0%">
                  <stop offset="0%" stopColor={colorStart} />
                  <stop offset="100%" stopColor={colorEnd} />
                </linearGradient>
              </defs>
              <path
                d={pathD}
                fill="none"
                stroke="url(#chartGradientLive)"
                strokeWidth={lineWidth}
                strokeLinecap="round"
                strokeLinejoin="round"
                style={{
                  filter: 'drop-shadow(0px 10px 10px rgba(0,0,0,0.5))',
                  transition: 'd 0.3s ease-out, stroke-width 0.3s ease'
                }}
              />
              {/* Optional: Draw points */}
              {data.map((p, i) => (
                <circle 
                  key={i} 
                  cx={p.x} 
                  cy={p.y} 
                  r={lineWidth * 1.5} 
                  fill="#0d1117" 
                  stroke="url(#chartGradientLive)" 
                  strokeWidth={lineWidth / 2} 
                  style={{ transition: 'all 0.3s ease-out' }}
                />
              ))}
            </svg>
          </div>
        </div>

        {/* Sidebar Controls */}
        <div className={styles.sidebar}>
          <h3>Interactive Controls</h3>
          
          <div className={styles.controlGroup}>
            <label>
              Data Points <span>{dataPointsCount}</span>
            </label>
            <input 
              type="range" 
              min="2" 
              max="20" 
              value={dataPointsCount} 
              onChange={(e) => setDataPointsCount(parseInt(e.target.value))}
            />
          </div>

          <div className={styles.controlGroup}>
            <label>
              Line Thickness <span>{lineWidth}px</span>
            </label>
            <input 
              type="range" 
              min="1" 
              max="15" 
              value={lineWidth} 
              onChange={(e) => setLineWidth(parseInt(e.target.value))}
            />
          </div>

          <div className={styles.controlGroup}>
            <label>
              Line Tension (Smoothness) <span>{tension}</span>
            </label>
            <input 
              type="range" 
              min="0" 
              max="1" 
              step="1"
              value={tension} 
              onChange={(e) => setTension(parseInt(e.target.value))}
            />
          </div>

          <div className={styles.controlGroup}>
            <label>Gradient Start Color</label>
            <input 
              type="color" 
              value={colorStart} 
              onChange={(e) => setColorStart(e.target.value)}
            />
          </div>

          <div className={styles.controlGroup}>
            <label>Gradient End Color</label>
            <input 
              type="color" 
              value={colorEnd} 
              onChange={(e) => setColorEnd(e.target.value)}
            />
          </div>

        </div>
      </div>

      {/* Code Display Area */}
      <div className={styles.codeArea}>
        <CodeBlock language="rust">
          {rustCode}
        </CodeBlock>
      </div>
    </div>
  );
}
