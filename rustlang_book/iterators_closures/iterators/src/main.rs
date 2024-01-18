fn main() {
    let vector = vec![1, 2, 3];

    let vector_iterator = vector.iter();

    for value in vector_iterator {
        println!("Got: {}", value);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let vector = vec![1, 2, 3];

        let mut vector_iterator = vector.iter();

        assert_eq!(vector_iterator.next(), Some(&1));
        assert_eq!(vector_iterator.next(), Some(&2));
        assert_eq!(vector_iterator.next(), Some(&3));
        assert_eq!(vector_iterator.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let vector = vec![1, 2, 3];

        let vector_iterator = vector.iter();

        let total: i32 = vector_iterator.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_adaptor() {
        let vector = vec![1, 2, 3];
        let vector_2: Vec<_> = vector.iter().map(|item| item + 1).collect();

        assert_eq!(vector_2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let my_sizes = shoes_in_size(shoes, 10);

        assert_eq!(
            my_sizes,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
