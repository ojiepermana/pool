# Langkah manual: `001-stack-and-workspace`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `001-stack-and-workspace.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F1-1` | Nyalakan Docker Desktop supaya build Linux bisa dijalankan sesi verify | spec 0001, kriteria build Linux (ID ditetapkan build notes) | belum ada | kapan saja; paling lambat sebelum `Verify it` | `NOT_RUN` |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| tidak ada | | | |

## Fitur 1: Stack and workspace (spec 0001)

### `F1-1` · Nyalakan Docker Desktop

Dari pemeriksaan lead (2026-11-14). Membuka bukti build Linux yang diminta [handoff](../../implementation-order.md#first-handoff-the-workspace); build di macOS bukan bukti itu.

- **Prasyarat:** `/Applications/Docker.app` terpasang (Docker 29.9.2); daemonnya mati saat koordinasi dimulai.
- **Jalankan:** buka Docker Desktop, lalu `docker info`
- **Lulus bila:** `docker info` mencetak bagian `Server:` tanpa error koneksi ke `docker.sock`.
- **Hanya langkah ini yang bisa membuktikan:** tanpa daemon, sesi verify mencatat build Linux `BLOCKED` dengan `owner: user`, dan perintah build persisnya ditambahkan di sini sebagai langkah manual berikutnya.
