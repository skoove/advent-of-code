#[derive(Debug)]
pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    pub fn side_areas(&self) -> [u32; 3] {
        [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
    }

    pub fn surface_area(&self) -> u32 {
        self.side_areas().iter().sum::<u32>() * 2
    }

    pub fn smallest_side(&self) -> u32 {
        let sides = self.side_areas();
        match sides.iter().min() {
            Some(area) => *area,
            None => *sides.last().unwrap(),
        }
    }

    pub fn paper_needed(&self) -> u32 {
        self.surface_area() + self.smallest_side()
    }

    /// parse from format wxhxl
    pub fn from_str(str: &str) -> Self {
        let mut split = str.split("x");

        Self {
            length: split.next().unwrap().parse().unwrap(),
            width: split.next().unwrap().parse().unwrap(),
            height: split.next().unwrap().parse().unwrap(),
        }
    }

    pub fn perimeters(&self) -> [u32; 3] {
        [
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ]
    }

    pub fn smallest_perimiter(&self) -> u32 {
        *self.perimeters().iter().min().unwrap()
    }

    pub fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    pub fn ribbon_needed(&self) -> u32 {
        self.smallest_perimiter() + self.volume()
    }
}3

#[cfg(test)]
mod tests {
    use super::Present;

    #[test]
    fn test_case_1() {
        let present = Present {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(present.paper_needed(), 58)
    }

    #[test]
    fn test_case_2() {
        let present = Present {
            length: 1,
            width: 1,
            height: 10,
        };
        assert_eq!(present.paper_needed(), 43)
    }

    #[test]
    fn test_case_3() {
        let present = Present {
            length: 2,
            width: 3,
            height: 4,
        };

        assert_eq!(present.ribbon_needed(), 34)
    }

    #[test]
    fn test_case_4() {
        let present = Present {
            length: 1,
            width: 1,
            height: 10,
        };

        assert_eq!(present.ribbon_needed(), 14)
    }
}
