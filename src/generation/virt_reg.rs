use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegKind {
    StackSpace,
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegInfo {
    pub kind: RegKind,
    pub lifetime: Range<usize>,
}
impl Default for RegInfo {
    fn default() -> Self {
        Self {
            kind: RegKind::Normal,
            lifetime: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtRegMap {
    /// Every row is an instruction (step), every column is a register and whether
    /// or not it's alive at the time, use `VirtRegMap.print_reg_lifetime_map(...)` to see a
    /// visualization of the map
    pub lifetime_map: Vec<Vec<bool>>,
    pub reg_infos: Vec<RegInfo>,
}
impl VirtRegMap {
    pub fn new(reg_count: usize, step_count: usize) -> Self {
        let row_sample = (0..reg_count).map(|_| false).collect::<Vec<bool>>();
        let map = (0..step_count)
            .map(|_| row_sample.clone())
            .collect::<Vec<Vec<bool>>>();
        let infos = (0..reg_count).map(|_| RegInfo::default()).collect();
        Self {
            lifetime_map: map,
            reg_infos: infos,
        }
    }
    pub fn mark_alive_until(&mut self, id: u64, step: usize) {
        self.reg_infos[id as usize].lifetime.end = step;
        self.lifetime_map
            .iter_mut()
            .enumerate()
            .skip_while(|(_, row)| row[id as usize] == false)
            .take_while(|(i, _)| *i <= step)
            .for_each(|(_, row)| row[id as usize] = true);
    }
    pub fn mark_alive(&mut self, id: u64, step: usize) {
        self.reg_infos[id as usize].lifetime.start = step;
        self.reg_infos[id as usize].lifetime.end = step;
        self.lifetime_map[step][id as usize] = true;
    }

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn print_reg_lifetime_map(&self) {
        println!("Virtual Register Lifetime Map:");
        self.lifetime_map.iter().enumerate().for_each(|(i, row)| {
            print!("{}\t", i);
            row.iter()
                .for_each(|b| print!("{} ", if *b { '√' } else { 'x' }));
            println!();
        });
    }
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn print_reg_infos(&self) {
        println!("Virtual registers:");
        self.reg_infos.iter().enumerate().for_each(|(i, info)| {
            println!(
                "{}:\t{}\t{:?}",
                i,
                match info.kind {
                    RegKind::StackSpace => "stack",
                    RegKind::Normal => "normal",
                },
                info.lifetime
            );
        })
    }
}
