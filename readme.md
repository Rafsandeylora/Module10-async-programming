## Experiment 1.1: Original timer from the book

### Tujuan
Eksperimen ini bertujuan untuk menjalankan contoh timer asynchronous dari Rust Async Book. Program menggunakan custom executor, spawner, task, dan waker untuk menjalankan sebuah future sampai selesai.

## Experiment 1.2: Understanding how it works.

### Tujuan
Eksperimen ini bertujuan untuk memahami bahwa `spawner.spawn(...)` tidak langsung menjalankan future sampai selesai. Saya menambahkan sebuah `println!` tepat setelah proses spawn untuk melihat urutan eksekusi program.