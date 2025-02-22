# Katalon API

## Deskripsi Proyek
Proyek ini bertujuan untuk menguji API menggunakan Katalon Studio. Pengujian mencakup berbagai endpoint untuk memastikan respons dan fungsionalitas sesuai dengan spesifikasi.

## Struktur Proyek
- **Test Cases**: Berisi skenario pengujian individual untuk setiap endpoint.
- **Object Repository**: Menyimpan definisi objek yang berinteraksi dengan API.
- **Test Suites**: Kumpulan test case yang dapat dijalankan secara bersamaan untuk pengujian menyeluruh.

![image](https://github.com/user-attachments/assets/7721e89c-d5d7-484c-bd36-9b3b481ea363)


## Penjelasan Testing 
- **POST Log In User**: untuk memverifikasi API kredensial pengguna dengan benar dan memberikan respons dengan status **200 OK**. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/login

![image](https://github.com/user-attachments/assets/47e5bc5b-6c66-4cef-970c-4a7122e42f8a)

- **GET User Profile**: untuk mengambil data API profil pengguna yang sedang login dengan status **200 OK**. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/me

![image](https://github.com/user-attachments/assets/3ce3d59f-9387-4a74-930f-829de75cf3e4)

- **PATCH Update User**: untuk memperbarui sebagian data API pengguna yang telah terdaftar dalam sistem dengan status **200 OK**. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/me

![image](https://github.com/user-attachments/assets/0a58d829-ab4e-4e8e-8610-8d5f99fddd17)

- **POST Log Out User**: untuk memverifikasi API kredensial pengguna sudah logout sistem dengan status **200 OK**. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/logout

![image](https://github.com/user-attachments/assets/fb44cd6e-c43a-4b21-91eb-3d156e8366cd)

- **DELETE Delete User**: untuk menghapus API kredensial pengguna dari sistem dengan status **200 OK**. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/me

![image](https://github.com/user-attachments/assets/6bfc3dcb-beae-4c9b-86a8-85a63c1984e4)

## Langkah-langkah Pengujian

### Persiapan Lingkungan
1. Unduh dan instal [Katalon Studio](https://www.katalon.com/download/).
2. Klon repositori ini:
   ```bash
   git clone https://github.com/Nikenarra0816/Katalon-taskapi-NikenArra.git
3. Running test pada **Test Suite**
