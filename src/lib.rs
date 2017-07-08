use std::collections::HashMap;
use std::hash::Hash;


trait Counter<T, I> {
    fn inc(&mut self, target: T) -> I;
    fn dec(&mut self, target: T) -> I;
    fn count_of(&self, target: T) -> I;
}

impl<T: Hash + Eq + std::fmt::Debug> Counter<T, i64> for HashMap<T, i64> {

    fn inc(&mut self, target: T) -> i64 {
        let count = self.entry(target).or_insert(0);
        *count += 1;

        *count
    }

    fn dec(&mut self, target: T) -> i64 {
        let count = self.entry(target).or_insert(0);
        *count -= 1;

        *count
    }

    fn count_of(&self, target: T) -> i64 {
        match self.get(&target) {
            Some(&val) => val,
            None => 0,
        }
    }

}


#[cfg(test)]
mod tests {

    use Counter;
    use std::collections::HashMap;

    #[test]
    fn inc_dec() {
        let s1 = "Test string 1";
        let s2 = "Test string 2";

        let runs = 10000;

        let mut counter: HashMap<&str, i64> = HashMap::new();

        assert_eq!(counter.count_of("foo"), 0, "Defaults to 0");
        assert_eq!(counter.count_of(&s1), 0, "Defaults to 0");
        assert_eq!(counter.count_of(&s2), 0, "Defaults to 0");

        // Try inc then dec
        for i in 0..runs {
            assert_eq!(counter.inc(&s1), i + 1);
        }

        assert_eq!(counter.count_of(&s1), runs);

        for i in 0..2*runs {
            assert_eq!(counter.dec(&s1), runs - 1 - i);
        }

        assert_eq!(counter.count_of(&s1), -runs);

        // Try dec then inc
        for i in 0..runs{
            assert_eq!(counter.dec(&s2), -i - 1);
        }

        assert_eq!(counter.count_of(&s2), -runs);

        for i in 0..2*runs {
            assert_eq!(counter.inc(&s2), -runs + 1 + i);
        }

        assert_eq!(counter.count_of(&s2), runs);

        assert_eq!(counter.count_of("foo"), 0, "Defaults to 0")
    }
}

