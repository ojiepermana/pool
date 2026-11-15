# Langkah manual: `003-control-and-session-data-model`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `003-control-and-session-data-model.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F3-1` | Izin menulis ke GitHub untuk scope ini: lead mem-push `main` lokal ke branch `scope/003-control-and-session-data-model`, membuka pull request dengan `gh`, menunggu tujuh check, lalu `gh pr merge --merge` (jalur perubahan spec 0002; `main` menolak push langsung) | tidak ada; ini jalur perubahan, bukan kriteria spec 0003 | belum ada | kapan saja; dibutuhkan paling lambat saat penutupan | `NOT_RUN` |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| tidak ada | | | |

## Fitur `3`: `Control and session data model` (spec `0003`)

### `F3-1` · Izin menulis ke GitHub untuk scope 003

Dari lead (`2026-11-15`). Tidak membuktikan kriteria spec; ini syarat agar hasil scope 003 sampai ke `origin/main`.

- **Prasyarat:** `gh auth status` masuk sebagai pemilik repo; `git status --short` kosong di checkout utama; `HEAD` = `main`.
- **Jalankan:** cukup jawab lead "Ya, izin GitHub" (lead yang menjalankan), atau jalankan sendiri: `git push origin main:scope/003-control-and-session-data-model`, `gh pr create --base main --head scope/003-control-and-session-data-model`, tunggu `gh pr checks scope/003-control-and-session-data-model`, `gh pr merge scope/003-control-and-session-data-model --merge`, `git pull --ff-only origin main`.
- **Lulus bila:** tujuh check (`fmt`, `clippy`, `test`, `deny`, `linux-build`, `guards`, `docs`) lulus pada pull request itu, pull request ter-merge dengan merge commit, dan `git status -sb | head -1` menampilkan `## main...origin/main` tanpa selisih.
- **Hanya langkah ini yang bisa membuktikan:** bahwa perubahan scope 003 lulus CI di runner dan sudah ada di `origin/main`. Izin dari scope 002 (`F2-1`) hanya berlaku untuk scope 002.
