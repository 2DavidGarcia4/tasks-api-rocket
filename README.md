# Rust Rocket webserver

This example is a [Rocket](https://rocket.rs) web server
Servidor web o api de tareas con autenticación y token de autorización tipo **JWT** con [Rocket](https://rocket.rs) y [Diesel](https://diesel.rs/) como ORM usando postgreSQL como base de datos relacional.

<!-- [![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/new/template/soL3yG) -->

## ✨ Características: 

- Rust
- Rocket
- Diesel
- Uuid

## 💁‍♀️ Cómo usar

- Crear el código sql de las tablas en el archivo up.sql
- Crear las tablas para la base de datos con `diesel migration run`
- Ejecutar el servidor con `cargo run`

## 📝 Notas

De forma predeterminada, el servidor Rocket se inicia en modo de ensayo. Puede comenzar en el modo de producción cambiando `ROCKET_ENV` en el `Dockerfile`.
