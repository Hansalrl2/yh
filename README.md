# ToAnime Converter

![GitHub](https://img.shields.io/badge/Version-1.0-blue)
![Vercel](https://img.shields.io/badge/Deployed%20on-Vercel-brightgreen)

ToAnime Converter adalah aplikasi web yang mengonversi gambar menjadi gaya anime menggunakan teknologi _web scraping_ dan _machine learning_. Aplikasi ini mendukung berbagai bahasa pemrograman seperti HTML, CSS, JavaScript, Node.js, TypeScript, Go, Rust, dan lainnya.

---

## Fitur Utama

- **Konversi Gambar ke Anime**: Menggunakan layanan dari situs https://taoanhdep.com untuk mengubah gambar menjadi gaya anime.
- **Multi-Bahasa Backend**: Mendukung backend dengan Node.js, Go, Rust, Python, dll.
- **Frontend Interaktif**: Antarmuka pengguna yang responsif dibangun menggunakan HTML, CSS, dan JavaScript.
- **Deployment Otomatis**: Mudah di-deploy ke Vercel.

---

## Struktur Proyek

```bash
project/
├── frontend/               # Frontend menggunakan HTML, CSS, JavaScript
│   ├── index.html          # Halaman utama aplikasi
│   ├── styles.css          # Styling untuk halaman
│   └── app.js              # Logika interaksi frontend
├── backend/                # Backend menggunakan Node.js, Go, Rust, dll.
│   ├── nodejs/             # Node.js handler
│   │   ├── server.ts       # Server utama Node.js
│   ├── golang/             # Go handler
│   │   ├── main.go         # Server utama Go
│   ├── rust/               # Rust handler
│   │   ├── Cargo.toml      # Konfigurasi Rust
│   │   ├── src/main.rs     # Server utama Rust
│   └── vercel.json         # Konfigurasi deployment Vercel
└── README.md               # Dokumentasi proyek
