# Langkah manual: `001-stack-and-workspace`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `001-stack-and-workspace.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F1-1` | Nyalakan Docker Desktop supaya build Linux bisa dijalankan sesi developer dan verify | `0001 AC-2` | F1 · `Scaffold from the decision`, `Verify it` | kapan saja | `NOT_RUN` |
| `F1-2` | Jalankan perintah build Linux spec 0001 (hanya bila sesi tidak sempat menjalankannya karena daemon mati) | `0001 AC-2`, klausa `/dev/full` `0001 AC-11` | F1 · `Scaffold from the decision`, `Verify it`, `Test it` (dua test khusus Linux) | `F1-1` lulus; sekarang juga, kode sudah di `main` | `NOT_RUN` |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| `F1-N1` | Arsitektur VM proxy dan PostgreSQL: `x86_64` atau ARM. Target `x86_64-unknown-linux-gnu` disimpulkan dari Proxmox VE (spec 0001 Follow-up 6). Bila ARM, sebut ke lead: `--platform` dan triple diganti lewat `architect (update)`. | `0001 AC-2` (tidak `BLOCKED`; berjalan dengan asumsi x86_64) | `docs/specs/0001-initial-architecture/index.md` bagian "Toolchain and Linux target" dan "Linux build" |

## Fitur 1: Stack and workspace (spec 0001)

### `F1-1` · Nyalakan Docker Desktop

Dari pemeriksaan lead (2026-11-14). Membuka bukti build Linux yang diminta [handoff](../../implementation-order.md#first-handoff-the-workspace); build di macOS bukan bukti itu.

- **Prasyarat:** `/Applications/Docker.app` terpasang (Docker 29.9.2); daemonnya mati saat koordinasi dimulai.
- **Jalankan:** buka Docker Desktop, lalu `docker info`
- **Lulus bila:** `docker info` mencetak bagian `Server:` tanpa error koneksi ke `docker.sock`.
- **Hanya langkah ini yang bisa membuktikan:** tanpa daemon, sesi verify mencatat build Linux `BLOCKED` dengan `owner: user`, dan perintah build persisnya ditambahkan di sini sebagai langkah manual berikutnya.

### `F1-2` · Jalankan build Linux spec 0001

Dari laporan `arc-f1-update` (2026-11-14). Bukti `0001 AC-2`. Sesi developer dan verify menjalankannya sendiri bila `docker info` berhasil; langkah ini hanya untukmu bila daemon masih mati saat mereka sampai di sana.

- **Prasyarat:** `F1-1` lulus; branch developer sudah di-merge ke `main`; `git status --short` tidak menampilkan perubahan pada file yang dilacak.
- **Jalankan:** dari root repo, blok `bash` persis di [spec 0001 bagian "Linux build"](../../specs/0001-initial-architecture/index.md#linux-build) (dimulai `git archive --format=tar HEAD | docker run --rm -i --platform linux/amd64 ...`). Unduhan image sekitar 320 MB; di Apple silicon berjalan lewat emulasi, beberapa menit.
- **Lulus bila:** exit 0, output memuat `host: x86_64-unknown-linux-gnu`, test lulus (termasuk `a_failed_write_exits_3_without_stderr` dan `ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr`, yang hanya berjalan di Linux), satu blok `== <fixture> <format> exit=<n>` per fixture dan format (0 untuk `valid-*`, 1 untuk lainnya), tidak ada `canary leaked`, dan baris terakhir `linux build check passed`.
- **Kirim ke lead:** `PASS` atau `FAIL`, baris `sha256sum`, dan `git rev-parse HEAD`; bila `FAIL`, outputnya.
