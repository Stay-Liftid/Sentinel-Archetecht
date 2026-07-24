// Zero-Trust Topology and Mesh Canvas Renderer
class MeshTopologyVisualizer {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.nodes = new Map();
        this.links = [];
        
        this.initWebSocket();
    }

    initWebSocket() {
        const ws = new WebSocket("ws://127.0.0.1:8080");
        ws.onmessage = (event) => {
            const packet = JSON.parse(event.data);
            this.updateNodeState(packet);
            this.render();
        };
    }

    updateNodeState(packet) {
        // Track nodes via their dynamic ZK nullifier epochs & HLC timestamps
        this.nodes.set(packet.nullifier, {
            id: packet.nullifier,
            lastSeen: Date.now(),
            hlc: packet.hlcTimestamp,
            rssi: packet.rssi || -65
        });
    }

    render() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        
        // Draw mesh nodes and connections on an offline local canvas grid
        let index = 0;
        const totalNodes = this.nodes.size;
        const centerX = this.canvas.width / 2;
        const centerY = this.canvas.height / 2;
        const radius = 120;

        this.nodes.forEach((node, id) => {
            const angle = (index / totalNodes) * 2 * Math.PI;
            const x = centerX + radius * Math.cos(angle);
            const y = centerY + radius * Math.sin(angle);

            // Render Node Circle
            this.ctx.beginPath();
            this.ctx.arc(x, y, 8, 0, 2 * Math.PI);
            this.ctx.fillStyle = '#00ff66';
            this.ctx.fill();
            this.ctx.strokeStyle = '#ffffff';
            this.ctx.stroke();

            // Render Node Label
            this.ctx.fillStyle = '#a0a0a0';
            this.ctx.font = '10px monospace';
            this.ctx.fillText(`Node: ${id.substring(0, 6)}...`, x - 20, y + 20);

            index++;
        });
    }
}

// Initialize on DOM load
window.addEventListener('DOMContentLoaded', () => {
    if (document.getElementById('meshCanvas')) {
        new MeshTopologyVisualizer('meshCanvas');
    }
});
