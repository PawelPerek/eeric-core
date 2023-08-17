pub trait IterMaskExt: Iterator {
    fn masked_map<Mask, Destination, Func>(self, mask: Mask, dest: Destination, func: Func) -> 
        MaskedMap<Self, Mask, Destination, Func>
        where
            Self: Sized,
            Mask: Iterator,
            Destination: Iterator,
        {
            MaskedMap { values: self, mask, dest, func }
        }
}

struct MaskedMap<I, Mask, Destination, Func>
    where 
        Mask: Iterator,
        Destination: Iterator,
{
    values: I,
    mask: Mask,
    dest: Destination,
    func: Func,
}

impl <I, Mask, Destination, Func> Iterator for MaskedMap<I, Mask, Destination, Func>
    where
        I: Iterator,
        Mask: Iterator,
        Mask::Item: PartialEq<u64>,
        Destination: Iterator,
        Func: Fn(I::Item, Mask::Item) -> Destination::Item,
{
    type Item = Destination::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_item = self.values.next();
        let dest_item = self.dest.next();

        self.mask.next().map(|m| {
            if m == 1 {
                (&mut self.func)(iter_item.unwrap(), m)
            } else {
                dest_item.unwrap()
            }
        })
    }
}

impl <I: Iterator> IterMaskExt for I {}