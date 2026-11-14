# Langkah manual: `002-standards-and-tooling`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `002-standards-and-tooling.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F2-1` | Izinkan lead menulis ke GitHub untuk scope ini (push branch uji, buka PR uji, pasang branch protection), atau lakukan sendiri langkah PR yang nanti dirinci di sini | spec 0002, AC ditetapkan architect | belum ada | kapan saja; rincian langkah PR ditambahkan sesudah spec 0002 diterima | `NOT_RUN` |
| `F2-2` | Nyalakan Docker Desktop (sama dengan `F1-1` di `001-stack-and-workspace.manual.md`) supaya pemeriksaan yang butuh container bisa dijalankan sesi | spec 0002, AC ditetapkan architect; `0001 AC-2` | belum ada | kapan saja | `NOT_RUN` |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| `F2-N1` | Roster reviewer: nama orang (atau "model baru") untuk tiap peran review di [standards](../../engineering-standards.md#security-and-reviews) dan [risk register](../../plan/risk-register.md): security, protocol, database/infrastructure, maintainer, release maintainer, project lead | spec 0002, AC roster (nomor ditetapkan architect) | ditetapkan spec 0002 |

## Fitur 2: Standards and tooling (spec 0002)

### `F2-1` · Izin menulis ke GitHub, atau jalankan langkah PR sendiri

Dari pemeriksaan lead (2026-11-14). Kotak "Verify the required checks on a pull request and link the result" butuh PR nyata: pemeriksaan harus terbukti gagal pada perubahan yang sengaja salah dan lulus pada perubahan yang benar, di runner CI, bukan di Mac ini.

- **Prasyarat:** spec 0002 diterima dan pemeriksaan sudah di `main` lokal. Keadaan sekarang: `main` di `origin` belum diproteksi, Actions aktif, belum ada workflow dan belum ada PR.
- **Jalankan:** balas lead dengan salah satu: (a) "izin GitHub": lead mem-push `main`, mem-push satu branch uji, membuka PR uji dengan `gh`, dan memasang branch protection sesuai spec 0002; atau (b) kamu menjalankan sendiri langkah yang dirinci di file ini sesudah spec diterima.
- **Lulus bila:** tautan PR dan run CI tercatat di `verify.md` spec 0002.
- **Hanya langkah ini yang bisa membuktikan:** workflow benar-benar berjalan di runner, required checks dan aturan review benar-benar menahan merge.

### `F2-2` · Nyalakan Docker Desktop

Dari pemeriksaan lead (2026-11-14): `docker info` gagal (daemon mati). Lihat `F1-1` di [`001-stack-and-workspace.manual.md`](001-stack-and-workspace.manual.md); satu kali menyalakan Docker membuka keduanya.

- **Prasyarat:** `/Applications/Docker.app` terpasang (Docker 29.9.2).
- **Jalankan:** buka Docker Desktop, lalu `docker info`
- **Lulus bila:** `docker info` mencetak bagian `Server:` tanpa error koneksi.
- **Hanya langkah ini yang bisa membuktikan:** pemeriksaan lokal yang memakai container (build Linux spec 0001, PostgreSQL 18 untuk integration test bila spec 0002 memilih container).
