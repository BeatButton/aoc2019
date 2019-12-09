const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const INPUT: &str = include_str!("input");

const NUM_LAYERS: usize = INPUT.len() / (WIDTH * HEIGHT);
const LAYER_LEN: usize = INPUT.len() / NUM_LAYERS;

fn main() {
    let chars: Vec<char> = INPUT.chars().collect();
    let layers = (0..NUM_LAYERS)
        .map(|i| {
            chars[(i * LAYER_LEN)..((i + 1) * LAYER_LEN)]
                .iter()
                .map(|&ch| ch as u8 - '0' as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut image: Vec<u8> = vec![2; LAYER_LEN];
    for layer in layers {
        for (idx, pixel) in layer.into_iter().enumerate() {
            if image[idx] == 2 {
                image[idx] = pixel;
            }
        }
    }
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!(
                "{}",
                match image[i * WIDTH + j] {
                    0 => " ",
                    1 => "X",
                    _ => "?",
                }
            )
        }
        println!("");
    }
}
