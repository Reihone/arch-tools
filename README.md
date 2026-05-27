# Arch Tools - CachyOS Auto Configuration

Автоматизированный инструмент для настройки CachyOS/Arch Linux систем.

## Возможности

- ✅ Автоматическая проверка и установка зависимостей
- ✅ Установка GPU драйверов (NVIDIA, Mesa, Intel)
- ✅ Выбор и установка Desktop Environment (KDE, GNOME, Hyprland, i3)
- ✅ Установка популярных приложений (Steam, Discord, Telegram)
- ✅ Установка браузеров (Firefox, Chrome, Zen, Brave)
- ✅ Установка пакетных менеджеров (yay, paru, aura, flatpak, snap)
- ✅ Полная автоматическая настройка системы
- ✅ Кастомная установка пакетов

## Требования

- CachyOS или другой Arch-based дистрибутив
- Rust 1.70+
- Интернет соединение
- Базовые знания Linux/Arch

## Установка

```bash
git clone https://github.com/Reihone/arch-tools
cd arch-tools
cargo build --release
```

## Использование

```bash
./target/release/arch-tools
```

Или после установки:
```bash
cargo install --path .
arch-tools
```

## Структура проекта

```
src/
├── main.rs                 # Точка входа
├── package_manager.rs      # Работа с pacman/yay/paru
├── dependency_checker.rs   # Проверка зависимостей
├── menu.rs                 # Главное меню
└── installers/
    ├── gpu.rs             # Установка GPU драйверов
    ├── desktop.rs         # Установка DE
    ├── apps.rs            # Установка приложений
    ├── browsers.rs        # Установка браузеров
    ├── package_managers.rs# Установка PM
    ├── full_setup.rs      # Полная настройка
    └── custom.rs          # Кастомная установка
```

## Дорожная карта

- [ ] GUI интерфейс (egui)
- [ ] Сохранение профилей конфигураций
- [ ] Восстановление системы из бэкапа
- [ ] Автоматический выбор GPU
- [ ] Интеграция с AUR helper'ами
- [ ] Конфигурация система настроек

## Лицензия

MIT

## Автор

Reihone
