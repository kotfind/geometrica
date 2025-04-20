#import "@preview/touying:0.6.1": *
#import themes.university: *
#import "@preview/diagraph:0.3.3"

== Про терминологию Rust

#align(
    center + horizon,
    table(
        columns: 2,
        inset: 5mm,
        align: center + horizon,
        table.header(
            align(center + top)[*Термин*],
            align(center + top)[*Аналог*\ (примерный)],
        ),

        [Крейт], [Пакет],
        [Трейт], [Интерфейс],
        [`enum`],
        [
            `sealed class`

            #text(fill: gray)[`typesafe`] `union`

            `std::variant`
        ],
    ),
)

== Краткое описание

- #pause Построение и изменение геометрических чертежей

- #pause Встроенный язык программирования (далее Язык)

- #pause #text(fill: gray)[Локальный] сервер + 3 клиента: `cli`, `gui` и `lib`

== Применимость

- #pause Целевая аудитория:
    - Школьники/ школьные учителя (`gui`)
    - Студенты/ преподаватели ВУЗов (`cli`, `gui`, `lib`)

- #pause Примеры использования:
    - #pause Наглядная демонстрация теорем
        #text(fill: gray)[(см. "Описание языка", гл. 4)]
    - #pause Совместное решение задач в классе
    - #pause Проведение проверочных работ
    - #pause Выполнение домашних заданий
    - #pause Самостоятельное решение задач
    - #pause Отладка программ

= Практический пример
== Постановка задачи

#align(
    center + horizon,
    image("circles.svg", height: 90%),
)

== Решение с GeoGebra

#align(
    center + horizon,
    image("circles-geogebra.png", height: 90%),
)

== Решение с Geometrica

#slide(
    composer: (1fr, 1fr),
    [
        Варианты решения:
        - #only("2-")[Через lib-клиент]

        - #only("3-")[Через api]

        - #only("4-")[Через стандартный вывод, Язык и cli-клиент]
    ],
    block(
        height: 90%,
        align(
            center + horizon,
            image("circles-geometrica.png", width: 100%),
        ),
    ),
)

= Аналоги

#import "util.typ": analog

#analog(
    name: "GeoGebra",
    icon: "geogebra-icon.svg",
    dev: "Markus Hohenwarter",
    url: "https://www.geogebra.org/geometry",
    plus: (
        [Бесплатно],
        [Есть оффлайн версия],
        [Есть библиотека для сущ. ЯП],
        [Есть стили],
    ),
    minus: (
        [Нет макросов],
        [Ограниченный ЯП],
        [Нет REST API],
        [Нельзя работать из терминала],
    ),
)

#analog(
    name: "Desmos",
    icon: "desmos-icon.png",
    dev: "Desmos Studio PBC",
    url: "https://www.desmos.com/geometry",
    plus: (
        [Бесплатно],
        [Есть библиотека для сущ. ЯП],
        [Есть стили],
    ),
    minus: (
        [Нет оффлайн версии],
        [Нет макросов],
        [Ограниченный встроенный ЯП],
        [Нет REST API],
        [Нельзя работать из терминала],
    ),
)

#analog(
    name: "Живая Математика",
    icon: "Живая_Математика-logo.png",
    dev: [Учреждение ДПО "ИНТ"],
    url: "https://www.int-edu.ru/content/rusticus-0",
    plus: (
        [Есть оффлайн версия],
        [Есть макросы],
        [Есть стили],
    ),
    minus: (
        [Платно: домашная --- 2400 руб, базовая --- 6120 руб],
        [Нет встроенного ЯП],
        [Нет библиотеки для сущ. ЯП],
        [Нет REST API],
        [Нельзя работать из терминала],
    ),
)

#analog(
    name: "Математический Конструктор (MathKit)",
    icon: "MathKit-logo.png",
    dev: [ООО "Виртуальная лаборатория"],
    url: "https://obr.1c.ru/mathkit/",
    plus: (
        [Бесплатно],
        [Есть оффлайн версия],
        [Есть макросы],
        [Есть стили],
    ),
    minus: (
        [Нет встроенного ЯП],
        [Нет библиотеки для сущ. ЯП],
        [Нет REST API],
        [Нельзя работать из терминала],
    ),
)

= Функционал

==

- #pause Создать новый объект

    Типы: `bool`, `int`, `real`, `str`, `pt`, `line`, `circ`

    Виды: свободный/ зависимый

- #pause Изменить свободный объект и *пересчитать все зависимые*

- #pause Удалить объект и все зависимые

- #pause Получить значение объекта

- #pause Выполнить код на Языке

- #pause Экспортировать/ импортировать чертеж из файла

- #pause Экспортировать в `svg`

= Цель и задачи

==

*Цель:* разработать программный продукт "Geometrica"

#pause
*Задачи:*

- Определения функциональных требований
- Выбор стека технологий
- Написание "Технического Задания"
- Разработка архитектуры приложения
- Реализация программной системы "Geometrica"
- Тестирование программной системы "Geometrica"
- Написание итоговой документации
- Защита проекта

= Описание языка

== Общая структура

// definition
#let d(body) = text(fill: green, body)

// command
#let c(body) = text(fill: red, body)

// expr
#let e(body) = text(fill: blue, body)

#slide(
    composer: (1fr, 2fr),
    [
        - Типизация:
            - Сильная
            - Статическая
        - Конструкции:
            - Императивные:
                - #d[Объявления]
                - #c[Команды]
            - Функциональные #e[выражения]
    ],
    [
        #set text(font: "DejaVu Sans Mono", size: 20pt)

        #d[fact n:int -> int =] #e[if]

        #e("    " + [n > 0 then n \* (fact (n - 1)),])

        #e("    " + [n == 0 then 1])

        #d[n = ]#e[5]

        #d[t = ]#e[fact n]

        #c[set! n] #e[(1 + 1)]

        #c[get_all!]
    ],
)

== Конструкции

#slide[
    === Скрипт
    - #pause Выражение (Statement)
        - #pause Вызов команды
        - #pause Объявление
            - #pause #text(fill: gray)[Объявление] функции
            - #pause #text(fill: gray)[Объявление] значения
]

#slide[
    #show regex("\w*!"): set text(weight: "bold")

    === Команды

    #grid(
        columns: (1fr, 1fr),
        gutter: 10mm,
        [
            - #pause Изменения:
                ```
                clear!
                rm! x y z
                set! x (10 * 2 + 1)
                ```
        ],
        [
            - #pause Служебные:
                ```
                list_cmd!
                list_func!
                ```
        ],

        [
            - #pause Работа с файлами:
                ```
                save! "file.geom"
                load! "file.geom"
                save_svg! "img.svg"
                ```
        ],
        [
            - #pause Вычисления:
                ```
                eval! (x + 1)
                get! x y z
                get_all!
                ```
        ],
    )
]

#slide[
    === Объявления значений

    #grid(
        columns: (1fr, 1fr),
        gutter: 10mm,
        [
            - #pause Независмые
            ```
            x:real = 42.0
            y = "Hello,\nworld!"
            p = 2.0 * (pt 10.0 20.0)

            // ошибка: real не int
            t:int = 10.0

            // none
            x = none line
            ```
        ],
        [
            - #pause Зависимые
            ```
            k:real = 2.0 * x
            l = (x + y) / 2.0

            // ошибка: m опр. через m
            m = 2 * m
            ```
        ],
    )
]

#slide[
    === Объявления функций

    ```
    sum x:int y:int -> int = x + y

    // перегрузка
    sum x:real y:real -> real = x + y

    // рекурсия
    fact n:int -> int = if
        n > 0 then n * (fact (n - 1))
        n == 0 then 1

    // ошибка: x - НЕ аргумент ф-ции
    add_x t:int -> int = t + x
    ```
]

#slide[
    === Выражения (Expr)

    - #pause Литерал
    - #pause Переменная
    - #pause Приведение типов `as`
    - #pause Условное выражение `if`
    - #pause Вызов функции
    - #pause `Dot`-нотация
    - #pause Применение бинарного оператора
    - #pause Применение унарного оператора
    - #pause Выражение `let`
]

#slide[
    === Выражения. Приведение типов `as`

    #align(
        center + horizon,
        ```
        x = 10
        y = x as real // y = 10.0
        ```,
    )
]

#slide[
    === Выражения. Условное выражение `if`

    #align(
        center + horizon,
        ```
        cmp x:int y:int -> str = if
            x > y  then "x is greater",
            x < y  then "y is greater",
            x == y then "x and y are the same",
            else        "just how?" // else можно не писать
        ```,
    )
]

#slide[
    === Выражения. Вызов функции

    #align(
        center + horizon,
        ```
        p1 = pt 100.0 100.0
        l = line p1 (pt 200.0 200.0)
        l_p2_y = y (p2 l)
        ```,
    )
]

#slide[
    === Выражения. `Dot`-нотация

    #align(
        center + horizon,
        ```
        l_p2_x = l.p2.x
        ```,
    )
]

#slide[
    === Выражения. Унарный оператор

    #align(
        center + horizon,
        ```
        y = -x
        cond2 = !cond1
        ```,
    )
]

#slide[
    === Выражения. Бинарный оператор
    хорошо
    #align(
        center + horizon,
        ```
        mid = (p1 + p2) / 2
        ```,
    )
]

#slide[
    === Выражения. Выражение `let`

    #align(
        center + horizon,
        ```
        dist p1:pt p2:pt -> real = let
            delta = p1 - p2,
            x = delta.x,
            y = delta.y,
        in
            (x^2.0 + y^2.0)^0.5
        ```,
    )
]

= Реализация

== Взаимодействие между крейтами

#slide(
    composer: (1fr, 1fr),
    [
        #align(
            center + horizon,
            diagraph.render(
                read("./crate_relations.dot"),
                width: 100%,
            ),
        )

        #set par(justify: true)
        #set text(fill: gray, size: 10pt)
        Бинарные крейты (bin) выделены жирным, крейты-библиотеки
        (lib) выделены курсивом. Обычными стрелками показаны
        библиотечные зависимости, пунктирными --- зависимости других типов.
    ],
    [
        - *Types* --- общие объявления

        - *Parser* --- парсер Языка

        - *Executor* --- основные вычисления

        - *Server* --- сервер

        - *Client* --- клиент--библиотека

        - *GUI* --- графический клиент

        - *CLI* --- клиент командной строки
    ],
)

== Крейт Types

- #pause Содержит:
    - #pause Типы Языка (`Value`, `FunctionSignature`, ...)
    - #pause Коснтрукции Языка (`Expr`, ...)
    - #pause api (`api::json::dump::{Request, Respone, ROUTE}`, ...)
- Легкий, использует условную компиляцию

#slide(
    repeat: 3,
    self => [
        #let (uncover, only, alternatives) = utils.methods(self)

        === Про api

        #grid(
            columns: (auto, 1fr),
            gutter: 10mm,
            [
                #list(
                    only("2-")[
                        Только `POST`
                    ],
                    only("3-")[
                        Есть обработка ошибок
                    ],
                    only("4-")[
                        Пример объявления:
                    ],
                )
            ],
            align(center + horizon)[
                #set text(size: 20pt)

                #only(
                    "4-",
                    ```rust
                    pub mod items {
                        pub mod get {
                            route! {
                                ROUTE "/items/get"
                                REQUEST {
                                    name: Ident
                                }
                                RESPONSE {
                                    value: Value
                                }
                            }
                        }
                    }
                    ```,
                )
            ],
        )
    ],
)

== Крейт Executor

- #pause Поддерживает состояние чертежа через `Node`
- #pause Компилирует `Expr` в `CExpr`
- #pause Выполняет все вычисления

== Крейт Server

- #pause Фасад над *Executor*
- #pause Реализует api из *Types*

== Крейт Client

- #pause Используется в *GUI* и *CLI*

- #pause Главная структура --- `Client`

- #pause При создании `Client` может запустить сервер

- #pause Всё делается через методы `Client`:
    - `Client::eval`
    - `Client::get_all_items`
    - `Cilent::command`
    - ...

- #pause Некоторые методы просто посылают запрос, другие имеют более сложную
    логику

#slide[
    === Процесс исполнение кода на Языке

    #align(
        center + horizon,
        diagraph.render(
            read("./execution_process.dot"),
            width: 100%,
        ),
    )

    #set par(justify: true)
    #set text(fill: gray, size: 20pt)
    В кругах обозначены состояния, в прямоугольниках --- действия, в ромбах ---
    условия. Действия в желтом прямоугольнике происходят на стороне сервера,
    остальные --- на стороне клиента.
]

== Крейт CLI

#grid(
    columns: (auto, 1fr),
    gutter: 10mm,
    [
        Режимы работы:
        - #only("2-")[Скриптовый]

            #only("2")[
                Запуск: ```bash cli script.geom```
            ]

        - #only("3-")[Стандартного ввода]

            #only("3")[
                Запуск: ```bash cat script.geom | cli```
            ]

        - #only("4-")[Интерактивный]

            #only("4")[
                Запуск: ```bash cli```

                Пример сессии:
            ]
    ],
    only("4")[
        #set text(size: 17pt)
        #set align(right + horizon)

        ```
        Welcome to Geometrica Cli!
        Enter list_cmd! to see all available commands.

        > x = 1.0
        > y = 2.0
        > z = (x + y) / 2.0
        > get! z
        ╭──────┬───────╮
        │ Name │ Value │
        ├──────┼───────┤
        │ z    │ 1.500 │
        ╰──────┴───────╯

        > set! x 4.0
        > get! z
        ╭──────┬───────╮
        │ Name │ Value │
        ├──────┼───────┤
        │ z    │ 3.000 │
        ╰──────┴───────╯
        ```
    ],
)

== Статистика

#align(
    center + horizon,
    table(
        columns: 2,
        align: (left, right),
        inset: 5mm,

        // cloc --vcs=git .
        [Строк кода:], `~6500`,

        // ls crates/
        [Крейтов:], `7`,

        // rg -w mod | rg -v test | wc -l
        [Модулей:], `~60`,

        // rg -w 'struct|enum' | wc -l
        [Структур и перечислений:], `~130`,

        // rg -w 'fn' | wc -l
        [Функций и методов:], `~500`,

        // cargo tarpaulin \
        //     --skip-clean \
        //     --workspace \
        //     --exclude-files 'crates/cli/*' \
        //     --exclude-files 'crates/gui/*' \
        //     --exclude-files 'crates/server/*' \
        //     --exclude-files 'crates/types/*' \
        //     --out html
        [Покрытие тестами\ (`parser`, `executor`, `client`):], `~60%`
    ),
)

== Стек технологий

#grid(
    columns: (1fr, 1fr),
    gutter: 10mm,
    [
        - #pause *Общие*:
            - #pause rust
            - #pause cargo
            - #pause nix

        - #pause *Документация:*
            - #pause Typst
            - #pause GraphViz

        - #pause *Types:*
            - #pause serde
    ],
    [
        - #pause *Parser:*
            - #pause peg

        - #pause *Server:*
            - #pause tokio
            - #pause axum

        - #pause *Client:*
            - #pause tokio
            - #pause reqwest

        - #pause *GUI:*
            - #pause iced
    ],
)

= Итог

== Выводы

- Цель достигнута
- Все поставленные задачи выполнены

== Сравнение с аналогами

#let cmp_products = (
    [*Geometrica*],
    [GeoGebra],
    [Desmos],
    [Жив. Мат.],
    [MathKit],
)

// dot
#let dot(color) = table.cell(
    align: center + horizon,
    stroke: none,
    circle(fill: color, radius: 0.2em),
)
// good
#let g = dot(green)
// middle
#let m = dot(yellow)
// bad
#let b = dot(red)

// @typstyle off
#let cmp_body = (
    //                                         *Geometrica*
    //                                          |    GeoGebra
    //                                          |    |    Desmos
    //                                          |    |    |    Живая Математика
    //                                          |    |    |    |    MathKit
    //                                          |    |    |    |    |
    [Бесплатно],              "+", "+", "+", "-", "+", m,
    [Оффлайн версия],         "+", "+", "-", "+", "+", m,

    [Макросы],                "?", "-", "-", "+", "+", m,

    [Библиотека для сущ. ЯП], "+", "+", "+", "-", "-", m,
    [Встроенный ЯП],          "+", "?", "?", "-", "-", g,
    [REST API],               "+", "-", "-", "-", "-", g,
    [Работа из терминала],    "+", "-", "-", "-", "-", g,

    [Стили],                  "-", "+", "+", "+", "+", b,
)

#grid(
    columns: (auto, 1fr),
    align: center + horizon,
    gutter: 10mm,
    [
        #table(
            columns: cmp_products.len() + 2,
            align: center + horizon,
            inset: 2mm,

            table.header(
                table.cell(stroke: none)[],
                ..cmp_products.map(col => rotate(90deg, reflow: true, col)),
                table.cell(stroke: none)[],
            ),

            ..cmp_body
        )
    ],
    [
        #set align(left)
        #set par(justify: true)
        #set text(fill: gray, size: 17pt)
        Полные названия аналогов приведены в секции "Аналоги". "+" --- функция
        имеется, "-" --- функция отсутствует, "?" --- функция частично
        присутствует/ имеются значительные ограничения. Зеленый --- Geometrica
        превосходит большинство аналогов, желтый --- Geometrica первосходит
        многие аналоги, красный --- Geometrica проигрывает аналогам.
    ],
)

