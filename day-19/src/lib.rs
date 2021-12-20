use itertools::Itertools;
use na::Norm;
use nalgebra as na;
use once_cell::sync::Lazy;
use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let scans = parse_input(input);
    let (_scanners, beacons) = deduct_beacons(&scans);
    beacons.len()
}

fn deduct_beacons(
    scans: &[HashSet<na::Point3<i32>>],
) -> (HashSet<na::Point3<i32>>, HashSet<na::Point3<i32>>) {
    let mut scans: Vec<_> = scans.iter().collect();
    let mut transforms_to_0 = vec![na::one()];
    let mut beacons_0 = scans[0].clone();
    beacons_0.extend((1..scans.len()).flat_map(|mid| {
        let (a, b) = scans.split_at(mid);
        let (ia, ib, t) = a
            .iter()
            .enumerate()
            .rev()
            .cartesian_product(b.iter().enumerate())
            .find_map(|((ia, a), (ib, b))| good_enough_guess(a, b).map(|t| (ia, ib, t)))
            .unwrap();
        let t = transforms_to_0[ia] * t;
        transforms_to_0.push(t);
        scans.swap(mid, mid + ib);
        transform_beacons(scans[mid], &t).collect_vec().into_iter()
    }));
    (
        transforms_to_0
            .iter()
            .map(|t| {
                let h = t * na::Point3::origin().to_homogeneous();
                assert_eq!(h[3], 1);
                na::Point3::from(h.xyz())
            })
            .collect(),
        beacons_0,
    )
}

pub fn part_2(input: &str) -> f64 {
    let scans = parse_input(input);
    let (scanners, _beacons) = deduct_beacons(&scans);
    scanners
        .iter()
        .map(|b| b.cast::<f64>())
        .tuple_combinations()
        .map(|(a, b)| na::LpNorm(1).metric_distance(&a.coords, &b.coords))
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap()
}

fn parse_input(input: &str) -> Vec<HashSet<na::Point3<i32>>> {
    let mut i = 0;
    input
        .lines()
        .batching(|lines| {
            assert_eq!(lines.next()?, format!("--- scanner {} ---", i));
            i += 1;
            Some(
                lines
                    .take_while(|l| !l.is_empty())
                    .map(|l| {
                        let (x, y, z) = l
                            .split(',')
                            .map(|s| s.parse().unwrap())
                            .collect_tuple()
                            .unwrap();
                        na::Point3::from_slice(&[x, y, z])
                    })
                    .collect(),
            )
        })
        .collect()
}

/// Returns transformation matrix from 1 to 0 if there's one
fn good_enough_guess(
    scan_0: &HashSet<na::Point3<i32>>,
    scan_1: &HashSet<na::Point3<i32>>,
) -> Option<na::Matrix4<i32>> {
    guess_iter(scan_0, scan_1)
        .filter(|t| check_guess(scan_0, scan_1, &t).unwrap_or(0) >= 12)
        .next()
}

fn guess_iter<'a>(
    scan_0: &'a HashSet<na::Point3<i32>>,
    scan_1: &'a HashSet<na::Point3<i32>>,
) -> impl Iterator<Item = na::Matrix4<i32>> + 'a {
    scan_0
        .iter()
        .cartesian_product(scan_1)
        .cartesian_product(ROTATIONS.clone())
        .map(|((p0, p1), t)| {
            // let a and b be the same
            let mut t = t.to_homogeneous();
            t.prepend_translation_mut(&-p1.coords);
            t.append_translation_mut(&p0.coords);
            t
        })
}

fn check_guess(
    scan_0: &HashSet<na::Point3<i32>>,
    scan_1: &HashSet<na::Point3<i32>>,
    transform_1_to_0: &na::Matrix4<i32>,
) -> Option<usize> {
    transform_beacons(scan_1, transform_1_to_0)
        .filter(|v| v.iter().map(|c| c.abs()).max().unwrap() <= 1000)
        .try_fold(0, |acc, v| scan_0.contains(&v).then(|| acc + 1))
}

fn transform_beacons<'a>(
    scan: &'a HashSet<na::Point3<i32>>,
    transform: &'a na::Matrix4<i32>,
) -> impl Iterator<Item = na::Point3<i32>> + 'a {
    scan.iter().map(move |v1| {
        let v1 = v1.to_homogeneous();
        let v0 = transform * v1;
        assert_eq!(v0[3], 1);
        na::Point3::from(v0.xyz())
    })
}

const ROTATIONS: Lazy<[na::Matrix3<i32>; 24]> = Lazy::new(|| {
    let signs = {
        let units = [1, -1];
        itertools::cons_tuples(
            units
                .into_iter()
                .cartesian_product(units)
                .cartesian_product(units),
        )
    };
    let axes: [na::Vector3<i32>; 3] = [
        na::Vector3::x_axis().into_inner(),
        na::Vector3::y_axis().into_inner(),
        na::Vector3::z_axis().into_inner(),
    ];

    let mut iter = axes
        .into_iter()
        .permutations(3)
        .cartesian_product(signs)
        .map(|(axes, (sa, sb, sc))| {
            let (a, b, c) = axes.into_iter().collect_tuple().unwrap();
            na::Matrix3::from_columns(&[sa * a, sb * b, sc * c])
        })
        .filter(|m| m.cast::<f64>().determinant() == 1.0);

    let out = [(); 24].map(|()| iter.next().unwrap());
    assert_eq!(iter.next(), None);
    out
});
