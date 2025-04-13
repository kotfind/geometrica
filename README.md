# Geometrica
# Чубий Савва Андреевич, БПИ233

## Ветки

Новейшая версия документации находится в ветке `doc` (папке `doc`).

Новейшая версия кода находится в ветке `dev`.

## Запуск, компиляция и тестирование

Для выполнения каждого из действий есть два пути: вручную через `cargo` или
автоматически через `nix`.

Для запуска команд через `nix`, нужно установить `nix` и включить его
экспериментальные функции `nix-command` и `flakes`. Как это сделать зависит от
вашего дистрибутива. Один из способов описан здесь:
<https://github.com/mschwaig/howto-install-nix-with-flake-support>

Перед выполнением команд через `cargo` нужно установить все зависимости. Проще
всего это сделать через nix shell:
```bash
nix develop
```

Перед запуском клиентов и при запуске тестов через `cargo`, нужно установить
сервер или вручную добавить его в `$PATH`:
```bash
export PATH="$(nix eval .#server --raw --apply toString)/bin:$PATH"
```

Далее будем считать, что в переменной `$crateName` лежит имя крейта, который
вас интересует. Все пути указаны относительно корня репозитория.

### Сборка (без запуска)

#### Через cargo

```bash
nix develop
cargo build --release --package "$crateName"
```

Исполняемый файл --- `taret/release/$crateName`.

#### Через nix

```bash
nix build ".#${crateName}"
```

Исполняемый файл --- `result/bin/$crateName`.

### Запуск сервера (`$crateName = server`)

#### Через cargo

```bash
nix develop
cargo run --release --package server
```

#### Через nix

```bash
nix run .#server
```

### Запуск клиентов (`$crateName != server`)

#### Через cargo

```bash
nix develop
cargo build --release "--bin" server
export PATH="$(realpath target/release):$PATH"
cargo run --release --package "$crateName"
```

#### Через nix

```bash
export PATH="$(nix eval .#server --raw --apply toString)/bin:$PATH"
nix run ".#$crateName"
```

### Запуск всех тестов тестов

#### Через cargo

```bash
nix develop
cargo build --release --bin server
export PATH="$(realpath target/release):$PATH"
cargo test --release --all-features --all
```

#### Через nix

Путь к серверу "подтягивается" автоматически. Менять `$PATH` вручную не надо.

```bash
nix flake check
```

Для более подробного вывода:
```bash
nix flake check -L
```

### Сборка документации

#### Через cargo

```bash
nix develop
cargo doc --no-deps --package "$crateName"
```

#### Через nix

```bash
nix build ".#$crateName-doc"
```

Начальный файл документации --- `result/share/doc/$crateName/index.html`.`

### Открытие документации

#### Через cargo

```bash
nix develop
cargo doc --no-deps --package "$crateName" --open
```

#### Через nix

```bash
nix run ".#$crateName-doc"
```
