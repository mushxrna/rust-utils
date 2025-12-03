use std::ops::RangeInclusive;

pub struct Splitter<A> {
    indicator: A,
}

impl<A> Splitter<A>
where
    A: PartialEq,
{
    pub fn new(indicator: A) -> Splitter<A> {
        Splitter {
            indicator: indicator,
        }
    }

    pub fn split_into_slices<'a, CompA: PartialEq<A>>(
        &self,
        source: &'a [CompA],
    ) -> Vec<&'a [CompA]> {
        let mut index_buffer = vec![];
        let mut result = vec![];

        for (i, x) in source.iter().enumerate() {
            if x != &self.indicator {
                index_buffer.push(i);
            } else {
                let (begin, end) = (index_buffer[0], *index_buffer.last().unwrap());
                result.push(&source[begin..=end]);
                index_buffer.clear();
            }
        }

        let (last_begin, last_end) = (index_buffer[0], *index_buffer.last().unwrap());
        result.push(&source[last_begin..=last_end]);

        result
    }
    /*
    pub fn split_on_and<'a, F, R>(&self, source: &'a [T], f: F) -> Vec<R>
    where
        F: Fn(&'a [T]) -> R,
        T: 'a,
    {
        let x = self.split(source);
        x.into_iter().map(|e| -> R { f(e) }).collect()
    }
    */
    pub fn split_into_ranges<CompA: PartialEq<A>>(
        &self,
        source: &[A],
    ) -> Vec<RangeInclusive<usize>> {
        let mut index_buffer = vec![];
        let mut result = vec![];

        for (i, x) in source.iter().enumerate() {
            if x != &self.indicator {
                index_buffer.push(i);
            } else {
                let (begin, end) = (index_buffer[0], *index_buffer.last().unwrap());
                result.push(begin..=end);
                index_buffer.clear();
            }
        }

        let (last_begin, last_end) = (index_buffer[0], *index_buffer.last().unwrap());
        result.push(last_begin..=last_end);

        result
    }
}
