use core::f64;
use std::path::Path;

use client::{
    types::{
        core::{Circ, Ident, Line, Pt, ValueType},
        lang::{Definition, Expr, ValueDefinition},
    },
    Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cs = [
        Circ::new(Pt::new(200.0, 200.0), 100.0),
        Circ::new(Pt::new(350.0, 250.0), 150.0),
        Circ::new(Pt::new(170.0, 320.0), 120.0),
    ];

    let ls = get_lines(&cs);

    print(cs.iter(), ls.iter());
    show(cs.iter(), ls.iter(), Path::new("circles.svg")).await?;

    Ok(())
}

fn print<'a>(cs: impl Iterator<Item = &'a Circ>, ls: impl Iterator<Item = &'a Line>) {
    for c in cs {
        println!("{c}");
    }
    println!();

    for l in ls {
        println!("{l}");
    }
    println!();
}

async fn show(
    cs: impl Iterator<Item = &Circ>,
    ls: impl Iterator<Item = &Line>,
    svg_file: &Path,
) -> anyhow::Result<()> {
    let client = Client::new().await?;
    client.clear().await?;

    for (i, c) in cs.enumerate() {
        client
            .define_one(Definition::ValueDefinition(ValueDefinition {
                name: Ident(format!("c{i}")),
                value_type: Some(ValueType::Circ),
                body: Expr::Value((*c).into()),
            }))
            .await?;
    }

    for (i, l) in ls.enumerate() {
        client
            .define_one(Definition::ValueDefinition(ValueDefinition {
                name: Ident(format!("l{i}")),
                value_type: Some(ValueType::Line),
                body: Expr::Value((*l).into()),
            }))
            .await?;
    }

    client.save_svg(svg_file).await?;
    println!("Exported to {svg_file:?}");

    Ok(())
}

fn get_lines(cs: &[Circ]) -> Vec<Line> {
    let min_x = 0f64;
    let max_x = 500f64;
    let n = 100usize;
    let h = (max_x - min_x) / n as f64;

    let mut ls = vec![];
    for i in 0..n {
        let a = min_x + i as f64 * h;
        let b = a + h;
        let x = (b + a) / 2.0;

        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for c in cs {
            if let Some((y1, y2)) = inter(*c, x) {
                min_y = min_y.min(y1);
                max_y = max_y.max(y2);
            }
        }
        if min_y != f64::MAX && max_y != f64::MIN {
            ls.push(Line::new(Pt::new(x, min_y), Pt::new(x, max_y)));
        }
    }

    ls
}

fn inter(Circ { o, r }: Circ, x: f64) -> Option<(f64 /* y */, f64)> {
    let dx = (o.x - x).abs();
    if dx > r {
        None
    } else {
        let dy = (r * r - dx * dx).sqrt();
        Some((o.y - dy, o.y + dy))
    }
}
