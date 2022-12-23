#![allow(dead_code)]

#[derive(PartialEq, Eq, Debug)]
pub struct Move(String, String);

fn h(ori: &str, dst: &str, aux: &str, n: u32, on_move: &mut impl FnMut(&str, &str)) {
    if n == 1 {
        on_move(ori, dst);
        return;
    }

    h(ori, aux, dst, n - 1, on_move);
    on_move(ori, dst);
    h(aux, dst, ori, n - 1, on_move);
}

#[cfg(test)]
mod test_recursive {
    use super::{h, Move};

    fn hanoi(ori: &str, dst: &str, aux: &str, n: u32) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut f = |ori: &str, dst: &str| moves.push(Move(ori.to_string(), dst.to_string()));

        h(ori, dst, aux, n, &mut f);
        moves
    }

    #[test]
    fn hanoi_1() {
        let moves = hanoi("A", "C", "B", 1);
        assert_eq!(moves, [Move("A".to_string(), "C".to_string())]);
    }

    #[test]
    fn hanoi_2() {
        let moves = hanoi("A", "C", "B", 2);
        let result = [
            Move("A".to_string(), "B".to_string()),
            Move("A".to_string(), "C".to_string()),
            Move("B".to_string(), "C".to_string()),
        ];
        assert_eq!(moves, result);
    }

    #[test]
    fn hanoi_3() {
        let moves = hanoi("A", "C", "B", 3);
        let result = [
            Move("A".to_string(), "C".to_string()),
            Move("A".to_string(), "B".to_string()),
            Move("C".to_string(), "B".to_string()),
            Move("A".to_string(), "C".to_string()),
            Move("B".to_string(), "A".to_string()),
            Move("B".to_string(), "C".to_string()),
            Move("A".to_string(), "C".to_string()),
        ];
        assert_eq!(moves, result);
    }
}
