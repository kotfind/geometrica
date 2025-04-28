#import "@preview/touying:0.6.1": *
#import themes.university: *

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
    #let repeat_num = 3

    #let slide_inner(self) = [
        #let (uncover, only, alternatives) = utils.methods(self)

        #let sign(sign) = {
            align(center, text(weight: "bold", size: 50pt, sign))
        }

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
                sign("+"), uncover("2-", list(..plus)),
                grid.cell(colspan: 2, line(length: 100%)),
                sign("-"), uncover("3-", list(..minus))
            ),
        )
    ]

    == #name

    #slide(repeat: repeat_num, slide_inner)
]
