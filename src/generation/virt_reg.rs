use std::{collections::HashMap, hash::Hash, ops::Range};

use crate::ir::Instruction;

pub trait Register
where
    Self: Sized + Copy,
{
    fn caller_saved() -> Vec<Self>;
    fn callee_saved() -> Vec<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtRegKind {
    StackSpace,
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtRegInfo<R>
where
    R: Register,
{
    pub kind: VirtRegKind,
    pub lifetime: Range<usize>,
    pub real_reg: Option<R>,
}
impl<R> Default for VirtRegInfo<R>
where
    R: Register,
{
    fn default() -> Self {
        Self {
            kind: VirtRegKind::Normal,
            lifetime: Default::default(),
            real_reg: None,
        }
    }
}
impl<R> VirtRegInfo<R>
where
    R: Register,
{
    fn new(kind: VirtRegKind) -> Self {
        Self {
            kind,
            lifetime: 0..0,
            real_reg: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// State of a virtual register's life at one since step
enum VirtRegLifeState {
    Dead = 0b000,
    Live = 0b100,
    Born = 0b101,
    Last = 0b110,
}
impl Default for VirtRegLifeState {
    fn default() -> Self {
        Self::Dead
    }
}
impl VirtRegLifeState {
    fn is_living(self) -> bool {
        (self as usize) & 0b100 != 0
    }
}

#[derive(Debug, Clone)]
pub struct VirtRegMap<R>
where
    R: Register,
{
    lifetime_map: Vec<HashMap<u64, VirtRegLifeState>>, // step: living registers
    reg_infos: HashMap<u64, VirtRegInfo<R>>,
}
impl<R> VirtRegMap<R>
where
    R: Register + Hash + Eq,
{
    pub fn empty(step_count: usize) -> Self {
        Self {
            lifetime_map: (0..step_count).map(|_| HashMap::new()).collect(),
            reg_infos: HashMap::<u64, VirtRegInfo<R>>::new(),
        }
    }
    /// Allocate real registers or stack space for the all virtual registers
    pub fn alloc_real_registers(&mut self) {
    }
    /// Extend the lifetime of a virtual register to `step`
    /// Start of the lifetime does not change
    /// If `step` < `start` the function would mark the register alive to the end of the body
    pub fn mark_alive_until(&mut self, id: u64, end: usize) {
        let lifetime = &mut self.reg_infos.get_mut(&id).unwrap().lifetime;
        self.lifetime_map[lifetime.end + if lifetime.start == lifetime.end { 1 } else { 0 }..end]
            .iter_mut()
            .for_each(|row| {
                row.insert(id, VirtRegLifeState::Live);
            });
        self.lifetime_map[end].insert(id, VirtRegLifeState::Last);
        lifetime.end = end;
    }
    /// Mark a virtual register alive at `step`
    pub fn mark_alive(&mut self, id: u64, step: usize) {
        self.reg_infos.get_mut(&id).unwrap().lifetime = step..step;
        self.lifetime_map[step].insert(id, VirtRegLifeState::Born);
    }
    pub fn generate_from_body(body: &Vec<Instruction>) -> Self {
        let mut map = VirtRegMap::empty(body.len());
        macro_rules! update_reg_lifetime_if_needed {
            ($i: expr, $step: expr) => {
                match $i {
                    Instruction::Reg(_, id) | Instruction::Load { id, dtype: _ } => {
                        map.mark_alive_until(*id, $step);
                    }
                    _ => (),
                }
            };
        }
        body.iter()
            .enumerate()
            .for_each(|(step, instr)| match instr {
                Instruction::DefReg { id, rhs } => {
                    map.reg_infos.insert(
                        *id,
                        VirtRegInfo::new(match rhs.as_ref() {
                            Instruction::Alloc(_) => VirtRegKind::StackSpace,
                            _ => VirtRegKind::Normal,
                        }),
                    );
                    map.mark_alive(*id, step);
                }
                Instruction::Store { id, rhs } => {
                    map.mark_alive_until(*id, step);
                    update_reg_lifetime_if_needed!(rhs.as_ref(), step);
                }
                Instruction::Ret(Some(ret_val)) => {
                    update_reg_lifetime_if_needed!(ret_val.as_ref(), step);
                }
                Instruction::Ret(None) => (),
                Instruction::Call {
                    ret_type: _,
                    fn_name: _,
                    args,
                } => {
                    for arg in args {
                        update_reg_lifetime_if_needed!(arg, step);
                    }
                }
                Instruction::Label(_) => (),
                instr => panic!("{:?} in root level is invalid", instr),
            });
        map
    }

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn print_reg_lifetime_map(&self) {
        print!("Virtual Register Lifetime Map:\n\t");
        self.reg_infos
            .iter()
            .for_each(|(id, _)| print!("%{}\t", id));
        self.lifetime_map
            .iter()
            .map(|regs| {
                self.reg_infos
                    .iter()
                    .map(|(id, _)| regs.get(id).unwrap_or(&VirtRegLifeState::Dead))
            })
            .enumerate()
            .map(|(i, stage)| {
                print!("\n{}\t", i);
                stage
            })
            .flatten()
            .for_each(|state| match state {
                VirtRegLifeState::Dead => print!("\t"),
                VirtRegLifeState::Live => print!("|\t"),
                VirtRegLifeState::Born => print!("+\t"),
                VirtRegLifeState::Last => print!("-\t"),
            });
        println!()
    }
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn print_reg_infos(&self) {
        println!("Virtual registers:");
        self.reg_infos.iter().for_each(|(id, info)| {
            println!(
                "{}:\t{}\t{:?}",
                id,
                match info.kind {
                    VirtRegKind::StackSpace => "stack",
                    VirtRegKind::Normal => "normal",
                },
                info.lifetime
            );
        })
    }
}
