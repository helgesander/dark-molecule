# Dark Molecule | Framework for Penetration Testers

## Deploy

### via Docker (NOT READY YET)

### without Docker (hard way)

For backend:
Install diesel and run this commands:

```shell
cd backend 
diesel setup
cargo run --release
```

At the first launch, an administrator account will be generated and displayed in the logs of backend.

For frontend (in directory with frontend folder):
```shell
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk serve --release
```

## Configuration

For backend in backend/.env

## Configuration (for Docker Deploy way)


## TODO
- [ ] Modules with some useful tools (wrote on lua)
- [x] Create users
- [x] Create projects
- [x] Frontend for all
- [x] Add authentication (JWT maybe...)
- [ ] Change synchronous postgres client to asynchronous
- [x] Сделать валидацию введения данных пользователя
- [ ] Сделать так, чтобы при возвращении Bad Request возвращалась причина
- [x] Добавить функцию валидации, что юзернейм, почта уникальны
- [x] Добавить инициализацию бд в код бекенда
- [x] Модуль сканировать nmap для заполнения списка хостов (САМОЕ ГЛАВНОЕ)
- [x] Модуль сканирования nuclei, а также преобразования результатов в готовые issues (САМОЕ ГЛАВНОЕ)
- [ ] Модуль сканирования gowitness, если нас интересует только веб
- [ ] Модуль сканирования Shodan для нахождения каких-то точек входа еще
- [ ] Модуль фаззинга ffufai для сканирования поддиректорий в вебе (ОЧЕНЬ ОПЦИОНАЛЬНО)
- [ ] Модуль сканирования nessus (ОЧЕНЬ ОПЦИОНАЛЬНО)
