use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

/// 尝试测定 c 是否位于曼德博集中, 使用最多 limit 次迭代来判定
///
/// 如果 c 不是集合成员之一, 则返回 Some(i), 其中 i 是 c 离开以原点为中心的半径为 2 的圆时需要的迭代次数
/// 如果 c 可能是集合成员之一(即达到了迭代次数限制后仍然无法证明不是成员), 则返回 None
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// 把字符串 s (形如 "400x600" 或 "1.0,0.5") 解析成一个坐标对
///
/// 字符串具有 <left><sep><right> 的格式, <left> 和 <right> 是可以被 T::From_str 解析的字符串
/// 如果 s 具有正确的格式就返回 Some<(x,y)>, 否则返回 None
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    // Option<T, T> 或者是 None, 或者是 Some((v1, v2))
    match s.find(separator) {
        None => None,
        Some(index) => {
            // &s[..index] 和 &s[index+1..] 是字符串切片, 类型 T 的关联函数会 from_str 会将他们解析成类型 T 的值
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// 给定输出图像中像素的行和列, 返回复平面中对应的坐标
///
/// bounds 定义了图像的像素宽度和像素高度
/// pixel 表示图像中特定像素的 (column, row) 二元组
/// upper_left 和 lower_right 定义了复平面中表示指定图像覆盖范围的点
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        // 这里用减法是因为在屏幕坐标系中 pixel.1 是向下递增的, 但复数的虚部是向上递增的
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

/// 将曼德博集对应的矩形渲染到像素缓冲区中
///
/// bounds 参数会给出缓冲区 pixels 的宽度和高度, 此缓冲区中每个字节都包含一个像素的灰度值
/// upper_left 和 lower_right 分别指定了复平面中的左上角和右下角的坐标
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    // 遍历所有的像素点
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result <(), std::io::Error>{
 let output = File::create(filename)?;
 let encoder = PNGEncoder::new(output);
 encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
 Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!( "Example: {} mandel.png 4000x3000 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    // 单线程
    // render(&mut pixels, bounds, upper_left, lower_right);

    // 并发
    // 使用 14 个线程, 并计算每个条带 (band) 分配到的行数
    let threads = 14;
    let rows_per_band = bounds.1 / threads + 1;
    {
        // 将整个像素缓冲区 pixels 划分成多个条带 bands, 相当于在做并行任务切片
        // rows_per_band 包含整行的像素, chunks_mut 生成的最后一个切片包含的行数可能少一些
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        // 使用 crossbeam::scope 创建一个线程池, 并在每个线程中渲染一个条带
        // |spawner| {...} 是 Rust 闭包, 它需要一个参数 spawner, scope 会等待所有线程运行完后再返回
        // 一切顺利的话 scope 会返回 OK(()), 如果我们启动的线程发送 panic, 它会返回一个 Err, 我们 unwrap 后也会 panic
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                // 创建一个线程运行 move |_| {...} 闭包, 闭包中的代码会在新线程中运行
                // move 表示这个闭包会接手所用变量的所有权
                // 参数列表 |_| 意味着闭包会接收一个参数, 但是不会使用它
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
