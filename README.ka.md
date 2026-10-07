# 🖨️ MMLAB Scanner Bridge (ქართული ვერსია)
> **სკანერის მართვის ლოკალური ცენტრი & REST API ხიდი ვებ-აპლიკაციებისთვის**  
> *English version is available at [README.md](README.md)*

<p align="center">
  <img src="icon_128.png" alt="MMLAB Scanner Bridge Logo" width="100" height="100" />
</p>

<p align="center">
  <a href="#-მიმოხილვა">📖 მიმოხილვა</a> •
  <a href="#-ძირითადი-ფუნქციები">✨ ფუნქციები</a> •
  <a href="#-rest-api-სრული-ენდპოინტები">🔌 API ენდპოინტები</a> •
  <a href="#-ინტეგრაცია-ვებ-აპლიკაციაში">💻 ვებ ინტეგრაცია</a> •
  <a href="#-აწყობა-და-პაკეტირება">🛠️ Build & Install</a>
</p>

---

## 📖 მიმოხილვა
**MMLAB Scanner Bridge** არის მაღალმწარმოებლური, ულტრა-მსუბუქი დესკტოპ აპლიკაცია და ლოკალური REST სერვისი (დაწერილი Rust-ზე), რომელიც წარმოადგენს უსაფრთხო საკომუნიკაციო ხიდს ნებისმიერ ვებ აპლიკაციასა (მაგ. ERP, CRM, დოკუმენტბრუნვის სისტემები) და ქსელურ პრინტერებს/სკანერებს შორის.

თანამედროვე ბრაუზერების უსაფრთხოების მკაცრი პოლიტიკა (**CORS** და **Private Network Access - PNA**) კრძალავს ვებ გვერდებიდან პირდაპირ HTTP/TCP კავშირს ლოკალური ქსელის მოწყობილობებთან (პრინტერის IP-ზე). **MMLAB Scanner Bridge** სრულად ხსნის ამ ბარიერს:
- უშვებს ლოკალურ სერვერს `http://127.0.0.1:58260`-ზე სრული CORS მხარდაჭერით (`Access-Control-Allow-Origin: *`).
- იღებს ბრძანებებს თქვენი ვებ-აპლიკაციიდან.
- აპარატურასთან კომუნიკაციას ახდენს უშუალოდ eSCL პროტოკოლით.
- ვებ აპლიკაციას უბრუნებს დასკანირებულ მზა **PDF** ფაილს (`application/pdf`).

---

## ✨ ძირითადი ფუნქციები
- **სრულიად დრაივერის გარეშე (Driverless eSCL):** იყენებს Apple AirScan / Mopria საერთაშორისო სტანდარტს HTTP/XML-ზე. თავსებადია პრაქტიკულად ყველა თანამედროვე ქსელურ HP (LaserJet, OfficeJet, PageWide), Canon, Epson, Brother და Xerox მრავალფუნქციურ აპარატთან.
- **მშობლიური Desktop GUI ფანჯარა:** ჩაშენებული დამოუკიდებელი მართვის ცენტრი macOS (WebKit) და Windows (WebView2) სისტემებისთვის — Electron-ის გარეშე (მთლიანი აპლიკაცია იწონის სულ რამდენიმე მეგაბაიტს).
- **System Tray / Menu Bar Daemon:** ფანჯრის დახურვა (`X`) არ თიშავს აპლიკაციას; ის გადადის ფონურ რეჟიმში და აგრძელებს ვებ-მოთხოვნების მომსახურებას.
- **სისტემური ენის დეტექცია (Dynamic Locale Engine):** სისტემის ენის (ქართული/ინგლისური) ავტომატური აღმოჩენა OS პარამეტრებიდან, ან ხელით გაშვება `--lang ka` / `--lang en` დროშებით.
- **სკანერების ძებნა (mDNS Discovery):** ლოკალურ ქსელში არსებული eSCL სკანერების ავტომატური პოვნა (ZeroConf).
- **მუქი / ნათელი თემა:** Dark & Light თემების მხარდაჭერა.
- **სატესტო სკანირება & Live PDF Preview:** Flatbed (მინა) და Feeder (ADF დოკუმენტების ავტომატური მიმწოდებელი), რეზოლუცია (150, 300, 600 DPI), ორმხრივი (Duplex) სკანირება.
- **მოწყობილობის რეალურ დროში მონიტორინგი:** ლაივ პინგი, ADF სენსორის შემოწმება (დევს თუ არა ფურცელი), შეყოვნების (Latency) გაზომვა.
- **ჭკვიანი პორტის კონტროლი (Smart Port Check):** გაშვებამდე ამოწმებს პორტს `58260`. თუ აპლიკაცია უკვე გაშვებულია, ავტომატურად ხსნის არსებულ მართვის პანელს ბრაუზერში და არ იწვევს კონფლიქტს.
- **უსაფრთხოების ტოკენი (`api_sec_print`):** სურვილისამებრ ჩართვადი ავტორიზაცია REST API-ზე.

---

## 🔌 REST API: სრული ენდპოინტები

სერვისი ლოკალურად ხელმისაწვდომია მისამართზე: **`http://127.0.0.1:58260`**  
ყველა ენდპოინტს აქვს CORS მხარდაჭერა (`Access-Control-Allow-Origin: *`), რაც საშუალებას გაძლევთ პირდაპირ გამოიძახოთ ნებისმიერი ვებ გვერდიდან (`fetch`, `axios`, `HttpClient`).

| მეთოდი | ენდპოინტი | აღწერა | დანიშნულება |
| :--- | :--- | :--- | :--- |
| `GET` | `/health` | სერვისის სტატუსი | შეამოწმეთ, გაშვებულია თუ არა Scanner Bridge კომპიუტერზე |
| `GET` | `/status` | სკანერის დიაგნოსტიკა | ამოწმებს პრინტერის ხელმისაწვდომობას, ADF სენსორს და პინგს |
| `GET` | `/discover` | **სკანერების ძებნა (mDNS)** | ლოკალურ ქსელში eSCL სკანერების ავტომატური აღმოჩენა (ZeroConf) |
| `POST` | `/scan` | **სკანირების დაწყება** | იღებს პარამეტრებს, ასკანირებს და აბრუნებს PDF ბინარულ ნაკადს |
| `GET` | `/config` | პარამეტრების წაკითხვა | აბრუნებს ამჟამად კონფიგურირებულ პრინტერის IP-ს, პორტს და `api_sec_print`-ს |
| `POST` | `/config` | პარამეტრების შენახვა | ანახლებს პრინტერის პარამეტრებს (`scanner_config.json`) |
| `GET` | `/style.css` | სტილები | აბრუნებს მართვის პანელის CSS სტილებს |
| `GET` | `/open-url?url=...` | ბრაუზერში გახსნა | ხსნის გადაცემულ URL-ს მომხმარებლის ნაგულისხმევ ბრაუზერში |
| `GET` | `/icon.png` | ლოგო | აბრუნებს აპლიკაციის ოფიციალურ PNG აიქონს |
| `GET` | `/` ან `/dashboard` | ვებ მართვის პანელი | ხსნის ჩაშენებულ ინტერაქციულ მართვის პანელს |

---

### 1. `GET /health` — სერვისის შემოწმება
გამოიყენეთ იმის დასადგენად, გაშვებულია თუ არა მომხმარებლის კომპიუტერზე MMLAB Scanner Bridge.

**Response (200 OK):**
```json
{
  "status": "ok",
  "app": "hr-scanner-bridge",
  "version": "1.0.0",
  "port": 58260,
  "language": "ka"
}
```

---

### 2. `GET /status` — პრინტერის/სკანერის სტატუსი
ამოწმებს კავშირს სკანერთან eSCL პროტოკოლით.  
*პარამეტრები (Query Params, არასავალდებულო):*
- `ip`: პრინტერის IP მისამართი (თუ არ გადაეცემა, იღებს შენახული კონფიგურაციიდან).
- `port`: პორტი (ნაგულისხმევი: `80`).

**მოთხოვნის მაგალითი:**
```http
GET http://127.0.0.1:58260/status?ip=192.168.3.133&port=80
```

**Response (200 OK — ონლაინ რეჟიმი):**
```json
{
  "online": true,
  "state": "Idle",
  "ip": "192.168.3.133",
  "port": 80,
  "error": null
}
```

**Response (200 OK — ოფლაინ / შეცდომა):**
```json
{
  "online": false,
  "state": "Offline",
  "ip": "192.168.3.133",
  "port": 80,
  "error": "მოწყობილობა მიუწვდომელია (connection timed out)"
}
```

---

### 3. `GET /discover` — სკანერების ავტომატური ძებნა (mDNS ZeroConf)
აგზავნის Multicast DNS მოთხოვნას ლოკალურ ქსელში (`_uscan._tcp.local`) და 2 წამში პოულობს ყველა ხელმისაწვდომ eSCL სკანერს IP-ის ხელით ჩაწერის გარეშე.

**მოთხოვნა:**
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

### 4. `POST /scan` — დოკუმენტის სკანირება (PDF დაბრუნება)
აგზავნის სკანირების ბრძანებას აპარატთან და აბრუნებს დასკანირებულ მზა PDF ფაილს.

**Headers:**
```http
Content-Type: application/json
api_sec_print: sec_prn_your_secret_token (არასავალდებულო, მხოლოდ თუ კონფიგურირებულია)
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

**პარამეტრების განმარტება:**
- `source` *(string, არასავალდებულო)*: `"Platen"` (მინა / Flatbed) ან `"Feeder"` (ავტომატური მიმწოდებელი / ADF). ნაგულისხმევი: `"Platen"`.
- `resolution` *(number, არასავალდებულო)*: `150`, `300` ან `600`. ნაგულისხმევი: `300` DPI.
- `duplex` *(boolean, არასავალდებულო)*: `true` ან `false`. ორმხრივი სკანირება (მუშაობს მხოლოდ მაშინ, როცა `source: "Feeder"` და პრინტერს აქვს Duplex-ის მხარდაჭერა).
- `ip` *(string, არასავალდებულო)*: პრინტერის IP მისამართი.
- `port` *(number, არასავალდებულო)*: პრინტერის პორტი (ნაგულისხმევი: `80`).
- `api_sec_print` *(string, არასავალდებულო)*: თავდაცვითი უსაფრთხოების კოდი (თუ პარამეტრებში ჩართულია).

**Response (200 OK):**
- **Content-Type:** `application/pdf`
- **Content-Disposition:** `inline; filename="scanned_doc.pdf"`
- **Body:** Binary Stream (ნამდვილი PDF დოკუმენტის ბაიტები).

**Response (401 Unauthorized — არასწორი ტოკენის დროს):**
```
არასანქცირებული წვდომა: არასწორი ან გამოტოვებული api_sec_print უსაფრთხოების კოდი (401 Unauthorized)
```

---

### 5. `GET /config` & `POST /config` — პარამეტრების მართვა
საშუალებას გაძლევთ წაიკითხოთ ან პროგრამულად შეცვალოთ პრინტერის ნაგულისხმევი მონაცემები და თავდაცვითი კოდი.

**GET /config:**
```json
{
  "printer_ip": "192.168.3.133",
  "printer_port": 80,
  "printer_name": "HP LaserJet Pro MFP 4103",
  "api_sec_print": "sec_prn_a89bc2",
  "language": "ka"
}
```

---

## 💻 ინტეგრაცია ვებ-აპლიკაციაში

### ვარიანტი 1: სუფთა JavaScript / TypeScript (`fetch` API)

```typescript
// 1. სკანირება
async function scanDocument() {
  const response = await fetch('http://127.0.0.1:58260/scan', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      source: 'Feeder',     // 'Platen' ან 'Feeder'
      resolution: 300,      // 150, 300, 600 DPI
      duplex: false
    })
  });

  if (!response.ok) {
    throw new Error(await response.text());
  }

  // 2. ვიღებთ მზა PDF Blob-ს
  const pdfBlob = await response.blob();
  const fileUrl = URL.createObjectURL(pdfBlob);
  window.open(fileUrl, '_blank');
}
```

---

### ვარიანტი 2: Angular (TypeScript Standalone Service)

```typescript
import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class ScannerBridgeService {
  private readonly bridgeUrl = 'http://127.0.0.1:58260';
  private readonly http = inject(HttpClient);

  checkHealth(): Observable<{ status: string; version: string; language: string }> {
    return this.http.get<{ status: string; version: string; language: string }>(`${this.bridgeUrl}/health`);
  }

  scanDocument(options = { source: 'Platen', resolution: 300 }): Observable<Blob> {
    return this.http.post(`${this.bridgeUrl}/scan`, options, { responseType: 'blob' });
  }
}
```

---

### ვარიანტი 3: React / Next.js Hook

```tsx
import { useState } from 'react';

export function useScanner() {
  const [loading, setLoading] = useState(false);
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);

  const scan = async () => {
    setLoading(true);
    try {
      const res = await fetch('http://127.0.0.1:58260/scan', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ source: 'Platen', resolution: 300 })
      });
      const blob = await res.blob();
      setPdfUrl(URL.createObjectURL(blob));
    } finally {
      setLoading(false);
    }
  };

  return { scan, loading, pdfUrl };
}
```

---

### ვარიანტი 4: Vue.js 3

```vue
<template>
  <button @click="scan" :disabled="loading">
    {{ loading ? 'სკანირება...' : 'დოკუმენტის სკანირება' }}
  </button>
  <iframe v-if="pdfUrl" :src="pdfUrl" width="100%" height="450px" />
</template>

<script setup lang="ts">
import { ref } from 'vue';

const loading = ref(false);
const pdfUrl = ref<string | null>(null);

async function scan() {
  loading.value = true;
  const res = await fetch('http://127.0.0.1:58260/scan', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ source: 'Platen', resolution: 300 })
  });
  const blob = await res.blob();
  pdfUrl.value = URL.createObjectURL(blob);
  loading.value = false;
}
</script>
```

---

### ვარიანტი 5: Node.js / Express.js

```javascript
const express = require('express');
const axios = require('axios');
const fs = require('fs');

const app = express();

app.post('/api/trigger-scan', async (req, res) => {
  try {
    const scanResponse = await axios.post('http://127.0.0.1:58260/scan', {
      source: 'Platen',
      resolution: 300
    }, { responseType: 'arraybuffer' });

    fs.writeFileSync(`uploads/scan_${Date.now()}.pdf`, scanResponse.data);
    res.json({ success: true, size: scanResponse.data.length });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.listen(3000, () => console.log('Server running on port 3000'));
```

---

## 🛠️ აწყობა და პაკეტირება

### წინაპირობები
- [Rust & Cargo](https://rustup.rs/) (v1.75+)
- **macOS:** Xcode Command Line Tools (`xcode-select --install`)
- **Windows:** Visual Studio C++ Build Tools & WebView2 Runtime

### macOS Release (.app & .dmg)
```bash
chmod +x package-mac.sh
./package-mac.sh
```
შედეგი `dist/` საქაღალდეში:
- `dist/macOS/MMLAB Scanner Bridge.app`
- `dist/MMLAB-Scanner-Bridge-macOS.dmg`
- `dist/MMLAB-Scanner-Bridge-macOS.zip`

### Windows Release (.exe)
```cmd
build-windows.bat
```
შედეგი:
- `dist\Windows\MMLAB Scanner Bridge.exe`

---

## 👤 ავტორი
**e404r**  
- 🐙 GitHub: [@e404r](https://github.com/e404r)  
- 💼 LinkedIn: [jugheli](https://www.linkedin.com/in/jugheli/)

## 📄 ლიცენზია
ეს პროექტი ვრცელდება [MIT ლიცენზიით](LICENSE).
