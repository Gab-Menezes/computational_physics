#![feature(generic_arg_infer)]
use nalgebra::*;

fn main() {
    const N: usize = 8;
    const SUBSCRIPT: u32 = 0x2080;
    let slice = &[        
        0f32,-2.75f32,0f32,0f32,-2.25f32,0f32,0f32,0f32,
        -2.75f32,0f32,-2.25f32,0f32,0f32,0f32,0f32,0f32,
        0f32,-2.25f32,0f32,-2.25f32,0f32,-2.75f32,0f32,0f32,
        0f32,0f32,-2.25f32,0f32,-2.75f32,0f32,0f32,-2.25f32,
        -2.25f32,0f32,0f32,-2.75f32,0f32,0f32,0f32,0f32,
        0f32,0f32,-2.75f32,0f32,0f32,0f32,-2.25f32,0f32,
        0f32,0f32,0f32,0f32,0f32,-2.25f32,0f32,-2.75f32,
        0f32,0f32,0f32,-2.25f32,0f32,0f32,-2.75f32,0f32
    ];
    let bondings = &[
        (0usize, 1usize),
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 0),
        (2, 5),
        (5, 6),
        (6, 7),
        (7, 3),
    ];
    let secular_determinant = SMatrix::<f32, N, N>::from_row_slice(slice);
    let e = nalgebra_lapack::Eigen::new(secular_determinant, false, true).unwrap();
    let eval = &e.eigenvalues_re;
    let evec = e
        .eigenvectors
        .as_ref()
        .unwrap()
        .as_view::<Const<_>, Const<_>, Const<_>, Const<_>>();

    println!("Molecular Orbitals Energy Levels (Eigenvalues):");
    println!("{eval}");

    println!("");

    println!("Atomic Orbital Energy Levels (Eigenvectors):");
    println!("{evec}");

    println!("");

    println!("Molecular Orbital Functions:");
    let mut zipped = eval.iter().zip(evec.column_iter()).collect::<Vec<_>>();
    zipped.sort_unstable_by(|(v1, _), (v2, _)| v1.partial_cmp(v2).unwrap());

    for (i, (val, vec)) in zipped.iter().enumerate() {
        let i = char::from_u32(SUBSCRIPT + i as u32).unwrap();
        print!("({val:+.3}) ψ{i} = ");
        for (j, v) in vec.iter().enumerate() {
            let j = char::from_u32(SUBSCRIPT + j as u32).unwrap();
            print!("{v:+.9}φ{j}");
        }
        println!("");
    }

    println!("");

    println!("Total Electron Distribution:");
    let mut electron_distribution = SMatrix::<f32, N, 1>::zeros();
    for (_, vec) in zipped.iter().take(4) {
        let squared = SMatrix::<f32, N, 1>::from_iterator(vec.iter().map(|v| v*v));
        electron_distribution += 2f32*squared;
    }
    for (i, v) in electron_distribution.into_iter().enumerate() {
        println!("Atom {i}: {v:.4}");
    }

    println!("");

    println!("Boding Order:");
    for (i, j) in bondings {
        let mut v = 0f32;
        for (_, vec) in zipped.iter().take(4) {
            v += 2f32 * vec[*i] * vec[*j];
        }
        println!("Bonding ({i}, {j}): {v:.4}");
    }

}
