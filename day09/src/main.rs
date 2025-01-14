#[derive(Debug, PartialEq)]
enum DiskEntry {
    File { id: usize, len: usize },
    Free(usize),
}
impl DiskEntry {
    fn parse(s: &str) -> Vec<DiskEntry> {
        let mut digits = s.trim().chars().map(|ch| (ch as usize - '0' as usize));

        let mut results = Vec::new();
        let mut id = 0;
        loop {
            let Some(file) = digits.next() else {
                break;
            };
            results.push(DiskEntry::File { id, len: file });
            id += 1;

            let Some(free) = digits.next() else {
                break;
            };
            results.push(DiskEntry::Free(free));
        }

        results
    }
}

// trait Defrag {
//     fn defrag(&mut self);
// }

// impl Defrag for Vec<DiskEntry> {
//     fn defrag(&mut self) {
// 	enum DiskEntryWithOffset {
// 	    File { offset:usize, id: usize, len: usize },
// 	    Free { offset:usize, len: usize },
// 	}
//     }    
// }



#[derive(Debug, PartialEq)]
struct DiskMap(Vec<Option<usize>>);
impl DiskMap {
    fn defrag(&mut self) {
        let n = self.0.len();
        let mut i: usize = 0;
        let mut j: isize = (n - 1) as isize;
        loop {
            // Scan i forward for next vacant spot
            while i < n {
                if self.0[i].is_none() {
                    break;
                }
                i += 1;
            }
            if i == n {
                break;
            }

            // Scan j backward for next occupied spot
            while j >= 0 {
                if self.0[j as usize].is_some() {
                    break;
                }
                j -= 1;
            }
            if j < 0 {
                break;
            }

            // If the pointers crossed, we're done.
            if i >= j as usize {
                break;
            }
            // Swap, and try again.
            self.0.swap(i, j as usize);
        }
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(position, entry)| match entry {
                Some(id) => position * id,
                None => 0,
            })
            .sum()
    }
}

impl FromIterator<DiskEntry> for DiskMap {
    fn from_iter<T>(entries: T) -> Self
	where T: IntoIterator<Item = DiskEntry>
    {
        let entries: Vec<_> = entries.into_iter().collect();
        let capacity: usize = entries
            .iter()
            .map(|entry| match entry {
                DiskEntry::File { len, .. } => len,
                DiskEntry::Free(len) => len,
            })
            .sum();
        let mut buffer: Vec<Option<usize>> = Vec::with_capacity(capacity);
        for entry in entries {
            match entry {
                DiskEntry::File { len, id } => {
                    for _ in 0..len {
                        buffer.push(Some(id));
                    }
                }
                DiskEntry::Free(len) => {
                    for _ in 0..len {
                        buffer.push(None);
                    }
                }
            }
        }
        Self(buffer)
    }
}

impl std::fmt::Display for DiskMap {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for entry in &self.0 {
            match entry {
                None => {
                    write!(formatter, ".")?;
                }
                Some(v) => {
                    write!(formatter, "{}", v)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const DATA: &str = "\
2333133121414131402";

    #[gtest]
    fn test_parse_entries() -> Result<()> {
        verify_that!(
            DiskEntry::parse("1234"),
            elements_are![
                eq(&DiskEntry::File { id: 0, len: 1 }),
                eq(&DiskEntry::Free(2)),
                eq(&DiskEntry::File { id: 1, len: 3 }),
                eq(&DiskEntry::Free(4))
            ]
        )
    }

    #[gtest]
    fn test_parse_diskmap() -> Result<()> {
        verify_that!(
            DiskEntry::parse("1234").into_iter().collect::<DiskMap>(),
            eq(&DiskMap(vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None
            ]))
        )
    }

    #[gtest]
    fn test_defrag() -> Result<()> {
        let mut diskmap: DiskMap = DiskEntry::parse(DATA).into_iter().collect();
        println!("{}", diskmap);
        diskmap.defrag();
        verify_that!(
            diskmap.to_string(),
            eq("0099811188827773336446555566..............")
        )
    }

    #[gtest]
    fn test_checksum() -> Result<()> {
        let mut diskmap: DiskMap = DiskEntry::parse(DATA).into_iter().collect();
        diskmap.defrag();
        verify_that!(diskmap.checksum(), eq(1928))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut diskmap: DiskMap = DiskEntry::parse(
        &std::io::read_to_string(std::io::stdin())?).into_iter().collect();
    diskmap.defrag();

    println!("Part 1: {}", diskmap.checksum());
    Ok(())
}
