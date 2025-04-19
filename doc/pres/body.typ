#import "@preview/touying:0.6.1": *
#import themes.university: *

== Про терминологию Rust

#align(
    center + horizon,
    table(
        columns: 2,
        inset: 5mm,
        align: center + top,
        table.header[*Термин*][*Аналог*\ (примерный)],
        [Крейт], [Пакет],
        [Трейт], [Интерфейс],
    ),
)

== Краткое описание

- #pause Построение и изменение геометрических чертежей

- #pause Встроенный язык программирования (далее Язык)

- #pause #text(fill: gray)[Локальный] сервер + 3 клиента: `cli`, `gui` и `lib`

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

#grid(
    columns: (1fr, 1fr),
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
