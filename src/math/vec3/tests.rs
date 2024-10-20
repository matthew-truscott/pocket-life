use super::Vec3;

macro_rules! test_assign {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (x, y, z) = $value;
                let my_vec = Vec3::new(x, y, z);
                assert_eq!(my_vec.x(), x);
                assert_eq!(my_vec.y(), y);
                assert_eq!(my_vec.z(), z);
            }
        )*
    }
}

test_assign! {
    assign_0: (1.0, 2.0, 3.0),
    assign_1: (0.0, 0.3, -0.3),
    assign_2: (12.34, 0.3, -134.2),
}

macro_rules! test_add {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (a, b, c) = $value;
                let added = a + b;
                assert_eq!(added, c);
            }
        )*
    }
}

test_add! {
    add_0: (Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 5.0), Vec3::new(3.0, 5.0, 8.0)),
    add_1: (Vec3::new(-1.0, 2.0, -3.0), Vec3::new(2.0, 3.0, 5.0), Vec3::new(1.0, 5.0, 2.0)),
    add_2: (Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, -3.0, -5.0), Vec3::new(3.0, -1.0, -2.0)),
}

macro_rules! test_sub {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (a, b, c) = $value;
                let subbed = a - b;
                assert_eq!(subbed, c);
            }
        )*
    }
}

test_sub! {
    sub_0: (Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 5.0), Vec3::new(-1.0, -1.0, -2.0)),
    sub_1: (Vec3::new(-1.0, 2.0, -3.0), Vec3::new(2.0, 3.0, 5.0), Vec3::new(-3.0, -1.0, -8.0)),
    sub_2: (Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, -3.0, -5.0), Vec3::new(-1.0, 5.0, 8.0)),
}
