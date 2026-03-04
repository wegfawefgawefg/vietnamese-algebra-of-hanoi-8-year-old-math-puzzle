use std::convert::TryInto;

fn main() {
    let mut digits: Vec<i32> = (1..=9).collect();
    let mut solutions: Vec<[i32; 9]> = Vec::new();

    permutations(&mut digits, 0, &mut |perm| {
        let a = perm[0];
        let b = perm[1];
        let c = perm[2];
        let d = perm[3];
        let e = perm[4];
        let f = perm[5];
        let g = perm[6];
        let h = perm[7];
        let i = perm[8];

        let base = a + d + 12 * e - f - 11 + g * h + i - 10;
        let lhs = 13 * b;
        let rhs = (66 - base) * c;

        if lhs == rhs {
            let combo: [i32; 9] = perm.try_into().expect("slice with incorrect length");
            solutions.push(combo);
        }
    });

    for (idx, solution) in solutions.iter().enumerate() {
        println!(
            "Solution {}: a={} b={} c={} d={} e={} f={} g={} h={} i={}",
            idx + 1,
            solution[0],
            solution[1],
            solution[2],
            solution[3],
            solution[4],
            solution[5],
            solution[6],
            solution[7],
            solution[8]
        );
    }

    println!("Total solutions: {}", solutions.len());
}

fn permutations<F>(arr: &mut [i32], start: usize, callback: &mut F)
where
    F: FnMut(&[i32]),
{
    if start == arr.len() {
        callback(arr);
        return;
    }

    for idx in start..arr.len() {
        arr.swap(start, idx);
        permutations(arr, start + 1, callback);
        arr.swap(start, idx);
    }
}
