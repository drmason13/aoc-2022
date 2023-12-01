use std::fmt;

use shared::types_2d::{modulus, Bounds, InfGrid, Vector};

#[derive(Clone, PartialEq, Eq)]
pub struct Sensor {
    pub position: Vector,
    pub closest_beacon: Vector,
    pub range: usize,
}

impl Sensor {
    pub fn new(position: Vector, closest_beacon: Vector) -> Self {
        let range = position.manhattan_distance(closest_beacon);

        Sensor {
            position,
            closest_beacon,
            range,
        }
    }

    pub fn distance_to_beacon(&self) -> usize {
        self.range
    }

    pub fn distance_to_point(&self, x: isize, y: isize) -> usize {
        self.position.manhattan_distance(Vector::new(x, y))
    }

    pub fn coverage_at_y(&self, y: isize, min: isize, max: isize) -> Option<Coverage> {
        let distance_to_y = modulus(self.position.y - y) as isize;
        let spare = self.range as isize - distance_to_y;
        if spare < 0 {
            None
        } else {
            let left = (self.position.x - spare).clamp(min, max);
            let right = (self.position.x + spare).clamp(min, max);
            Some(Coverage { left, right })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Coverage {
    left: isize,
    right: isize,
}

impl Coverage {
    pub fn new(left: isize, right: isize) -> Self {
        Coverage { left, right }
    }
    fn merge(&self, other: &Coverage) -> Option<Coverage> {
        let al = self.left;
        let ar = self.right;

        let bl = other.left;
        let br = other.right;

        // no overlap
        if (ar + 1 < bl && al + 1 < bl) || (al - 1 > br && ar - 1 > br) {
            return None;
        }
        // b subsumes a
        if ar <= br && al >= bl {
            return Some(other.clone());
        }
        // a subsumes b
        if br <= ar && bl >= al {
            return Some(self.clone());
        }
        // b left of a
        if bl <= al && br + 1 >= al {
            return Some(Coverage {
                left: bl,
                right: ar,
            });
        }
        // a left of b
        if al <= bl && ar + 1 >= bl {
            return Some(Coverage {
                left: al,
                right: br,
            });
        }
        unreachable!("Coverage::merge accounts for all cases")
    }
}

impl Ord for Coverage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.left.cmp(&other.left) {
            std::cmp::Ordering::Equal => self.right.cmp(&other.right),
            ord => ord,
        }
    }
}

impl PartialOrd for Coverage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TotalCoverage {
    partial_coverages: Vec<Coverage>,
    pub coverage: Option<Coverage>,
}

impl TotalCoverage {
    pub fn new() -> Self {
        TotalCoverage {
            partial_coverages: Vec::new(),
            coverage: None,
        }
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        for coverage in self.partial_coverages.iter() {
            count += modulus(coverage.right - coverage.left);
        }
        if let Some(ref coverage) = self.coverage {
            count += modulus(coverage.right - coverage.left);
        }
        count
    }

    pub fn gap(&self, min: isize, max: isize) -> Option<usize> {
        if let Some(ref coverage) = self.coverage {
            if let Some(partial) = self.partial_coverages.last() {
                if coverage.left == min && partial.right == max {
                    if coverage.right == partial.left - 2 {
                        Some(modulus(coverage.right + 1))
                    } else {
                        None
                    }
                } else if partial.left == min && coverage.right == max {
                    if partial.right == coverage.left - 2 {
                        Some(modulus(partial.right + 1))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn add_coverage(&mut self, coverage: Coverage) {
        self.partial_coverages.push(coverage);
        let mut before = self.partial_coverages.len();
        let mut after = self.partial_coverages.len() + 1;
        while before != after {
            before = after;
            self.consolidate();
            after = self.partial_coverages.len();
        }
    }

    fn consolidate(&mut self) {
        self.partial_coverages.sort();
        let (consolidated, remaining_partials) = self.fold_partials();
        self.partial_coverages = remaining_partials;

        if let Some(coverage) = consolidated {
            // merge consolidated into self.coverage
            if self.coverage.is_none() {
                self.coverage = Some(coverage);
                return;
            }
            if let Some(merged) = self.coverage.as_ref().unwrap().merge(&coverage) {
                self.coverage = Some(merged);
                return;
            }
            // or push it to partial_coverages
            self.partial_coverages.push(coverage);
        }
    }

    fn fold_partials(&mut self) -> (Option<Coverage>, Vec<Coverage>) {
        self.partial_coverages.drain(..).fold(
            (None, Vec::new()),
            |(mut consolidated, mut remaining_partials), partial| {
                // merge partial into self.coverage
                if self.coverage.is_none() {
                    self.coverage = Some(partial);
                    return (consolidated, remaining_partials);
                } else if let Some(merged) = self.coverage.as_ref().unwrap().merge(&partial) {
                    self.coverage = Some(merged);
                    return (consolidated, remaining_partials);
                }

                // merge partial into consolidated
                if consolidated.is_none() {
                    consolidated = Some(partial);
                    return (consolidated, remaining_partials);
                }
                if let Some(merged) = consolidated.as_ref().unwrap().merge(&partial) {
                    consolidated = Some(merged);
                    return (consolidated, remaining_partials);
                }

                // add partial to remaining partials
                remaining_partials.push(partial);
                (consolidated, remaining_partials)
            },
        )
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Entity {
    Sensor(Sensor),
    Beacon,
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sensor(_) => write!(f, "S"),
            Self::Beacon => write!(f, "B"),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub grid: InfGrid<Entity>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            grid: InfGrid::new(),
        }
    }

    pub fn add(&mut self, position: Vector, entity: Entity) {
        self.grid.add(position, entity, false);
    }

    #[allow(unused)]
    pub fn get(&self, x: isize, y: isize) -> Option<&Entity> {
        self.grid
            .get(Vector::new(x, y))
            .and_then(|cell| cell.value.as_ref())
    }

    #[allow(unused)]
    pub fn bounds(&self) -> Bounds {
        self.grid.bounds()
    }

    pub fn sensors(&self) -> Sensors {
        Sensors {
            iter: Box::new(self.grid.cells().filter_map(
                |cell| match cell.value.as_ref().cloned() {
                    Some(Entity::Sensor(s)) => Some(s),
                    _ => None,
                },
            )),
        }
    }
}

pub struct Sensors<'a> {
    iter: Box<dyn Iterator<Item = Sensor> + 'a>,
}

impl<'a> Iterator for Sensors<'a> {
    type Item = Sensor;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod coverage_tests {
    use super::Coverage;
    use super::TotalCoverage;

    #[test]
    fn test_add_coverage() {
        let coverages = vec![
            (Coverage::new(0, 1), Coverage::new(0, 1)),
            (Coverage::new(4, 5), Coverage::new(0, 1)),
            (Coverage::new(8, 9), Coverage::new(0, 1)),
            (Coverage::new(2, 3), Coverage::new(0, 5)),
            (Coverage::new(1, 4), Coverage::new(0, 5)),
            (Coverage::new(5, 7), Coverage::new(0, 9)),
        ];
        let mut total = TotalCoverage::new();
        for c in coverages {
            total.add_coverage(c.0);
            assert_eq!(total.coverage.clone(), Some(c.1))
        }

        assert_eq!(total.coverage, Some(Coverage::new(0, 9)));
    }

    #[test]
    fn test_consolidate() {
        let mut total_coverage = TotalCoverage {
            partial_coverages: vec![
                Coverage {
                    left: 15,
                    right: 17,
                },
                Coverage { left: 0, right: 13 },
            ],
            coverage: Some(Coverage {
                left: 15,
                right: 20,
            }),
        };
        total_coverage.consolidate();
        assert_eq!(
            total_coverage,
            TotalCoverage {
                partial_coverages: vec![Coverage { left: 0, right: 13 }],
                coverage: Some(Coverage {
                    left: 15,
                    right: 20
                })
            }
        );

        let mut total_coverage = TotalCoverage {
            partial_coverages: vec![
                Coverage {
                    left: 15,
                    right: 17,
                },
                Coverage {
                    left: 15,
                    right: 20,
                },
            ],
            coverage: Some(Coverage { left: 0, right: 13 }),
        };
        total_coverage.consolidate();
        assert_eq!(
            total_coverage,
            TotalCoverage {
                partial_coverages: vec![Coverage {
                    left: 15,
                    right: 20
                }],
                coverage: Some(Coverage { left: 0, right: 13 })
            }
        );
    }

    #[test]
    fn merge_no_overlap() {
        let a = Coverage::new(0, 1);
        let b = Coverage::new(8, 9);
        assert!(a.merge(&b).is_none());
        assert!(b.merge(&a).is_none());
    }

    #[test]
    fn merge_subsume() {
        let a = Coverage::new(0, 9);
        let b = Coverage::new(5, 6);
        assert_eq!(a.merge(&b), Some(a.clone()));
        assert_eq!(b.merge(&a), Some(a.clone()));
    }

    #[test]
    fn merge() {
        let a = Coverage::new(0, 5);
        let b = Coverage::new(5, 9);
        let expected = Some(Coverage::new(0, 9));
        assert_eq!(a.merge(&b), expected);
        assert_eq!(b.merge(&a), expected);
    }

    #[test]
    fn merge_adjacent() {
        let a = Coverage::new(0, 4);
        let b = Coverage::new(5, 9);
        let expected = Some(Coverage::new(0, 9));
        assert_eq!(a.merge(&b), expected);
        assert_eq!(b.merge(&a), expected);
    }
}
