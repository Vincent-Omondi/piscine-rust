use std::iter;

pub fn get_diamond(c: char) -> Vec<String> {
    let size = ((c as u8) - b'A') as usize * 2 + 1;

    let half: Vec<_> = iter::once(format!("{0:^1$}", 'A', size))
        .chain((1..=size / 2).map(|i| {
            format!(
                "{0:^1$}",
                format!("{0}{1}{0}", (b'A' + i as u8) as char, " ".repeat(i * 2 - 1)),
                size
            )
        }))
        .collect();

    half.iter()
        .chain(half.iter().rev().skip(1))
        .cloned()
        .collect()
}

