use std::fmt::Write;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

fn cnt(layer: &[u8], b: u8) -> usize {
    layer.iter().filter(|&c| *c == b).count()
}

fn part1(inp: &str) -> usize {
    let mut minzeroes = HEIGHT * WIDTH;
    let mut min_layer: Option<&[u8]> = None;
    for layer in inp.as_bytes().chunks(HEIGHT * WIDTH) {
        let zeroes = cnt(layer, b'0');
        if zeroes < minzeroes {
            minzeroes = zeroes;
            min_layer = Some(layer);
        }
    }
    let min_layer = min_layer.unwrap();
    cnt(min_layer, b'1') * cnt(min_layer, b'2')
}

fn part2(inp: &str) -> String {
    let mut ret = String::new();
    let layers = inp
        .as_bytes()
        .chunks(HEIGHT * WIDTH)
        .collect::<Vec<&[u8]>>();
    for row in 0..HEIGHT {
        for column in 0..WIDTH {
            let mut pixel = None;
            for layer in &layers {
                let idx = column + WIDTH * row;
                if layer[idx] != b'2' {
                    pixel = layer.get(idx);
                    break;
                }
            }
            write!(
                &mut ret,
                "{}",
                match pixel.unwrap() {
                    b'0' => " ",
                    b'1' => "1",
                    _ => panic!("bruh"),
                }
            )
            .unwrap();
        }
        writeln!(&mut ret).unwrap();
    }
    ret
}

xaoc::xaoc!();
