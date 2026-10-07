# 🖨️ MMLAB Scanner Bridge

> **Ultra-lightweight local scanner control center and native REST API daemon for modern web applications.**  
> Built with 🦀 **Rust**, **eSCL** (AirScan/Mopria), **WebKit / WebView2**, and **Axum**.

<p align="center">
  <img src="icon_128.png" alt="MMLAB Scanner Bridge Logo" width="108" height="108" />
</p>

<p align="center">
  <a href="https://github.com/e404r/MMLAB-Scanner-Bridge/releases"><img src="https://img.shields.io/badge/release-v1.0.0-crimson.svg?style=flat-square" alt="Release v1.0.0" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License MIT" /></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey.svg?style=flat-square" alt="Platform" />
  <img src="https://img.shields.io/badge/protocol-eSCL%20(AirScan)-success.svg?style=flat-square" alt="Protocol eSCL" />
</p>

<p align="center">
  🇬🇪 <strong>ქართული ვერსიისთვის იხილეთ: <a href="README.ka.md">README.ka.md</a></strong>
</p>

---

## 📖 Overview

Modern web browsers enforce strict security policies (**CORS** and **Private Network Access - PNA**), preventing web applications (ERP, CRM, and document management systems) from directly initiating TCP/HTTP requests to physical hardware devices on local subnets.

**MMLAB Scanner Bridge** completely bridges this gap:
- Operates a high-performance local daemon on `http://127.0.0.1:58260` with full CORS support (`Access-Control-Allow-Origin: *`).
- Communicates directly with network printers and scanners using the universal **eSCL protocol** (Apple AirScan / Mopria).
- Returns pristine, binary **PDF documents** (`application/pdf`) straight to your web application via simple HTTP calls.
- Runs without bulky Electron dependencies — the entire native binary is only a few megabytes.

---

## ✨ Features

- **Universal Driverless Scanning (eSCL):** Speaks the standard Apple AirScan / Mopria protocol over HTTP/XML. Compatible with network multifunction printers (MFP) from HP (LaserJet, OfficeJet, PageWide), Canon, Epson, Brother, and Xerox.
- **Native OS Desktop Window:** Powered by `tao` and `wry` utilizing the operating system's native web engine (WebKit on macOS, WebView2 on Windows).
- **System Tray & Menu Bar Daemon:** Closing the window (`X`) minimizes the app directly to the system tray/menu bar, allowing continuous background scanning for web applications without keeping a window open.
- **Smart System Locale Detection:** Automatically detects whether the OS language is Georgian or English, displaying localized UI, CLI logs, tray menus, and API error responses. Can be manually overridden via `--lang ka` or `--lang en`.
- **ZeroConf Network Discovery (mDNS):** Finds all eSCL-enabled scanners on the local subnet in seconds using `_uscan._tcp.local.`, eliminating manual IP entry.
- **Security Token Protection (`api_sec_print`):** Optional configurable API authorization token to protect against unauthorized scan triggers.
- **Port Conflict Management:** Verifies port `58260` availability before launch. If already running, it smoothly focuses or opens the existing instance without crash or duplication.
- **Diagnostics & Live Preview:** Hardware ping, ADF paper sensor verification, latency tracking, flatbed/feeder selection, resolution options (150, 300, 600 DPI), duplex mode, and embedded live PDF preview.

---

## 🔌 REST API Reference

The daemon listens on **`http://127.0.0.1:58260`**. All endpoints include permissive CORS headers (`Access-Control-Allow-Origin: *`).

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/health` | Daemon health check & version verification |
| `GET` | `/status` | Scanner online diagnostics, ADF paper sensor, and state |
| `GET` | `/discover` | **mDNS ZeroConf network scanner auto-discovery** |
| `POST` | `/scan` | **Trigger scan job and stream back binary PDF** |
| `GET` | `/config` | Read current default printer IP, port, and security token |
| `POST` | `/config` | Persist updated settings to `scanner_config.json` |
| `GET` | `/style.css` | Serve dashboard CSS stylesheet |
| `GET` | `/icon.png` | Serve official application icon |
| `GET` | `/` or `/dashboard` | Interactive embedded web control center |

---

### 1. `GET /health` — Service Health Check

Quickly verify whether the Scanner Bridge daemon is active on the user's computer.

**Response (200 OK):**
```json
{
  "status": "ok",
  "app": "hr-scanner-bridge",
  "version": "1.0.0",
  "port": 58260,
  "language": "en"
}
```

---

### 2. `GET /status` — Hardware Diagnostics

Checks connectivity with the target scanner via eSCL.

**Query Parameters (Optional):**
- `ip`: Target scanner IP (defaults to configured IP).
- `port`: Port (defaults to configured port or `80`).

**Example Request:**
```http
GET http://127.0.0.1:58260/status?ip=192.168.3.133&port=80
```

**Response (200 OK — Online):**
```json
{
  "online": true,
  "state": "Idle",
  "ip": "192.168.3.133",
  "port": 80,
  "error": null
}
```

---

### 3. `GET /discover` — mDNS Scanner Discovery

Browses the local area network (`_uscan._tcp.local`) for 2 seconds and returns all discovered eSCL devices.

```http
GET http://127.0.0.1:58260/discover?timeout=2
```

**Response (200 OK):**
```json
[
  {
    "name": "HP LaserJet Pro MFP 4103 (F3B1A2)",
    "ip": "192.168.3.133",
    "port": 80,
    "model": "HP LaserJet Pro MFP 4103",
    "protocol": "eSCL"
  }
]
```

---

### 4. `POST /scan` — Initiate Scan & Retrieve PDF

Executes a scan job and returns the scanned PDF binary document.

**Headers:**
```http
Content-Type: application/json
api_sec_print: sec_prn_your_secret_token (optional, only if configured)
```

**Request Body (JSON):**
```json
{
  "source": "Platen",
  "resolution": 300,
  "duplex": false,
  "ip": "192.168.3.133",
  "port": 80,
  "api_sec_print": "sec_prn_your_secret_token"
}
```

**Parameters:**
- `source` *(string, optional)*: `"Platen"` (flatbed glass) or `"Feeder"` (automatic document feeder / ADF). Default: `"Platen"`.
- `resolution` *(number, optional)*: `150`, `300`, or `600` DPI. Default: `300`.
- `duplex` *(boolean, optional)*: `true` or `false` (requires `source: "Feeder"` and printer duplex capability). Default: `false`.
- `ip` *(string, optional)*: Override destination scanner IP.
- `port` *(number, optional)*: Override destination scanner port.
- `api_sec_print` *(string, optional)*: Security token if enabled in settings.

**Response (200 OK):**
- **Content-Type:** `application/pdf`
- **Content-Disposition:** `inline; filename="scanned_document.pdf"`
- **Body:** Binary Stream (`application/pdf`)

---

## 💻 Web Integration Guides

### Option 1: Vanilla JavaScript / TypeScript (`fetch`)

```typescript
async function scanDocument(): Promise<Blob> {
  const response = await fetch('http://127.0.0.1:58260/scan', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      source: 'Feeder',     // 'Platen' or 'Feeder'
      resolution: 300,      // 150, 300, 600 DPI
      duplex: false
    })
  });

  if (!response.ok) {
    throw new Error(`Scan failed: ${await response.text()}`);
  }

  const pdfBlob = await response.blob();
  
  // Preview in new browser tab
  const fileUrl = URL.createObjectURL(pdfBlob);
  window.open(fileUrl, '_blank');
  return pdfBlob;
}
```

---

### Option 2: Angular (Standalone Service)

```typescript
import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

export interface ScanOptions {
  source?: 'Platen' | 'Feeder';
  resolution?: 150 | 300 | 600;
  duplex?: boolean;
  ip?: string;
  port?: number;
}

@Injectable({ providedIn: 'root' })
export class ScannerBridgeService {
  private readonly bridgeUrl = 'http://127.0.0.1:58260';
  private readonly http = inject(HttpClient);

  checkHealth(): Observable<{ status: string; version: string; language: string }> {
    return this.http.get<{ status: string; version: string; language: string }>(`${this.bridgeUrl}/health`);
  }

  scanDocument(options: ScanOptions = {}): Observable<Blob> {
    return this.http.post(`${this.bridgeUrl}/scan`, options, {
      responseType: 'blob'
    });
  }
}
```

---

### Option 3: React / Next.js (Custom Hook)

```tsx
import { useState, useCallback } from 'react';

const BRIDGE_URL = 'http://127.0.0.1:58260';

export function useScannerBridge() {
  const [loading, setLoading] = useState(false);
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const scan = useCallback(async (params = { source: 'Platen', resolution: 300 }) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${BRIDGE_URL}/scan`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(params)
      });

      if (!res.ok) throw new Error(await res.text() || 'Scan failed');

      const blob = await res.blob();
      const url = URL.createObjectURL(blob);
      setPdfUrl(url);
      return blob;
    } catch (err: any) {
      setError(err.message || 'Connection error');
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  return { scan, loading, pdfUrl, error };
}
```

---

### Option 4: Vue.js 3 (Composition API)

```vue
<template>
  <div class="scanner-widget">
    <button @click="triggerScan" :disabled="loading">
      {{ loading ? 'Scanning in progress...' : '📄 Scan Document' }}
    </button>
    <iframe v-if="pdfUrl" :src="pdfUrl" width="100%" height="480px" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const loading = ref(false);
const pdfUrl = ref<string | null>(null);

async function triggerScan() {
  loading.value = true;
  try {
    const res = await fetch('http://127.0.0.1:58260/scan', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ source: 'Platen', resolution: 300 })
    });
    const blob = await res.blob();
    pdfUrl.value = URL.createObjectURL(blob);
  } finally {
    loading.value = false;
  }
}
</script>
```

---

### Option 5: Node.js & Express.js

```javascript
const express = require('express');
const axios = require('axios');
const fs = require('fs');

const app = express();
const BRIDGE_URL = 'http://127.0.0.1:58260';

app.post('/api/trigger-scan', async (req, res) => {
  try {
    const scanResponse = await axios.post(`${BRIDGE_URL}/scan`, {
      source: req.body.source || 'Platen',
      resolution: 300
    }, {
      responseType: 'arraybuffer'
    });

    const filename = `scan_${Date.now()}.pdf`;
    fs.writeFileSync(`uploads/${filename}`, scanResponse.data);

    res.json({ success: true, filename, size: scanResponse.data.length });
  } catch (err) {
    res.status(500).json({ error: err.response?.data?.toString() || err.message });
  }
});

app.listen(3000, () => console.log('Server running on port 3000'));
```

---

### Option 6: cURL CLI

```bash
# 1. Health check
curl -X GET http://127.0.0.1:58260/health

# 2. Scanner status check
curl -X GET "http://127.0.0.1:58260/status?ip=192.168.3.133"

# 3. Perform scan and save to PDF file
curl -X POST http://127.0.0.1:58260/scan \
  -H "Content-Type: application/json" \
  -d '{"source": "Platen", "resolution": 300}' \
  --output scanned_document.pdf
```

---

## 🛠️ Building & Packaging

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (v1.75+)
- **macOS:** Xcode Command Line Tools (`xcode-select --install`)
- **Windows:** Visual Studio C++ Build Tools & WebView2 Runtime

### macOS Release (.app & .dmg)
```bash
chmod +x package-mac.sh
./package-mac.sh
```
Outputs in `dist/`:
- `dist/macOS/MMLAB Scanner Bridge.app`
- `dist/MMLAB-Scanner-Bridge-macOS.dmg`
- `dist/MMLAB-Scanner-Bridge-macOS.zip`

### Windows Release (.exe)
```cmd
build-windows.bat
```
Output:
- `dist\Windows\MMLAB Scanner Bridge.exe`

---

## 👤 Author
**e404r**  
- 🐙 GitHub: [@e404r](https://github.com/e404r)  
- 💼 LinkedIn: [jugheli](https://www.linkedin.com/in/jugheli/)

## 📄 License
This project is open-source software licensed under the [MIT License](LICENSE).
