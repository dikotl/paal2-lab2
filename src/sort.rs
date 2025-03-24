use std::cmp::Ordering;

pub trait Sort: ExactSizeIterator {
    type SortedItem;

    fn swap(&mut self, a: usize, b: usize);

    fn cmp_by<F>(&self, a: usize, b: usize, cmp: &mut F) -> Ordering
    where
        F: FnMut(&Self::SortedItem, &Self::SortedItem) -> Ordering;

    fn bubble_sort<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&Self::SortedItem, &Self::SortedItem) -> Ordering,
    {
        let len = self.len();

        for i in 0..len {
            let mut swapped = false;

            for j in (i + 1)..len {
                if Sort::cmp_by(self, i, j, &mut cmp) == Ordering::Greater {
                    self.swap(i, j);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }
    }
}
