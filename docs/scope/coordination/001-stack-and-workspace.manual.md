# Langkah manual: `001-stack-and-workspace`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `001-stack-and-workspace.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F1-1` | Nyalakan Docker Desktop supaya build Linux bisa dijalankan sesi developer dan verify | `0001 AC-2` | F1 · `Scaffold from the decision`, `Verify it` | kapan saja | `PASS` 2026-11-14: user menyalakan Docker Desktop, `docker info` exit 0 (server 29.9.2) |
| `F1-2` | Jalankan perintah build Linux spec 0001 (hanya bila sesi tidak sempat menjalankannya karena daemon mati) | `0001 AC-2`, klausa `/dev/full` `0001 AC-11` | F1 · `Scaffold from the decision`, `Verify it`, `Test it` (dua test khusus Linux) | `F1-1` lulus; sekarang juga, kode sudah di `main` | `PASS` 2026-11-14 pada `18e9228`, dijalankan sesi `chk-f1-verify-2` dan lead (bukan user): exit 0, 136 test, `0481be185960bc48d842a22a70c14e709a66581c95fdd4f1a0ccd09444f7bdb3  target/release/pool`; bukti di `verify.md` bagian "Linux evidence: container build" |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| `F1-N1` | Arsitektur VM proxy dan PostgreSQL: `x86_64` atau ARM. **Dijawab user 2026-11-14: ARM.** Target spec 0001 diganti ke `aarch64-unknown-linux-gnu` (`--platform linux/arm64`) lewat `architect (update)`, dan build Linux diulang untuk target itu. | `0001 AC-2`: `PASS` untuk `aarch64` pada `41e8eda` | `docs/specs/0001-initial-architecture/index.md` bagian "Toolchain and Linux target" dan "Linux build" (sudah diperbarui) |

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
- **Sesudah jawaban `F1-N1` (ARM):** blok di spec kini memakai `--platform linux/arm64` dan mengharapkan `host: aarch64-unknown-linux-gnu`, tanpa emulasi di Apple silicon; unduhan sekitar 283 MB. Baris "Jalankan" dan "Lulus bila" di atas adalah bunyi langkah ini saat target masih `x86_64`. Run ulang untuk `aarch64` dijalankan sesi `chk-f1-verify-3` pada `41e8eda`: `PASS`, 136 test, `cb184231b4af3e5c927eb46bb1fd7ad289b5a357a7c8b62a14d45fd6b92d3532  target/release/pool`.
