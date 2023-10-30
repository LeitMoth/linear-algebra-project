use glam::Vec3;

macro_rules! tri {
    (
        ($a:expr, $b:expr, $c:expr)$(,)?
        ($d:expr, $e:expr, $f:expr)$(,)?
        ($g:expr, $h:expr, $i:expr)$(,)?
    ) => {
        [
            Vec3::new($a, $b, $c),
            Vec3::new($d, $e, $f),
            Vec3::new($g, $h, $i),
        ]
    };
}

macro_rules! quad {
    (
        ($a:expr, $b:expr, $c:expr)$(,)?
        ($d:expr, $e:expr, $f:expr)$(,)?
        ($g:expr, $h:expr, $i:expr)$(,)?
        ($j:expr, $k:expr, $l:expr)$(,)?
    ) => {
        [
            tri![($a, $b, $c)($d, $e, $f)($g, $h, $i)],
            tri![($d, $e, $f)($g, $h, $i)($j, $k, $l)],
        ]
    };
}

pub(crate) use {quad, tri};

pub fn cube() -> impl Iterator<Item = [Vec3; 3]> {
    [
        //from y
        quad![
            (1.0, -1.0, 1.0),
            (-1.0, -1.0, 1.0),
            (1.0, -1.0, -1.0),
            (-1.0, -1.0, -1.0),
        ],
        quad![
            (1.0, 1.0, 1.0),
            (-1.0, 1.0, 1.0),
            (1.0, 1.0, -1.0),
            (-1.0, 1.0, -1.0),
        ],
        //from x
        quad![
            (1.0, 1.0, 1.0),
            (1.0, -1.0, 1.0),
            (1.0, 1.0, -1.0),
            (1.0, -1.0, -1.0),
        ],
        quad![
            (-1.0, 1.0, 1.0),
            (-1.0, -1.0, 1.0),
            (-1.0, 1.0, -1.0),
            (-1.0, -1.0, -1.0),
        ],
        //from z
        quad![
            (1.0, 1.0, 1.0),
            (-1.0, 1.0, 1.0),
            (1.0, -1.0, 1.0),
            (-1.0, -1.0, 1.0),
        ],
        quad![
            (1.0, 1.0, -1.0),
            (-1.0, 1.0, -1.0),
            (1.0, -1.0, -1.0),
            (-1.0, -1.0, -1.0),
        ],
    ]
    .into_iter()
    .flatten()
}
