use std::collections::HashSet;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
}

pub fn area_occupied(data: &Vec<Rectangle>) -> i32 {
    let mut occupied = HashSet::new();

    for rect in data {
        let x_min = rect.a.x.min(rect.b.x);
        let x_max = rect.a.x.max(rect.b.x);
        let y_min = rect.a.y.min(rect.b.y);
        let y_max = rect.a.y.max(rect.b.y);

        for x in x_min..x_max {
            for y in y_min..y_max {
                occupied.insert((x, y));
            }
        }
    }

    occupied.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Rectangle> {
        vec![
            Rectangle {
                a: Point { x: 2, y: 9 },
                b: Point { x: 5, y: 3 },
            },
            Rectangle {
                a: Point { x: 3, y: 8 },
                b: Point { x: 10, y: 6 },
            },
            Rectangle {
                a: Point { x: 9, y: 10 },
                b: Point { x: 13, y: 6 },
            },
        ]
    }

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}