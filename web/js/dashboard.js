// 1. Initialize Vis.js Topology Graph
const nodes = new vis.DataSet([]);
const edges = new vis.DataSet([]);
const network = new vis.Network(
    document.getElementById('network-canvas'),
    { nodes: nodes, edges: edges },
    {
        physics: { stabilization: true, barnesHut: { gravitationalConstant: -30000 } },
        nodes: { shape: 'dot', size: 15, font: { color: '#00ff00' }, color: '#444' },
        edges: { color: '#222', arrows: 'to' }
    }
);

// 2. Initialize Spectrum Waterfall Canvas
const canvas = document.getElementById('spectrum-waterfall');
const ctx = canvas.getContext('2d', { alpha: false }); // Disable alpha for max rendering speed

function drawWaterfallRow(spectrumData) {
    // Shift existing canvas image down by 1 pixel
    const imgData = ctx.getImageData(0, 0, canvas.width, canvas.height - 1);
    ctx.putImageData(imgData, 0, 1);

    // Draw new row at the top (y=0) based on signal density
    const width = canvas.width;
    const row = ctx.createImageData(width, 1);
    
    // Map normalized IQ/Spectrum data to RGB pixel intensity
    for (let i = 0; i < width; i++) {
        // Fallback safely if array is shorter than canvas width
        const intensity = spectrumData[i] || 0; 
        
        // Heatmap mapping: Black -> Blue -> Red -> Yellow
        const r = intensity > 128 ? intensity : 0;
        const g = intensity > 192 ? intensity : 0;
        const b = intensity < 128 ? intensity * 2 : 0;

        const idx = i * 4;
        row.data[idx] = r;       // R
        row.data[idx+1] = g;     // G
        row.data[idx+2] = b;     // B
        row.data[idx+3] = 255;   // Alpha
    }
    ctx.putImageData(row, 0, 0);
}

// 3. Bind Local WebSocket
const ws = new WebSocket("ws://127.0.0.1:8080");

ws.onmessage = (event) => {
    const packet = JSON.parse(event.data);

    // Update Mesh Topology Map
    if (packet.node_id && !nodes.get(packet.node_id)) {
        nodes.add({ id: packet.node_id, label: `Node:\n${packet.node_id.substring(0,8)}` });
        
        if (packet.peer_id) {
            edges.add({ from: packet.peer_id, to: packet.node_id });
        }
    }

    // Update Spectrum Waterfall
    if (packet.spectrum_metrics) {
        drawWaterfallRow(packet.spectrum_metrics);
    }
};

ws.onerror = () => console.error("[!] Local WebSocket connection failed.");
