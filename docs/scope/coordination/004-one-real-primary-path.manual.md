# Langkah manual: `004-one-real-primary-path`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `004-one-real-primary-path.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F4-1` | Izin menulis ke GitHub untuk scope ini: lead mem-push `main` lokal ke branch `scope/004-one-real-primary-path`, membuka pull request dengan `gh`, menunggu check wajib, lalu `gh pr merge --merge` (jalur perubahan spec 0002; `main` menolak push langsung). Penutup scope 003 sudah masuk lewat pull request 5 (`673f91e`), jadi pull request ini hanya membawa pekerjaan scope 004 . Spec 0004 langkah manual M3 menambah: sesudah sembilan check berjalan sekali di pull request itu, `main.json` diterapkan ke ruleset di remote | bukti CI fitur 4: check wajib pada pull request, termasuk run CI `integration` dan `fuzz` (`0004 AC-26`, `AC-27`) | `Build it`, milestone "Fuzzing, documents and evidence", `Verify it` (2026-11-16, `b2aec30`), `Review it` (2026-11-16) | kapan saja; dibutuhkan paling lambat saat penutupan | `NOT_RUN`; ditanyakan ke user pada 2026-11-15, belum dijawab |
| `F4-2` | Keputusan lisensi (spec 0004 M1): bolehkah `ISC` dan `BSD-3-Clause` masuk pohon dependensi, hanya untuk `aws-lc-rs 1.19.0`, `aws-lc-sys 0.47.0`, `rustls-webpki 0.103.17`, `untrusted 0.9.0`, dan `subtle 2.6.1` | `0004 AC-3` | belum ada; menahan merge Build plan langkah 3 | sekarang | `PASS` 2026-11-15: user menjawab "Ya, izinkan" untuk `ISC` dan `BSD-3-Clause`, hanya untuk lima paket itu |
| `F4-3` | Job CI dan file ruleset (spec 0004 M2): tambah job `integration` dan `fuzz` ke `.github/workflows/ci.yml` dan dua context ke `.github/rulesets/main.json` dalam satu commit | `0004 AC-26`, `AC-27` (bukti CI) | `Build it`, milestone "Fuzzing, documents and evidence", `Verify it` (2026-11-16, `b2aec30`), `Review it` (2026-11-16) | Build plan langkah 12 di `main` | `NOT_RUN`; ditanyakan ke user pada 2026-11-16, belum dijawab. Reviewer (tinjauan ketiga, 2026-11-16) menyarankan job `integration` dijalankan sekali di CI sebelum fitur disebut selesai: itu kali pertama fixture bertemu host Linux. Yang sudah ada: check penuh di macOS di bawah `/bin/dash` dan `/bin/bash` tanpa terminal, dan fungsi skrip di container image yang di-pin |
| `F4-4` | Docker berjalan (spec 0004 M4) untuk `scripts/check.sh integration`, `linux-build`, dan `prove-checks.sh --with-linux-build` | semua kriteria yang dibuktikan di fixture (`0004 AC-9` sampai `AC-26`) | belum ada | kapan saja | berjalan pada 2026-11-15 (`docker info` exit 0); tetap tanggung jawabmu |
| `F4-5` | Laporan pemindai secret atau review otomatis di remote atas `crates/pool-core/tests/fixtures/node.key` (kunci privat test yang sengaja di-commit, keputusan 62): menutup laporan itu | `0004 AC-15` (cek TLS hermetic) | `Build it`, milestone "Fuzzing, documents and evidence", `Verify it` (2026-11-16, `b2aec30`), `Review it` (2026-11-16) | saat pull request scope 004 dibuka | `NOT_RUN`; jawabannya ada di spec 0004 "Tests and who owns them", paragraf "One test key is committed" |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| - | belum ada | - | - |

## Fitur `4`: `One real primary path` (spec `0004`)

Teks lengkap tiap langkah (tabel paket, YAML job CI, perintah ruleset) ada di [spec 0004, "CI definition and manual steps"](../../specs/0004-one-real-primary-path/index.md#ci-definition-and-manual-steps); bagian di bawah meringkasnya.

### `F4-2` · Keputusan lisensi

Dari ringkasan `arc-f4-spec` (2026-11-15). Membuktikan `0004 AC-3`.

- **Prasyarat:** tidak ada.
- **Jalankan:** jawab ya atau tidak ke lead. Dua lisensi baru: `ISC` (`aws-lc-rs`, `aws-lc-sys`, `rustls-webpki`, `untrusted`) dan `BSD-3-Clause` (`aws-lc-sys`, `subtle`). Keduanya permisif dan cocok dengan Apache 2.0. SQLite sendiri public domain dan masuk lewat `libsqlite3-sys` (MIT).
- **Lulus bila:** kamu menjawab ya. Akibatnya lima `[[licenses.exceptions]]` per paket di `deny.toml` (daftar lisensi umum tidak berubah) dan file `NOTICE` yang menyebut SQLite dan AWS-LC. Jawaban tidak untuk AWS-LC berarti runner-up `ring`, yang butuh dua lisensi yang sama plus satu pengecualian lagi; tidak untuk `ISC` sendiri menghentikan keputusan TLS spec 0001 dan kembali ke kamu.
- **Hanya langkah ini yang bisa membuktikan:** spec 0003 Follow-up 3 meminta keputusan orang atas tiap lisensi baru; `scripts/check.sh deny` hanya memeriksa bahwa file kebijakan diikuti.

### `F4-3` · Job CI `integration` dan `fuzz`

Dari ringkasan `arc-f4-spec` (2026-11-15). Membuktikan run CI untuk `0004 AC-26` dan `AC-27`.

- **Prasyarat:** Build plan langkah 12 di `main`.
- **Jalankan:** tambahkan dua job dari spec ke `.github/workflows/ci.yml` sesudah job `docs`, dan dua context ke `required_status_checks` di `.github/rulesets/main.json`, dalam satu commit. Tidak ada sub sesi yang mengubah `.github/`; lead bisa melakukannya atas izinmu.
- **Lulus bila:** `scripts/check.sh guards` lulus (rule C1 sampai C5) dan kedua job berjalan di pull request scope 004.
- **Hanya langkah ini yang bisa membuktikan:** bahwa check `integration` dan `fuzz` berjalan di runner CI, bukan hanya di workstation.

### Terbawa dari scope 003, masih keputusanmu

- Spec 0003 Follow-up 11: batas jumlah finding dari satu file konfigurasi. Spec 0003 menulis bahwa kamu memutuskannya sebelum scope 004 membuat daemon memanggil `validate` saat start. Di mode `auto` architect scope 004 mengambil opsi rekomendasi (buffer keluaran sekarang, plus batas atas jumlah finding) dan menandainya `berat`; barisnya muncul di "Keputusan delegasi" log dan bisa kamu batalkan.
- Spec 0003 Follow-up 13: catatan di `PRD-006 AC-2` tentang node karantina. Tidak ada sesi yang boleh mengubah `docs/prd/`; tetap menunggu kamu dan tidak menahan scope 004.
