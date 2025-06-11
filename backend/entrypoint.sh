#!/bin/sh

# Применяем миграции
diesel setup || exit 1

# Запускаем сервер на переднем плане (foreground)
exec ./backend