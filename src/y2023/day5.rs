use std::ops::Range;

use anyhow::{Context, Result};

pub fn solve(input: &[&str]) -> String {
    let almanac = Almanac::parse(input).unwrap();

    let part1 = almanac.find_lowest_location();
    let part2 = almanac.find_lowest_location_seed_ranges();

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[derive(Debug)]
struct Mapping {
    destination_range: Range<isize>,
    source_range: Range<isize>,
    range_offset: isize,
}

impl Mapping {
    pub fn parse(mapping_data: &str) -> anyhow::Result<Self> {
        let mapping_data = mapping_data
            .splitn(3, ' ')
            .map(|d| {
                d.parse().context(format!(
                    "Could not parse `{d}` as digit from `{mapping_data}`"
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let destination_range_start = mapping_data[0];
        let source_range_start = mapping_data[1];
        let range_length = mapping_data[2];

        let range_offset = {
            if source_range_start < destination_range_start {
                destination_range_start - source_range_start
            } else {
                destination_range_start - source_range_start
            }
        };

        Ok(Self {
            destination_range: destination_range_start..destination_range_start + range_length,
            source_range: source_range_start..source_range_start + range_length,
            range_offset,
        })
    }

    pub fn get(&self, value: isize) -> Option<isize> {
        if self.source_range.contains(&value) {
            Some(value + self.range_offset)
        } else {
            None
        }
    }

    pub fn get_reverse(&self, value: isize) -> Option<isize> {
        if self.destination_range.contains(&value) {
            Some(value - self.range_offset)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn destination_range(&self) -> Range<isize> {
        self.destination_range.clone()
    }
}

struct AlmanacMap {
    mappings: Vec<Mapping>,
}

impl AlmanacMap {
    pub fn parse(map_data: &[&str]) -> anyhow::Result<Self> {
        let mappings = map_data
            .iter()
            .map(|line| Mapping::parse(line))
            .collect::<Result<Vec<_>, _>>()
            .context(format!("Error parsing `{map_data:#?}` as AmanacMap"))?;

        Ok(Self { mappings })
    }

    pub fn get(&self, value: isize) -> isize {
        for mapping in self.mappings.iter() {
            if let Some(mapped_value) = mapping.get(value) {
                return mapped_value;
            }
        }

        value
    }

    pub fn get_reverse(&self, value: isize) -> isize {
        for mapping in self.mappings.iter() {
            if let Some(mapped_value) = mapping.get_reverse(value) {
                return mapped_value;
            }
        }

        value
    }

    #[allow(dead_code)]
    pub fn destination_mappings(&self) -> Vec<Range<isize>> {
        self.mappings
            .iter()
            .map(|mapping| mapping.destination_range())
            .collect()
    }
}

struct Almanac {
    seeds: Vec<isize>,
    seed_ranges: Vec<Range<isize>>,
    seed_to_soil_map: AlmanacMap,
    soil_to_fertilizer_map: AlmanacMap,
    fertilizer_to_water_map: AlmanacMap,
    water_to_light_map: AlmanacMap,
    light_to_temperature_map: AlmanacMap,
    temperature_to_humidity_map: AlmanacMap,
    humidity_to_location_map: AlmanacMap,
}

impl Almanac {
    pub fn parse(almanac_data: &[&str]) -> anyhow::Result<Self> {
        let seeds = almanac_data[0]
            .split(':')
            .nth(1)
            .expect(&format!("Split error on `{}`", almanac_data[0]))
            .trim_start()
            .split_whitespace()
            .map(|n| {
                n.parse::<isize>()
                    .context(format!("Cound not parse `{n}` as digit"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut seed_ranges = seeds[..]
            .chunks(2)
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .collect::<Vec<_>>();
        seed_ranges.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

        let (start, end) = Self::parse_section_start_end("seed-to-soil", almanac_data);
        let seed_to_soil_map = AlmanacMap::parse(&almanac_data[start..end])?;

        let (start, end) = Self::parse_section_start_end("soil-to-fertilizer", almanac_data);
        let soil_to_fertilizer_map = AlmanacMap::parse(&almanac_data[start..end])?;

        let (start, end) = Self::parse_section_start_end("fertilizer-to-water", almanac_data);
        let fertilizer_to_water_map = AlmanacMap::parse(&almanac_data[start..end])?;

        let (start, end) = Self::parse_section_start_end("water-to-light", almanac_data);
        let water_to_light_map = AlmanacMap::parse(&almanac_data[start..end])?;

        let (start, end) = Self::parse_section_start_end("light-to-temperature", almanac_data);
        let light_to_temperature_map = AlmanacMap::parse(&almanac_data[start..end])?;

        let (start, end) = Self::parse_section_start_end("temperature-to-humidity", almanac_data);
        let temperature_to_humidity_map = AlmanacMap::parse(&almanac_data[start..end])?;

        let (start, end) = Self::parse_section_start_end("humidity-to-location map", almanac_data);
        let humidity_to_location_map = AlmanacMap::parse(&almanac_data[start..end])?;

        Ok(Self {
            seeds,
            seed_ranges,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        })
    }

    pub fn find_lowest_location(&self) -> isize {
        self.seeds
            .iter()
            .map(|s| self.find_location_for_seed(*s))
            .min()
            .unwrap()
    }

    pub fn find_lowest_location_seed_ranges(&self) -> isize {
        dbg!(&self.seed_ranges);

        let locations = {
            let mut l = self
                .seed_ranges
                .iter()
                .map(|range| self.find_location_for_seed(range.start))
                .collect::<Vec<_>>();
            l.sort();
            l
        };

        locations[0]

        // let mut location_ranges = {
        //     let mut destination_mappings = self.humidity_to_location_map.destination_mappings();
        //     destination_mappings.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
        //
        //     let mut location_ranges = vec![];
        //     let chunks = destination_mappings[..].chunks_exact(2);
        //     for chunk in chunks {
        //         let a = &chunk[0];
        //         let b = &chunk[1];
        //
        //         let r = a.start + a.len() as isize..b.start;
        //
        //         location_ranges.extend([a.clone(), r, b.clone()]);
        //     }
        //
        //     if let Some(range) = chunks.remainder().get(0) {
        //         let r = range.start + range.len() as isize..isize::MAX;
        //
        //         location_ranges.extend([range.clone(), r]);
        //     }
        //
        //     location_ranges
        // };
        //
        // loop {
        //     for range in location_ranges.iter_mut() {
        //         if let Some(location) = range.next() {
        //             // dbg!(location);
        //             if let Some(_seed) = self.find_seed_for_location(location) {
        //                 return location;
        //             }
        //         }
        //
        //     }
        //
        //     if location_ranges.iter().fold(true, |acc, range| acc && !range.is_empty()) == false {
        //         break;
        //     }
        // }

        // 0
    }

    fn find_location_for_seed(&self, seed: isize) -> isize {
        self.humidity_to_location_map.get(
            self.temperature_to_humidity_map.get(
                self.light_to_temperature_map.get(
                    self.water_to_light_map.get(
                        self.fertilizer_to_water_map.get(
                            self.soil_to_fertilizer_map
                                .get(self.seed_to_soil_map.get(seed)),
                        ),
                    ),
                ),
            ),
        )
    }

    #[allow(dead_code)]
    fn find_seed_for_location(&self, location: isize) -> Option<isize> {
        let seed = self.seed_to_soil_map.get_reverse(
            self.soil_to_fertilizer_map.get_reverse(
                self.water_to_light_map.get_reverse(
                    self.light_to_temperature_map.get_reverse(
                        self.temperature_to_humidity_map
                            .get_reverse(self.humidity_to_location_map.get_reverse(location)),
                    ),
                ),
            ),
        );

        for range in self.seed_ranges.iter() {
            if range.contains(&seed) {
                return Some(seed);
            }
        }

        None
    }

    fn parse_section_start_end(section_heading: &str, almanac_data: &[&str]) -> (usize, usize) {
        let mut start = None;
        let mut end = almanac_data.len();

        for (i, line) in almanac_data.iter().enumerate() {
            if line.starts_with(section_heading) {
                start = Some(i + 1);
            }

            if start.is_some() && line.is_empty() {
                end = i;
                break;
            }
        }

        (start.unwrap(), end)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mapping() {
        let input = "50 98 2";

        let mapping = Mapping::parse(&input).unwrap();

        let value = 98;
        let expected = Some(50);
        let acutual = mapping.get(value);

        assert_eq!(acutual, expected);

        let value = 99;
        let expected = Some(51);
        let acutual = mapping.get(value);

        assert_eq!(acutual, expected);

        let value = 97;
        let expected = None;
        let acutual = mapping.get(value);

        assert_eq!(acutual, expected);

        let value = 100;
        let expected = None;
        let acutual = mapping.get(value);

        assert_eq!(acutual, expected);
    }

    #[test]
    fn test_mapping_large_range() {
        let input = "52 50 48";

        let mapping = Mapping::parse(&input).unwrap();

        let value = 49;
        let expected = None;
        let acutual = mapping.get(value);

        assert_eq!(acutual, expected);

        let value = 100;
        let expected = None;
        let acutual = mapping.get(value);

        let offset = 2;
        for value in 50..=97 {
            let expected = Some(value + offset);
            let actual = mapping.get(value);

            assert_eq!(
                actual, expected,
                "Got {actual:?} when expecting {expected:?} from calling get with {value:?}"
            );
        }

        assert_eq!(acutual, expected);
    }

    #[test]
    fn test_almanac_map() {
        let mapping_data = ["50 98 2", "52 50 48"];

        let map = AlmanacMap::parse(&mapping_data).unwrap();

        let value = 10;
        let expected = value;
        let actual = map.get(value);

        assert_eq!(actual, expected);

        let value = 98;
        let expected = 50;
        let actual = map.get(value);

        assert_eq!(actual, expected);

        let value = 50;
        let expected = 52;
        let actual = map.get(value);

        assert_eq!(actual, expected);

        let value = 97;
        let expected = 99;
        let actual = map.get(value);

        assert_eq!(actual, expected);

        let value = 99;
        let expected = 51;
        let actual = map.get(value);

        assert_eq!(actual, expected);

        let value = 100;
        let expected = 100;
        let actual = map.get(value);

        assert_eq!(actual, expected);

        let value = 79;
        let expected = 81;
        let actual = map.get(value);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_lowest_location() {
        let almanac_data = [
            "seeds: 79 14 55 13",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];

        let almanac = Almanac::parse(&almanac_data).unwrap();

        let expected = 35;
        let actual = almanac.find_lowest_location();

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore = "trying to find a faster way to do this"]
    fn test_find_lowest_location_seed_ranges() {
        let almanac_data = [
            "seeds: 79 14 55 13",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];

        let almanac = Almanac::parse(&almanac_data).unwrap();

        let expected = 46;
        let actual = almanac.find_lowest_location_seed_ranges();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mapping_get_reverse() {
        let input = "50 98 2";

        let mapping = Mapping::parse(&input).unwrap();

        let value = 50;
        let expected = Some(98);
        let acutual = mapping.get_reverse(value);

        assert_eq!(acutual, expected);

        let value = 51;
        let expected = Some(99);
        let acutual = mapping.get_reverse(value);

        assert_eq!(acutual, expected);

        let value = 49;
        let expected = None;
        let acutual = mapping.get_reverse(value);

        assert_eq!(acutual, expected);

        let value = 52;
        let expected = None;
        let acutual = mapping.get_reverse(value);

        assert_eq!(acutual, expected);
    }
}
