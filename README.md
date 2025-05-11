# Dark Molecule | Framework for Penetration Testers

## Deploy

### via Docker

```shell
docker compose up --build
```

### without Docker (hard way)

For backend:
Install diesel and run this commands:

```shell
cd backend 
diesel setup
cargo run --release
```

At the first launch, an administrator account will be generated and displayed in the logs of backend.

For frontend:
TODO

## Configuration (for Docker Deploy way)

All settings presented in .env.example:

```dotenv
POSTGRES_USER: postgres
POSTGRES_PASSWORD: postgres
POSTGRES_DB: dark-molecule-db
POSTGRES_PORT: 5432
BACKEND_PORT: 8080
FRONTEND_PORT: 80
TEMPLATES_PATH: ./templates

```


## TODO
- [ ] Modules with some usefull tools (wrote on lua)
- [x] Create users
- [x] Create projects
- [x] Frontend for all
- [x] Add authentication (JWT maybe...)
- [ ] Change synchronous postgres client to asynchronous
- [x] Сделать валидацию введения данных пользователя
- [ ] Сделать так, чтобы при возвращении Bad Request возвращалась причина
- [x] Добавить функцию валидации, что юзернейм, почта уникальны
- [x] Добавить инициализацию бд в код бекенда
- [ ] Модуль сканировать nmap для заполнения списка хостов (САМОЕ ГЛАВНОЕ)
- [ ] Модуль сканирования nuclei, а также преобразования результатов в готовые issues (САМОЕ ГЛАВНОЕ)
- [ ] Модуль сканирования gowitness, если нас интересует только веб
- [ ] Модуль сканирования Shodan для нахождения каких-то точек входа еще
- [ ] Модуль фаззинга ffufai для сканирования поддиректорий в вебе (ОЧЕНЬ ОПЦИОНАЛЬНО)
- [ ] Модуль сканирования nessus (ОЧЕНЬ ОПЦИОНАЛЬНО)
