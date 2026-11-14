# Langkah manual: `002-standards-and-tooling`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `002-standards-and-tooling.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F2-1` | Izin menulis ke GitHub untuk scope ini (push `main`, push branch uji, buka PR uji, pasang branch protection) | spec 0002, AC ditetapkan architect | belum ada | diberikan | `PASS` 2026-11-14: user menjawab "Ya, izin GitHub"; langkah remote dijalankan lead sesudah spec 0002 diterima dan pemeriksaan ada di `main` |
| `F2-2` | Nyalakan Docker Desktop (sama dengan `F1-1` di `001-stack-and-workspace.manual.md`) supaya pemeriksaan yang butuh container bisa dijalankan sesi | spec 0002, AC ditetapkan architect; `0001 AC-2` | belum ada | kapan saja | `PASS` 2026-11-14: Docker Desktop dinyalakan user untuk `F1-1`; `docker info` exit 0 dicek lead scope 002 |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| `F2-N1` | Roster reviewer: nama pemegang 12 peran (Security engineer, Protocol engineer, Database architect, HA engineer, Infrastructure operator, Systems engineer, SRE, Control engineer, Performance engineer, Maintainer, Release maintainer, Project lead), atau konfirmasi tetap `Not assigned` | `0002 AC-18` (lulus dengan `Not assigned`; gerbang HA dan infrastruktur tetap terbuka sampai ada nama) | `docs/engineering-standards.md` bagian "Reviewer roster" |
| `F2-N2` | Konfirmasi daftar lisensi awal sebagai pemilik repo: `Apache-2.0`, `MIT`, `Unicode-3.0`, plus pengecualian `NCSA` untuk `libfuzzer-sys` (keputusan delegasi `berat`) | `0002 AC-4` (tidak `BLOCKED`; berjalan dengan daftar itu) | `deny.toml` |
| `F2-N3` | Apakah dan kapan ada akun GitHub maintainer kedua (approval wajib naik dari 0 ke 1) | tidak ada AC yang menunggu | `.github/rulesets/main.json` |
| `F2-N4` | Kanal laporan keamanan privat dan tim maintainer (private vulnerability reporting di repo saat ini mati) | tidak ada AC spec 0002 (Follow-up 5) | `SECURITY.md`, `CONTRIBUTING.md` |

## Fitur 2: Standards and tooling (spec 0002)

### `F2-1` · Izin menulis ke GitHub

Dijawab 2026-11-14: "Ya, izin GitHub". Lead menjalankan langkah remote scope ini; sub sesi tetap tidak menulis ke GitHub. Izin ini hanya untuk scope 002 dan hanya untuk empat hal ini: push `main`, push branch uji, membuka PR uji, dan memasang branch protection.

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
