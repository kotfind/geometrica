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
        - #only("2-")[Через lib клиент]

        - #only("3-")[Через API]

        - #only("4-")[Через стандартный вывод и Язык]
    ],
    block(
        height: 90%,
        align(
            center + horizon,
            image("circles-geometrica.png", width: 100%),
        ),
    ),
)


#let analog(
    // Analog name
    name: none,
    /// Icon path
    icon: none,
    /// Developer
    dev: none,
    /// Site url
    url: none,
    /// +
    plus: none,
    /// -
    minus: none,
) = [
    #let sign(sign) = {
        align(center, text(weight: "bold", size: 50pt, sign))
    }

    #let lst(
        lst,
        offset: 0,
    ) = {
        list(for (i, item) in lst.enumerate(start: offset + 2) {
            uncover(str(i) + "-", item)
        })
    }

    == #name

    #grid(
        columns: (0mm, 40fr, 60fr),
        align: center + horizon,
        block(height: 90%),
        [
            #image(icon, height: 20%)

            #text(fill: blue, underline(link(url)))

            #text(fill: gray)[от] #dev
        ],
        grid(
            columns: (10mm, 1fr),
            align: left,
            gutter: 10mm,
            sign("+"), lst(plus),
            grid.cell(colspan: 2, line(length: 100%)),
            sign("-"), lst(minus, offset: plus.len())
        ),
    )
]

= Аналоги

#analog(
    icon: "geogebra-icon.svg",
    dev: "Markus Hohenwarter",
    url: "https://www.geogebra.org/geometry",
    plus: (
        [Бесплатно],
        [Есть оффлайн версия],
        [Есть библиотека для существ ЯП],
        [Есть стили],
    ),
    minus: (
        [Нет макросов],
        [Ограниченный ЯП],
        [Нельзя работать из терминала],
    ),
)

// #let cmp_products = (
//     [*Geometrica*#footnote[Данная система.]],
//     [GeoGebra#footnote(link("https://www.geogebra.org/geometry"))],
//     [Desmos Geometry#footnote(link("https://www.desmos.com/geometry"))],
//     [Живая Математика#footnote(link("https://www.int-edu.ru/content/rusticus-0"))],
//     [MathKit#footnote(link("https://obr.1c.ru/mathkit/"))],
// )
// #let cosmetic = [
//     Возможны косметические преобразования#footnote[
//         То есть можно менять цвета различных объектов, ширину прямых и т.д.
//     ]
// ]
//
// // @typstyle off
// #let cmp_body = (
//     //                                        *Geometrica*
//     //                                         |    GeoGebra
//     //                                         |    |    Desmos
//     //                                         |    |    |    Живая Математика
//     //                                         |    |    |    |    MathKit
//     //                                         |    |    |    |    |
//     [Программа бесплатна],                    "+", "+", "+", "-", "+",
//     [Есть оффлайн версия],                    "+", "+", "-", "+", "+",
//
//     [Возможно создание макросов],             "?", "-", "-", "+", "+",
//
//     [Есть встроенный ЯП],                     "+", "?", "?", "-", "-",
//     [Есть библиотека для существующего ЯП],   "+", "+", "+", "-", "-",
//     [Есть REST API],                          "+", "-", "-", "-", "-",
//     [Возможная работа из командной строки],   "+", "-", "-", "-", "-",
//
//     cosmetic,                                 "-", "+", "+", "+", "+",
// )

// TODO: Обязательно указывайте авторов (компании) аналогов, средств разработки и т.п.
