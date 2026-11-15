# Langkah manual: `004-one-real-primary-path`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `004-one-real-primary-path.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F4-1` | Izin menulis ke GitHub untuk scope ini: lead mem-push `main` lokal ke branch `scope/004-one-real-primary-path`, membuka pull request dengan `gh`, menunggu check wajib, lalu `gh pr merge --merge` (jalur perubahan spec 0002; `main` menolak push langsung). Penutup scope 003 sudah masuk lewat pull request 5 (`673f91e`), jadi pull request ini hanya membawa pekerjaan scope 004 | bukti CI fitur 4: check wajib pada pull request | belum ada | kapan saja; dibutuhkan paling lambat saat penutupan | `NOT_RUN`; ditanyakan ke user pada 2026-11-15, belum dijawab |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| - | belum ada | - | - |

## Fitur `4`: `One real primary path` (spec `0004`)

Belum ada langkah selain `F4-1`. Langkah baru ditambahkan dari laporan sub sesi.

### Terbawa dari scope 003, masih keputusanmu

- Spec 0003 Follow-up 11: batas jumlah finding dari satu file konfigurasi. Spec 0003 menulis bahwa kamu memutuskannya sebelum scope 004 membuat daemon memanggil `validate` saat start. Di mode `auto` architect scope 004 mengambil opsi rekomendasi (buffer keluaran sekarang, plus batas atas jumlah finding) dan menandainya `berat`; barisnya muncul di "Keputusan delegasi" log dan bisa kamu batalkan.
- Spec 0003 Follow-up 13: catatan di `PRD-006 AC-2` tentang node karantina. Tidak ada sesi yang boleh mengubah `docs/prd/`; tetap menunggu kamu dan tidak menahan scope 004.
