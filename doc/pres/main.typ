#import "@preview/touying:0.6.1": *
#import themes.university: *

#show: university-theme.with(
    aspect-ratio: "16-9",
    config-info(
        title: [Geometrica],
        subtitle: [
            Система построение геометрических чертежей со встроенным языком
            программирования и возможностью удаленного программного управления
        ],
        author: [Чубий Савва Андреевич],
    ),
)

#title-slide(
    title: ["Geometrica"],
    authors: grid(
        columns: (auto, 14cm),
        align: (right, left),
        gutter: 3mm,
        [Студент:], [Чубий Савва Андреевич],
        [], text(fill: gray)[БПИ 233],
        [Научный руководитель:], [Куренков Владимир Вячеславович],
        [],
        text(fill: gray)[
            старший преподаватель департамента больших
            данных и информационного поиска
        ],
    ),
)

#include "body.typ"
